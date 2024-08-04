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

macro_rules! setup_logging {
    () => {
        let (non_blocking, _guard) =
            tracing_appender::non_blocking(std::io::stdout());
        let filter = tracing_subscriber::EnvFilter::builder()
            .with_default_directive(
                tracing::metadata::LevelFilter::TRACE.into(),
            )
            .from_env()
            .unwrap()
            .add_directive("leptos_fluent_gtk_example=trace".parse().unwrap())
            .add_directive("leptos_fluent=trace".parse().unwrap());

        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_env_filter(filter)
            .init();
    };
}

#[tracing::instrument(level = "trace")]
fn main() {
    setup_logging!();

    let _ = create_runtime();
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    let i18n = leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "./locales",
        check_translations: "./src/**/*.rs",
        initial_language_from_data_file: true,
        initial_language_from_system: true,
        initial_language_from_system_to_data_file: true,
        set_language_to_data_file: true,
        data_file_key: "leptos-fluent-gtk-example",
        #[cfg(debug_assertions)]
        provide_meta_context: true,
    }};

    #[cfg(debug_assertions)]
    tracing::info!("Macro parameters: {:?}", i18n.meta().unwrap());

    // Run the application
    app.run();
}

#[tracing::instrument(level = "trace", skip(app))]
fn build_ui(app: &Application) {
    let button = counter_button();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("leptos-fluent + gtk4")
        .child(&button)
        .build();

    window.present();
}

#[tracing::instrument(level = "trace")]
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
        tracing::debug!(
            "Initial language set to \"{}\"",
            expect_i18n().language.get_untracked().id
        );
        move |_| {
            tracing::debug!(
                "New language set to \"{}\"",
                expect_i18n().language.get_untracked().id
            );
            button
                .set_label(&tr!("count", { "times" => value.get_untracked()}));
        }
    });

    button
}
