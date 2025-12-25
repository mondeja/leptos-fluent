use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_fluent_macros::leptos_fluent;

static_loader! {
    pub static TRANSLATIONS = {
        locales: "../../../../examples/csr-minimal/locales",
        fallback_language: "en",
    };
}

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "../../../../examples/csr-minimal/locales",
        #[cfg(feature = "ssr")]
        check_translations: "../../../../leptos-fluent-macros/tests/ui/leptos_fluent/fail/check_translations_cfg_feature.rs",
        #[cfg(all(not(feature = "ssr"), not(debug_assertions)))]
        check_translations: "../../../../leptos-fluent-macros/tests/ui/leptos_fluent/fail/check_translations_cfg_feature.rs",
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18n>
            <p>{move_tr!("select-a-language")}</p>
        </I18n>
    }
}

fn main() {}
