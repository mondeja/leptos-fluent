use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{expect_i18n, move_tr};

use crate::app::COMPOUND;
use crate::i18n;

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales/islands",
        // core_locales: "./locales/core",
        fallback_language: "en",
    };
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>{move_tr!("welcome-to-leptos")}</h1>
        <I18Context>
            <LanguageSelector/>
            <Counter/>
        </I18Context>
    }
}

#[island]
fn I18Context(children: Children) -> impl IntoView {
    i18n!([TRANSLATIONS, TRANSLATIONS] + COMPOUND, "./locales/islands");
    children()
}

#[island]
fn Counter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { <button on:click=on_click>{move_tr!("click-me")} " - " {count}</button> }
}

#[island]
fn LanguageSelector() -> impl IntoView {
    // `expect_i18n()` to get the i18n context
    // `i18n.languages` is a static array with the available languages
    // `i18n.language.get()` to get the current language
    // `lang.activate()` to set the current language
    // `lang.is_active()` to check if a language is the current selected one

    view! {
        <fieldset>
            {
                move || expect_i18n().languages.iter().map(|lang| {
                    view! {
                        <div>
                            <input
                                type="radio"
                                id=lang
                                name="language"
                                value=lang
                                checked=lang.is_active()
                                on:click=move |_| lang.activate()
                            />
                            <label for=lang>{lang.name}</label>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </fieldset>
    }
}
// #[island]
// fn LanguageSelector() -> impl IntoView {
//     let i18n = i18n!([TRANSLATIONS, TRANSLATIONS] + COMPOUND);
//
//     view! {
//         <fieldset>
//
//             {move || {
//                 i18n.languages
//                     .iter()
//                     .map(|lang| {
//                         view! {
//                             <div>
//                                 <input
//                                     type="radio"
//                                     id=lang
//                                     name="language"
//                                     value=lang
//                                     checked=*lang == i18n.language.get()
//                                     on:click=move |_| {
//                                         i18n.language.set(lang);
//                                         window()
//                                             .location()
//                                             .reload()
//                                             .expect("Failed to reload the page");
//                                     }
//                                 />
//
//                                 <label for=lang>{lang.name}</label>
//                             </div>
//                         }
//                     })
//                     .collect::<Vec<_>>()
//             }}
//
//         </fieldset>
//     }
// }
