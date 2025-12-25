use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr, I18n};

#[component]
pub fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        check_translations: "./src/**/*.rs",
        default_language: "en",
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
pub fn LanguageSelector() -> impl IntoView {
    let i18n = expect_context::<I18n>();

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
        <pre>
            {move_tr!(
                if {i18n.language.get().id == "en"} {
                    "language-is-english"
                } else {
                    "language-is-spanish"
                }
            )}
        </pre>
    }
}
