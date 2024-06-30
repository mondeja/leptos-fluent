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
    let i18n = leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "../../../../examples/csr-minimal/locales",
        // A comment
        provide_meta_context: true,
    }};

    logging::log!("I18n context: {:?}", i18n.meta().unwrap());

    view! { <p>Foo</p> }
}

fn main() {}
