use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{i18n, leptos_fluent, tr, Language};

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: TRANSLATIONS,
        locales: "./locales",
    }};

    view! { <OtherComponent/> }
}

#[component]
fn OtherComponent() -> impl IntoView {
    view! {
        <p>{move || tr!("select-a-language")}</p>
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
                                checked=*lang == i18n().language.get()
                                on:click=move |_| i18n().language.set(lang)
                            />
                            <label for=lang.id.to_string()>{lang.name}</label>
                        </div>
                    }
                }
            />

        </fieldset>
    }
}
