use leptos::*;
use leptos_fluent_macros::leptos_fluent;

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        locales: "../../../../examples/csr-minimal/locales",
        translations: [],
    };

    view! { <p></p> }
}

fn main() {}
