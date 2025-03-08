use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr};

static_loader! {
    pub static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

fn get_language_from_url_path(path: &str) -> &str {
    if let Some(language) = path.split('/').nth(1) {
        return language;
    }
    ""
}

#[component]
pub fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "./locales",
        url_path: get_language_from_url_path,
        initial_language_from_url_path: true,
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18n>
            <LanguageSelector />
        </I18n>
    }
}

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let i18n = expect_i18n();

    view! {
        <p>{move_tr!("select-a-language")}</p>
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
