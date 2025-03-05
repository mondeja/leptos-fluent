use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent_macros::leptos_fluent;

static_loader! {
    pub static TRANSLATIONS = {
        locales: "../../../../examples/csr-minimal/locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    let url_param = 777;
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../../../../examples/csr-minimal/locales",
        url_param,
        initial_language_from_url_param: true,
    };

    view! { <p>Foo</p> }
}

fn main() {}
