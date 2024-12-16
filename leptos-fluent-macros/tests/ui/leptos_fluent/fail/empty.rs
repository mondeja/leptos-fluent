use leptos::*;
use leptos_fluent_macros::leptos_fluent;

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {};

    view! { <p>Foo</p> }
}

fn main() {}
