use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, Language};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: [TRANSLATIONS],
        languages: "./locales/languages.json",
        locales: "./locales",
        check_translations: "./src/**/*.rs",
        sync_html_tag_lang: true,
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_localstorage: true,
        set_language_to_url_param: true,
        localstorage_key: "language",
        initial_language_from_localstorage: true,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,
    }};

    view! { <ChildComponent/> }
}

#[component]
fn ChildComponent() -> impl IntoView {
    let i18n = expect_i18n();

    view! {
        <p>{move_tr!("select-a-language")}</p>
        <fieldset>
            <For
                each=move || i18n.languages
                key=move |lang| *lang
                children=move |lang: &&Language| {
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
                }
            />

        </fieldset>
        <p>{move_tr!("html-tag-lang-is", { "lang" => i18n.language.get().id.to_string() })}</p>
        <p>{move_tr!("add-es-en-url-param")}</p>
    }
}
