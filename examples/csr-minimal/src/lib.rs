use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, Language};

static_loader! {
    pub static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: TRANSLATIONS,
        locales: "./locales",
    }};

    view! { <ChildComponent/> }
}

#[component]
pub fn ChildComponent() -> impl IntoView {
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
                                checked=i18n.is_active_language(lang)
                                on:click=move |_| i18n.language.set(lang)
                            />
                            <label for=lang>{lang.name}</label>
                        </div>
                    }
                }
            />

        </fieldset>
    }
}
