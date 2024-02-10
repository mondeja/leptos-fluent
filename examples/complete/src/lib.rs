use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{i18n, leptos_fluent, Language};
use std::collections::HashMap;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        locales: LOCALES,
        languages: "./locales/languages.json",
        sync_html_tag_lang: true,
        initial_language_from_url: true,
        initial_language_from_url_param: "lang",
        initial_language_from_url_to_localstorage: true,
        initial_language_from_localstorage: true,
        initial_language_from_navigator: true,
        localstorage_key: "language",
        provide_context: true,
    }};

    view! { <OtherComponent/> }
}

#[component]
fn OtherComponent() -> impl IntoView {
    let i18n_ctx = i18n();

    view! {
        <p>{move || i18n().tr("select-a-language")}</p>
        <fieldset>
            <For
                each=move || i18n().languages
                key=move |lang| lang.id.to_string()
                children=move |lang: &&Language| {
                    view! {
                        <div>
                            <input
                                type="radio"
                                id=lang.id.to_string()
                                name="language"
                                value=lang.id.to_string()
                                checked=*lang == i18n_ctx.language.get()
                                on:click=move |_| i18n().language.set(lang)
                            />
                            <label for=lang.id.to_string()>{lang.name}</label>
                        </div>
                    }
                }
            />

        </fieldset>
        <p>
            {move || {
                let i18n = i18n();
                i18n.trs(
                    "html-tag-lang-is-now",
                    &{
                        let mut map = HashMap::new();
                        map.insert("lang".to_string(), i18n.language.get().id.to_string().into());
                        map
                    },
                )
            }}

        </p>
        <p>{move || i18n().tr("add-es-en-url-param")}</p>
    }
}
