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
        // A comment
        #[cfg(debug_assertions)]
        sync_html_tag_lang: true,
        #[cfg(not(debug_assertions))]
        sync_html_tag_lang: false,
        #[cfg(debug_assertions)]
        // Other comment
        sync_html_tag_dir: true,
        #[cfg(not(debug_assertions))]
        sync_html_tag_dir: false,
    }};

    view! { <p>Foo</p> }
}

fn main() {}
