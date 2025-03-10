use leptos::prelude::*;
use leptos_fluent_macros::leptos_fluent;

#[component]
pub fn I18n() -> impl IntoView {
    leptos_fluent! {}
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18n>
            <p>+</p>
        </I18n>
    }
}

fn main() {}
