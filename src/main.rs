//use dirs::config_dir;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, FileChooserAction, FileChooserDialog,
    Image, Orientation, PolicyType, ResponseType, ScrolledWindow,
};
use std::env;
use std::fs::{self};
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

const APP_ID: &str = "LinuxWEGUI";

struct AppState {
    current_process: Option<Child>,
}

fn build_ui(app: &Application) {
    let app_state = Arc::new(Mutex::new(AppState {
        current_process: None,
    }));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .build();
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);

    if let Some(image_dir) = load_config_dir() {
        load_images(&image_dir, &scrolled_window, Arc::clone(&app_state));
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Linux Wallpaper Engine GUI")
        .default_width(800)
        .default_height(600)
        .build();

    let app_state_clone = app_state.clone();
    window.connect_close_request(move |_| {
        let mut state = app_state_clone.lock().unwrap();
        if let Some(mut child) = state.current_process.take() {
            println!("Terminating wallpaper engine process");
            let _ = child.kill();
        }
        false.into()
    });

    let wrapper_box = GtkBox::new(Orientation::Vertical, 5);
    wrapper_box.set_hexpand(true);
    wrapper_box.set_vexpand(true);

    // FILE CHOOSER BUTTON
    let button = Button::with_label("Select Workshop Folder");
    wrapper_box.append(&button);

    // SCROLLED WINDOW FOR IMAGES
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .build();
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);
    wrapper_box.append(&scrolled_window);

    // Connect click event to button
    let window_weak = window.downgrade();
    let scrolled_window_clone = scrolled_window.clone();
    let app_state_clone = app_state.clone();

    button.connect_clicked(move |_| {
        if let Some(window) = window_weak.upgrade() {
            let dialog = FileChooserDialog::new(
                Some("Select Workshop Folder"),
                Some(&window),
                FileChooserAction::SelectFolder,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Open", ResponseType::Accept),
                ],
            );

            let scrolled_window = scrolled_window_clone.clone();
            let app_state = app_state_clone.clone();

            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                        println!("Selected folder: {}", file_path.display());
                        load_images(&file_path, &scrolled_window, app_state.clone());
                    }
                }
                dialog.close();
            });
            dialog.present();
        }
    });

    window.set_child(Some(&wrapper_box));
    window.present();
}

fn load_config_dir() -> Option<PathBuf> {
    let config_home = env::var("XDG_CONFIG_HOME")
        .unwrap_or_else(|_| format!("{}/.config", env::var("HOME").unwrap()));

    let config_path = PathBuf::from(format!("{}/linux-wallpaperengine/config.toml", config_home));

    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(&config_path) {
            for line in content.lines() {
                if line.starts_with("image_dir") {
                    if let Some(dir_str) = line.split('\"').nth(1) {
                        let path = PathBuf::from(dir_str);
                        if path.exists() && path.is_dir() {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }
    None
}

fn load_images(
    image_dir: &PathBuf,
    scrolled_window: &ScrolledWindow,
    app_state: Arc<Mutex<AppState>>,
) {
    // DISPLAY IMAGES
    let all_rows_box = GtkBox::new(Orientation::Vertical, 5);
    let mut row_box = GtkBox::new(Orientation::Horizontal, 5);
    let mut images_in_row = 0;

    let image_dir = load_config_dir().unwrap_or_else(|| image_dir.clone());

    // read config file to get image_dir if it exists in config
    for entry in WalkDir::new(&image_dir)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path().to_path_buf();
        if path.file_name().is_some_and(|name| name == "preview.jpg") {
            let image = Image::from_file(&path);
            image.set_pixel_size(150);

            let button = Button::builder().child(&image).build();
            let path_clone = path.clone();
            let app_state_clone = app_state.clone();

            button.connect_clicked(move |_| {
                let command_path = path_clone.to_string_lossy().replace("/preview.jpg", "");
                let mut state = app_state_clone.lock().unwrap();
                if let Some(mut child) = state.current_process.take() {
                    println!("Terminating previous wallpaper process");
                    let _ = child.kill();
                }
                let child = Command::new("linux-wallpaperengine")
                    .arg("--use-angle=GL")
                    .arg("--screen-root=DP-2")
                    .arg("--screen-root=DP-1")
                    .arg("--screen-root=HDMI-A-1")
                    .arg("--silent")
                    .arg(&command_path)
                    .spawn();
                //let status = child.wait().expect("Failed to wait on child process");
                match child {
                    Ok(child_process) => {
                        println!("Started wallpaper engine with: {}", command_path);
                        state.current_process = Some(child_process);
                    }
                    Err(e) => println!("Failed to start wallpaper engine: {}", e),
                }
            });
            row_box.append(&button);
            images_in_row += 1;
            if images_in_row == 10 {
                all_rows_box.append(&row_box);
                row_box = GtkBox::new(Orientation::Horizontal, 5);
                images_in_row = 0;
            }
        }
    }
    if images_in_row > 0 {
        all_rows_box.append(&row_box);
    }
    scrolled_window.set_child(Some(&all_rows_box));
}
pub fn main() -> gtk::glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
