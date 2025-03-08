use gtk::prelude::*;
use gtk::FileChooserDialog;
use gtk::{glib, Application, ApplicationWindow, Button, FileChooserAction, ResponseType};

const APP_ID: &str = "org.gtk_rs.HelloWorld";

pub fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Folder")
        .margin_top(2)
        .margin_bottom(2)
        .margin_start(2)
        .margin_end(2)
        .build();

    // Connect button signal
    let window_weak = window.downgrade();
    button.connect_clicked(move |_| {
        // Get a strong reference (to avoid borrowing issues)
        if let Some(window) = window_weak.upgrade() {
            // Create file chooser dialog
            let dialog = FileChooserDialog::new(
                Some("Select a file"),
                Some(&window),
                FileChooserAction::Open,
                &[
                    ("Cancel", ResponseType::Cancel),
                    ("Open", ResponseType::Accept),
                ],
            );

            // Run the dialog (blocks until a response is received)
            let response = dialog.run();
            if response == ResponseType::Accept {
                if let Some(file_path) = dialog.file().and_then(|f| f.path()) {
                    println!("Selected file: {}", file_path.display());
                }
            }

            dialog.close();
        }
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Linux WallpaperEngine GUI")
        .child(&button)
        .build();

    // Present window
    window.present();
}
