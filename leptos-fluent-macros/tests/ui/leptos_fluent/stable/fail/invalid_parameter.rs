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
pub fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "../../../../examples/csr-minimal/locales",
        invalid_parameter: "foo",
    }
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
