// Allow to pass a single file path to `check_translations` parameter of
// `leptos_fluent!` macro.
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_fluent_macros::leptos_fluent;

#[component]
pub fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../../../examples/csr-minimal/locales",
        check_translations: "../../../../leptos-fluent-macros/tests/ui/leptos_fluent/stable/pass/tr_checker_single_file.rs",
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <p>{move_tr!("select-a-language")}</p>
        <p>{move_tr!("language-is-english")}</p>
        <p>{move_tr!("language-is-spanish")}</p>
    }
}

fn main() {}
