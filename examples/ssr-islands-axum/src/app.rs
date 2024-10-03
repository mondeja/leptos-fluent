use crate::error_template::{AppError, ErrorTemplate};
use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::tr;
use leptos_fluent::{expect_i18n, move_tr};
use leptos_meta::*;
use leptos_router::*;

static_loader! {
    static COMPONENTS_TRANSLATIONS = {
        locales: "./locales/server",
        fallback_language: "en",
    };
}

static_loader! {
    static ISLANDS_TRANSLATIONS = {
        locales: "./locales/islands",
        fallback_language: "en",
    };
}

#[macro_export]
macro_rules! i18n {
    ($translations:expr, $locales:expr$(,)?) => {{
        ::leptos_fluent::leptos_fluent! {
            translations: $translations,
            languages: "./locales/languages.json",
            locales: $locales,
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

    i18n!([COMPONENTS_TRANSLATIONS], "./locales/server");

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-fluent-ssr-hydrate-axum-islands-example.css" />

        <Title text=tr!("welcome-to-leptos") />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Archipelago>
            <Counter />
            <LanguageSelector />
        </Archipelago>
    }
}

#[island]
fn Archipelago(children: Children) -> impl IntoView {
    i18n!([ISLANDS_TRANSLATIONS], "./locales/islands");

    // Children will be executed when the i18n context is ready.
    // See https://book.leptos.dev/islands.html#passing-context-between-islands
    //
    // > That’s why in `HomePage`, I made `let tabs = move ||` a function, and
    // > called it like `{tabs()}`: creating the tabs lazily this way meant that
    // > the `Tabs` island would already have provided the `selected` context by
    // > the time each `Tab` went looking for it.
    children()
}

#[island]
fn Counter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { <button on:click=on_click>{move_tr!("click-me")} " - " {count}</button> }
}

#[island]
fn LanguageSelector() -> impl IntoView {
    let i18n = expect_i18n();
    view! {
        <fieldset>
            {move || {
                i18n.languages
                    .iter()
                    .map(|lang| {
                        view! {
                            <div>
                                <input
                                    type="radio"
                                    id=lang
                                    name="language"
                                    value=lang
                                    checked=lang.is_active()
                                    on:click=move |_| i18n.language.set(lang)
                                />
                                <label for=lang>{lang.name}</label>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            }}
        </fieldset>
    }
}
