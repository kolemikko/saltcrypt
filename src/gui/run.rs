use crate::core;
use gtk::{
    gdk::Display, glib, prelude::*, Application, ApplicationWindow, Button, CheckButton,
    CssProvider, FileChooserAction, FileChooserDialog, Grid, Label, PasswordEntry, ResponseType,
    Statusbar, StyleContext,
};

trait Post {
    fn post(&self, text: &str);
}

impl Post for Statusbar {
    fn post(&self, text: &str) {
        self.push(self.context_id("msg"), text);
    }
}

pub fn start() {
    let app = Application::builder().build();
    glib::set_program_name(Some("Saltcrypt"));
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
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

    execute_button.connect_clicked(glib::clone!(@weak statusbar => move |_| {
        if file_view.text().is_empty() {
            statusbar.post("Please select a file first.");
        } else if password_entry.text().is_empty() || salt_entry.text().is_empty() {
            statusbar.post("Please give valid password and salt.");
        } else {
            if enc_mode_button.is_active() {
                match core::encrypt_file(
                    file_view.text().to_string(),
                    &password_entry.text(),
                    &salt_entry.text(),
                ) {
                    Ok(_) => {
                        statusbar.post("Encrypted succesfully!");
                        password_entry.set_text("");
                        salt_entry.set_text("");
                    }
                    Err(err) => {
                        statusbar.post(&format!("Encryption failed: {}", err));
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
                        statusbar.post("Decrypted succesfully!");
                        password_entry.set_text("");
                        salt_entry.set_text("");
                    }
                    Err(_) => {
                        statusbar.post("Error: Wrong password or salt.");
                    }
                };
            }
        }
    }));

    window.show();

    let clear_status = move || {
        statusbar.remove_all(statusbar.context_id("msg"));
        glib::Continue(true)
    };

    glib::timeout_add_seconds_local(10, clear_status);
}
