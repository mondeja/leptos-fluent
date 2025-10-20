use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, move_tr};

app_helpers::shell_and_app_impl!(HomePage);

#[component]
fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../../../../../examples/ssr-hydrate-axum/locales",
        initial_language_from_accept_language_header: true,
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <I18nProvider>
            <h1>{move_tr!("welcome-to-leptos")}</h1>
        </I18nProvider>
    }
}
