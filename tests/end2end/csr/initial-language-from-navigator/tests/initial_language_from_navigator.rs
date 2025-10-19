use end2end_csr_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{cookie, leptos_fluent, local_storage};
use leptos_fluent_csr_minimal_example::LanguageSelector;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const LOCAL_STORAGE_KEY: &str = "ilfn";
const COOKIE: &str = "ilfn";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../../../examples/csr-minimal/locales",
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_local_storage: true,
        initial_language_from_navigator_to_cookie: true,
        local_storage_key: LOCAL_STORAGE_KEY,
        cookie_name: COOKIE,
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <I18n>
            <LanguageSelector />
        </I18n>
    }
}

#[wasm_bindgen_test]
pub async fn test_initial_language_from_navigator() {
    let es = move || input_by_id("es");

    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");

    // *_to_local_storage
    assert_eq!(
        local_storage::get(LOCAL_STORAGE_KEY),
        Some("es".to_string())
    );
    // *_to_cookie
    assert_eq!(cookie::get(COOKIE), Some("es".to_string()));
}
