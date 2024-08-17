use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr};

static_loader! {
    pub static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[derive(Clone)]
struct Foo {
    bar: String,
}

#[component]
pub fn App() -> impl IntoView {
    let foo = Foo {
        bar: "baz".to_string(),
    };
    ::leptos::prelude::provide_context::<Foo>(foo);


    leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "./locales",
    }};

    view! { <LanguageSelector/> }
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
                        ::leptos::prelude::expect_context::<Foo>();
                        ::leptos::logging::log!("here");
                        view! {
                            <div>
                                <input
                                    type="radio"
                                    id=lang
                                    name="language"
                                    value=lang
                                    checked=lang.is_active()
                                    on:click=move |_| {
                                        ::leptos::prelude::expect_context::<Foo>();
                                        lang.activate()
                                    }
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
