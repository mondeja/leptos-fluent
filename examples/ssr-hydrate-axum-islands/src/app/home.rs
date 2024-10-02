use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::move_tr;

use crate::app::COMPOUND;
use crate::i18n;

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1>{move_tr!("welcome-to-leptos")}</h1>
        <LanguageSelector/>
        <Counter/>
    }
}

#[island]
fn Counter() -> impl IntoView {
    i18n!([TRANSLATIONS, TRANSLATIONS] + COMPOUND);

    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { <button on:click=on_click>{move_tr!("click-me")} " - " {count}</button> }
}

#[island]
fn LanguageSelector() -> impl IntoView {
    let i18n = i18n!([TRANSLATIONS, TRANSLATIONS] + COMPOUND);

    view! {
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
                                    checked=*lang == i18n.language.get()
                                    on:click=move |_| {
                                        i18n.language.set(lang);
                                        window()
                                            .location()
                                            .reload()
                                            .expect("Failed to reload the page");
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
