use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box as GtkBox, Button, FileChooserAction, FileChooserDialog,
    Image, Orientation, PolicyType, ResponseType, ScrolledWindow,
};
use walkdir::WalkDir;

const APP_ID: &str = "org.gtk_rs.Example";

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Images in Rows of 5")
        .default_width(800)
        .default_height(600)
        .build();

    let wrapper_box = GtkBox::new(Orientation::Vertical, 5);
    wrapper_box.set_hexpand(true);
    wrapper_box.set_vexpand(true);

    let button = Button::with_label("Folder");
    wrapper_box.append(&button);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vscrollbar_policy(PolicyType::Automatic)
        .build();
    scrolled_window.set_hexpand(true);
    scrolled_window.set_vexpand(true);

    let all_rows_box = GtkBox::new(Orientation::Vertical, 5);

    let image_dir = "/home/user/Desktop/LinuxWallpaperEngineGUI";
    let mut row_box = GtkBox::new(Orientation::Horizontal, 5);
    let mut images_in_row = 0;

    for entry in WalkDir::new(image_dir)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.file_name().map_or(false, |name| name == "preview.jpg") {
            let image = Image::from_file(path);
            image.set_pixel_size(150);

            row_box.append(&image);
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

    wrapper_box.append(&scrolled_window);

    let window_weak = window.downgrade();
    button.connect_clicked(move |_| {
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
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Accept {
                    if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                        println!("Selected file: {}", file_path.display());
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

pub fn main() -> gtk::glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}
