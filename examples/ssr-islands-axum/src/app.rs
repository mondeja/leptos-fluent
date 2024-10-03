use crate::error_template::{AppError, ErrorTemplate};
use fluent_templates::{once_cell::sync::Lazy, static_loader, StaticLoader};
use leptos::*;
use leptos_fluent::leptos_fluent;
use leptos_fluent::tr;
use leptos_meta::*;
use leptos_router::*;

mod home;

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales/server",
        // core_locales: "./locales/core",
        fallback_language: "en",
    };
}

pub static COMPOUND: &[&Lazy<StaticLoader>] = &[&TRANSLATIONS, &TRANSLATIONS];

#[macro_export]
macro_rules! i18n {
    ($translations:expr, $locales:expr$(,)?) => {{
        use leptos_fluent::leptos_fluent;
        leptos_fluent! {
            translations: $translations,
            languages: "./locales/languages.json",
            locales: $locales,
            // core_locales: "./locales/core",
            sync_html_tag_lang: true,
            sync_html_tag_dir: true,
            cookie_name: "lang",
            cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
            initial_language_from_cookie: true,
            initial_language_from_cookie_to_localstorage: true,
            set_language_to_cookie: true,
            url_param: "lang",
            initial_language_from_url_param: true,
            initial_language_from_url_param_to_localstorage: true,
            initial_language_from_url_param_to_cookie: true,
            set_language_to_url_param: true,
            localstorage_key: "language",
            initial_language_from_localstorage: true,
            initial_language_from_localstorage_to_cookie: true,
            set_language_to_localstorage: true,
            initial_language_from_navigator: true,
            initial_language_from_navigator_to_localstorage: true,
            initial_language_from_accept_language_header: true,
        }
    }};
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    i18n!([TRANSLATIONS, TRANSLATIONS] + COMPOUND, "./locales/server");

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-fluent-ssr-hydrate-axum-islands-example.css"/>

        // sets the document title
        <Title text=tr!("welcome-to-leptos")/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=home::HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
