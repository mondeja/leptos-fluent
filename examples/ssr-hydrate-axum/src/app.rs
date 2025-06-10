use fluent_templates::{static_loader, StaticLoader};
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr, tr, I18n};
use leptos_meta::{MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use std::sync::LazyLock;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

pub static COMPOUND: &[&LazyLock<StaticLoader>] =
    &[&TRANSLATIONS, &TRANSLATIONS];

#[component]
fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS, TRANSLATIONS] + COMPOUND,
        locales: "./locales",
        default_language: "en",
        check_translations: "./src/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
        initial_language_from_cookie: true,
        initial_language_from_cookie_to_local_storage: true,
        set_language_to_cookie: true,
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_local_storage: true,
        initial_language_from_url_param_to_cookie: true,
        set_language_to_url_param: true,
        local_storage_key: "language",
        initial_language_from_local_storage: true,
        initial_language_from_local_storage_to_cookie: true,
        set_language_to_local_storage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_local_storage: true,
        initial_language_from_accept_language_header: true,
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18nProvider>
            <Title text=move || tr!("welcome-to-leptos") />
            <Router>
                <main>
                    <Routes fallback=|| tr!("not-found").into_view()>
                        <Route path=StaticSegment("/") view=Home />
                    </Routes>
                </main>
            </Router>
        </I18nProvider>
    }
}

#[component]
fn Home() -> impl IntoView {
    let i18n = expect_context::<I18n>();

    view! {
        <h1>{move_tr!("select-a-language")}</h1>
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
                                    checked=&i18n.language.get() == lang
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_axum::ResponseOptions>();
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>{move_tr!("not-found")}</h1> }
}
