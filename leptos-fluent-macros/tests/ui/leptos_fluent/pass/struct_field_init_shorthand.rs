use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent_macros::leptos_fluent;

static_loader! {
    pub static TRANSLATIONS = {
        locales: "../../../../examples/csr-minimal/locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    let cookie_name = "my_cookie";
    let initial_language_from_cookie = true;

    leptos_fluent! {
        translations: [TRANSLATIONS],
        cookie_name,
        locales: "../../../../examples/csr-minimal/locales",
        initial_language_from_cookie,
    };

    view! { <p>Foo</p> }
}

fn main() {}
