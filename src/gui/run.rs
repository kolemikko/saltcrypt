use crate::core;
use gtk::{
    glib, prelude::*, Application, ApplicationWindow, Button, CheckButton, FileChooserAction,
    FileChooserDialog, Grid, Label, PasswordEntry, ResponseType, Statusbar,
};

pub fn start() {
    let app = Application::builder().build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let builder = gtk::Builder::from_string(include_str!("ui.xml"));

    let window: ApplicationWindow = builder
        .object("window")
        .expect("Failed getting object 'window' from builder");
    window.set_application(Some(app));

    let grid: Grid = builder
        .object("grid")
        .expect("Failed getting object 'grid' from builder");
    grid.set_halign(gtk::Align::Center);
    grid.set_valign(gtk::Align::Center);
    window.set_child(Some(&grid));

    let statusbar: Statusbar = builder
        .object("statusbar")
        .expect("Failed getting object 'statusbar' from builder");
    let enc_mode_button: CheckButton = builder
        .object("enc_mode_button")
        .expect("Failed getting object 'enc_mode_button' from builder");
    let dec_mode_button: CheckButton = builder
        .object("dec_mode_button")
        .expect("Failed getting object 'dec_mode_button' from builder");
    enc_mode_button.set_group(Some(&dec_mode_button));

    let file_view: Label = builder
        .object("file_view")
        .expect("Failed getting object 'file_view' from builder");
    let open_button: Button = builder
        .object("open_button")
        .expect("Failed getting object 'open_button' from builder");

    let password_entry: PasswordEntry = builder
        .object("password_entry")
        .expect("Failed getting object 'password_entry' from builder");
    let salt_entry: PasswordEntry = builder
        .object("salt_entry")
        .expect("Failed getting object 'salt_entry' from builder");
    let execute_button: Button = builder
        .object("execute_button")
        .expect("Failed getting object 'execute_button' from builder");

    open_button.connect_clicked(glib::clone!(@weak window, @weak file_view  => move |_| {

        let dialog = FileChooserDialog::new(
            Some("Select a file"),
            Some(&window),
            FileChooserAction::Open,
            &[("Select", ResponseType::Ok), ("Cancel", ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: ResponseType| {
            if response == ResponseType::Ok {
                let file = d.file().expect("Couldn't get file");
                let filepath = file.path().unwrap().into_os_string().into_string().expect("Couldn't get file path");
                file_view.set_label(&filepath);
            }
            d.close();
        });
        dialog.show();
    }));

    execute_button.connect_clicked(move |_| {
        if file_view.text().is_empty() {
            statusbar.push(
                statusbar.context_id("error"),
                "Please give valid password and salt.",
            );
        } else {
            if password_entry.text().is_empty() || salt_entry.text().is_empty() {
                statusbar.push(
                    statusbar.context_id("error"),
                    "Please give valid password and salt.",
                );
            } else {
                if enc_mode_button.is_active() {
                    match core::encrypt_file(
                        file_view.text().to_string(),
                        &password_entry.text(),
                        &salt_entry.text(),
                    ) {
                        Ok(_) => {
                            statusbar
                                .push(statusbar.context_id("encrypt"), "Encrypted succesfully!");
                            password_entry.set_text("");
                            salt_entry.set_text("");
                        }
                        Err(err) => {
                            let error_msg = format!("Encryption failed: {}", err);
                            statusbar.push(statusbar.context_id("error"), &error_msg);
                        }
                    };
                }

                if dec_mode_button.is_active() {
                    match core::decrypt_file(
                        file_view.text().to_string(),
                        &password_entry.text(),
                        &salt_entry.text(),
                    ) {
                        Ok(_) => {
                            statusbar
                                .push(statusbar.context_id("decrypt"), "Decrypted succesfully!");
                            password_entry.set_text("");
                            salt_entry.set_text("");
                        }
                        Err(err) => {
                            let error_msg = format!("Decryption failed: {}", err);
                            statusbar.push(statusbar.context_id("error"), &error_msg);
                        }
                    };
                }
            }
        }
    });

    window.show();
}
