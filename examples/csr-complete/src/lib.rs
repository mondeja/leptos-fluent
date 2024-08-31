use fluent_templates::static_loader;
use leptos::prelude::*;
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
        child: LanguageSelector,
        translations: [TRANSLATIONS],
        languages: "./locales/languages.json",
        locales: "./locales",
        check_translations: "./src/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; Path=/; Max-Age=600",
        set_language_to_cookie: true,
        initial_language_from_cookie: true,
        initial_language_from_cookie_to_localstorage: true,
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
        initial_language_from_navigator_to_cookie: true,
        initial_language_from_navigator_to_localstorage: true,
        set_language_from_navigator: true,
    }}
}

#[component]
fn LanguageSelector() -> impl IntoView {
    view! {
        <p>{move_tr!("select-a-language")}</p>
        <fieldset>

            {move || {
                expect_i18n().languages.iter().map(|lang| render_language(lang)).collect::<Vec<_>>()
            }}

        </fieldset>

        <ul>
            <li>
                <p>
                    {move_tr!("html-tag-lang-is", { "lang" => expect_i18n().language.read().id.to_string() })}
                </p>
                <p>{move_tr!("add-es-en-url-param")}</p>
            </li>
            <li>
                <p>
                    {move_tr!("html-tag-dir-is", { "dir" => expect_i18n().language.read().dir.to_string() })}
                </p>
            </li>
        </ul>
    }
}

fn render_language(lang: &'static Language) -> impl IntoView {
    // Passed as atrribute, `Language` is converted to their code,
    // so `<input id=lang` becomes `<input id=lang.id.to_string()`
    let i18n = expect_i18n();
    view! {
        <div>
            <input
                id=lang
                name="language"
                value=lang
                checked=lang.is_active()
                on:click=move |_| i18n.language.set(lang)
                type="radio"
            />
            <label for=lang>{lang.name}</label>
        </div>
    }
}
