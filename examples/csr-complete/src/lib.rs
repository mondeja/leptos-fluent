use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr, I18n, Language};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        languages: "./locales/languages.json",
        locales: "./locales",
        default_language: "es",
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
        initial_language_from_url_param_to_sessionstorage: true,
        initial_language_from_url_param_to_cookie: true,
        set_language_to_url_param: true,
        localstorage_key: "language",
        initial_language_from_localstorage: true,
        initial_language_from_localstorage_to_cookie: true,
        set_language_to_localstorage: true,
        sessionstorage_key: "language",
        initial_language_from_sessionstorage: true,
        initial_language_from_sessionstorage_to_cookie: true,
        initial_language_from_sessionstorage_to_localstorage: true,
        set_language_to_sessionstorage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_cookie: true,
        initial_language_from_navigator_to_localstorage: true,
        initial_language_from_navigator_to_sessionstorage: true,
        set_language_from_navigator: true,
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18nProvider>
            <LanguageSelector />
        </I18nProvider>
    }
}

#[component]
fn LanguageSelector() -> impl IntoView {
    let i18n = expect_context::<I18n>();
    view! {
        <p>{move_tr!("select-a-language")}</p>
        <fieldset>
            {move || {
                i18n.languages.iter().map(|lang| render_language(lang)).collect::<Vec<_>>()
            }}
        </fieldset>

        <ul>
            <li>
                <p>
                    {move_tr!(
                        "html-tag-lang-is", { "lang" => i18n.language.read_untracked().id.to_string() }
                    )}
                </p>
                <p>{move_tr!("add-es-en-url-param")}</p>
            </li>
            <li>
                <p>
                    {move_tr!(
                        "html-tag-dir-is", { "dir" => i18n.language.read_untracked().dir.to_string() }
                    )}
                </p>
            </li>
        </ul>
    }
}

fn render_language(lang: &'static Language) -> impl IntoView {
    // Passed as atrribute, `Language` is converted to their code,
    // so `<input id=lang` becomes `<input id=lang.id.to_string()`
    let i18n = expect_context::<I18n>();
    view! {
        <div>
            <input
                id=lang
                name="language"
                value=lang
                checked=i18n.language.get() == lang
                on:click=move |_| i18n.language.set(lang)
                type="radio"
            />
            <label for=lang>{lang.name}</label>
        </div>
    }
}
