use gtk::prelude::*;
use gtk::FileChooserDialog;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Button, FileChooserAction, Image,
    Orientation, ResponseType,
};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const APP_ID: &str = "org.gtk_rs.HelloWorld";

pub fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window first
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Linux WallpaperEngine GUI")
        .build();

    // Create a button
    let button = Button::builder()
        .label("Folder")
        .margin_top(2)
        .margin_bottom(2)
        .margin_start(2)
        .margin_end(2)
        .build();

    let container = GtkBox::new(Orientation::Horizontal, 5);

    // let image_path = "/home/user/Pictures/Wallpapers/*.png";
    // let image = Image::from_file(image_path);
    // fs::create_dir("/home/user/.cache/LinuxWEGUI");
    // fs::copy(image_path, "/home/user/.cache/LinuxWEGUI/a32xicnes4k91.png");

    // let image_dir = "/media/gamedisk4/SteamLibrary/steamapps/workshop/content/431960/";
    let image_dir = "/home/user/Desktop/LinuxWallpaperEngineGUI";

    for entry in WalkDir::new(image_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        println!("processing file: {}", path.to_string_lossy());

        let cache_file = Path::new("/home/user/.cache/LinuxWEGUI/").join(path.file_name().unwrap());
        if cache_file.exists() {
            if path.file_name().map_or(false, |name| name == "preview.png") {
                let image = Image::from_file(
                    "/home/user/Desktop/LinuxWallpaperEngineGUI/431960/864310972/preview.png",
                );
                image.set_pixel_size(150);
                container.append(&image);

                let dest_path =
                    Path::new("/home/user/.cache/LinuxWEGUI/").join(path.file_name().unwrap());

                if let Some(parent_dir) = dest_path.parent() {
                    fs::create_dir_all(parent_dir).expect("Failed to create parent directory");
                } else {
                    fs::copy(path, dest_path).expect("yeet");
                }
            }
        }
    }

    // fs::copy(path, dest_path).expect("yeet");

    // fs::copy(path, dest_path).expect("yeet");

    // fs::copy(path, "/home/user.cache/LinuxWEGUI/").expect("yeet");

    let window_weak = window.downgrade();
    button.connect_clicked(move |_| {
        // Get a strong reference back from the weak window
        if let Some(window) = window_weak.upgrade() {
            let dialog = FileChooserDialog::new(
                Some("Select Workshop Folder"),
                Some(&window),
                FileChooserAction::Open,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Open", ResponseType::Accept),
                ],
            );

            // Use a response callback rather than dialog.run()
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                        println!("Selected file: {}", file_path.display());
                    }
                }
                dialog.close();
            });

            // Present the dialog (non-blocking)
            dialog.present();
        }
    });

    // Attach the button to the window and show everything
    window.set_child(Some(&button));
    // window.set_child(Some(&image));
    window.set_child(Some(&container));
    window.present();
}
