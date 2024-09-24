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
    leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "../../../../examples/csr-minimal/locales",
        url_param: ,
    }};

    view! { <p>Foo</p> }
}

fn main() {}
