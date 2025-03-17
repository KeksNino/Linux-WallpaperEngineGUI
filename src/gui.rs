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

    let main_box = GtkBox::new(Orientation::Horizontal, 5);

    let image_dir = "/media/gamedisk4/SteamLibrary/steamapps/workshop/content/431960/";
    // let image_dir = "/home/user/Desktop/LinuxWallpaperEngineGUI";

    let mut column_box = GtkBox::new(Orientation::Vertical, 5);
    let mut images_in_column = 0;

    for entry in WalkDir::new(image_dir)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        if path.file_name().map_or(false, |name| name == "preview.jpg") {
            let image = Image::from_file(path);
            image.set_pixel_size(150);

            column_box.append(&image);
            images_in_column += 1;

            if images_in_column == 5 {
                main_box.append(&column_box);
                column_box = GtkBox::new(Orientation::Vertical, 5);
                images_in_column = 0;
            }
        }
    }
    if images_in_column > 0 {
        main_box.append(&column_box);
    }

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

    let wrapper_box = GtkBox::new(Orientation::Vertical, 5);
    wrapper_box.append(&button);
    wrapper_box.append(&main_box);

    window.set_child(Some(&wrapper_box));
    window.present();
}
