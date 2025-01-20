use leptos::prelude::*;
use leptos_fluent::leptos_fluent;
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, mount, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_navigator: true,
    };

    view! { <LanguageSelector /> }
}

#[wasm_bindgen_test]
async fn test_initial_language_from_accept_language_header() {
    let es = move || input_by_id("es");

    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    unmount!();
}
