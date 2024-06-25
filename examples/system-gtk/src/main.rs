use fluent_templates::static_loader;
use gtk::{prelude::*, Application, ApplicationWindow, Button};
use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, tr};

const APP_ID: &str = "dev.leptos.Counter";

static_loader! {
    pub static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

// Basic GTK app setup from https://gtk-rs.org/gtk4-rs/stable/latest/book/hello_world.html
fn main() {
    let _ = create_runtime();
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "./locales",
        check_translations: "./src/**/*.rs",
        initial_language_from_system: true,
        set_language_to_data_file: true,
        initial_language_from_data_file: true,
        data_file_key: "gtk-example",
        initial_language_from_system_to_data_file: true,
    }};

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let button = counter_button();

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("leptos-fluent + gtk4")
        .child(&button)
        .build();

    // Present window
    window.present();
}

fn counter_button() -> Button {
    let (value, set_value) = create_signal(0);

    // Create a button with label and margins
    let button = Button::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |_| {
        // Set the label to "Hello World!" after the button has been clicked on
        set_value.update(|value| *value += 1);
        let i18n = expect_i18n();
        let new_lang =
            match i18n.language.get_untracked().id.to_string().as_str() {
                "en" => i18n.languages[1],
                "es" => i18n.languages[0],
                _ => i18n.languages[0],
            };
        i18n.language.set(new_lang);
    });

    create_effect({
        let button = button.clone();
        move |_| {
            button
                .set_label(&tr!("count", { "times" => value.get_untracked()}));
        }
    });

    button
}
