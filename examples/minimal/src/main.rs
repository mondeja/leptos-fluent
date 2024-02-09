use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{i18n, leptos_fluent, Language};

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[component]
fn App() -> impl IntoView {
    leptos_fluent! {{
        locales: LOCALES,
        languages: "./locales/languages.json",
    }}
    .provide_context(None);

    view! { <OtherComponent/> }
}

#[component]
fn OtherComponent() -> impl IntoView {
    let i18n_ctx = i18n();

    view! {
        <p>{move || i18n().tr("select-a-language")}</p>
        <select on:change=move |ev| {
            let value = event_target_value(&ev);
            let language = i18n_ctx.language_from_str(&value).unwrap();
            i18n_ctx.language.set(language);
        }>
            <For
                each=move || i18n().languages
                key=move |lang| lang.id.to_string()
                children=move |lang: &&Language| {
                    view! { <option value=lang.id.to_string()>{lang.name}</option> }
                }
            />

        </select>
    }
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}
