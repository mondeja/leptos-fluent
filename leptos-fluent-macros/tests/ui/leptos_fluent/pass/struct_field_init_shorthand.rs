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
fn I18n(children: Children) -> impl IntoView {
    let cookie_name = "my_cookie";
    let initial_language_from_cookie = true;

    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        cookie_name,
        locales: "../../../../examples/csr-minimal/locales",
        initial_language_from_cookie,
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
