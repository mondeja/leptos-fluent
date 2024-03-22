use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{i18n, leptos_fluent, move_tr, Language};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: TRANSLATIONS,
        languages: "./locales/languages.json",
        sync_html_tag_lang: true,
        initial_language_from_url: true,
        initial_language_from_url_param: "lang",
        initial_language_from_url_to_localstorage: true,
        initial_language_from_localstorage: true,
        initial_language_from_navigator: true,
        localstorage_key: "language",
    }};

    view! { <ChildComponent/> }
}

#[component]
fn ChildComponent() -> impl IntoView {
    let i18n = i18n();

    view! {
        <p>{move_tr!("select-a-language")}</p>
        <fieldset>
            <For
                each=move || i18n.languages
                key=move |lang| lang.id.to_string()
                children=move |lang: &&Language| {
                    view! {
                        <div>
                            <input
                                type="radio"
                                id=lang.id.to_string()
                                name="language"
                                value=lang.id.to_string()
                                checked=*lang == i18n.language.get()
                                on:click=move |_| i18n.set_language_with_localstorage(lang)
                            />
                            <label for=lang.id.to_string()>{lang.name}</label>
                        </div>
                    }
                }
            />

        </fieldset>
        <p>{move_tr!("html-tag-lang-is", {"lang" => i18n.language.get().id.to_string()})}</p>
        <p>{move_tr!("add-es-en-url-param")}</p>
    }
}
