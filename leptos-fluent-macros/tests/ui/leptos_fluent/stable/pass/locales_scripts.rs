use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr};

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../../../leptos-fluent-macros/tests/ui/leptos_fluent/stable/pass/locales_scripts",
        default_language: "sr-Latn",
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18n>
            <p>{move_tr!("hello")}</p>
        </I18n>
    }
}

fn main() {}
