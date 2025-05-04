use end2end_helpers::{element_text, input_by_id, mount, unmount};
use leptos::prelude::*;
use leptos_fluent::{cookie, leptos_fluent};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const COOKIE_NAME: &str = "my-weird-cookie-name";

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_cookie: true,
        cookie_name: COOKIE_NAME,
        set_language_to_cookie: true,
    };

    view! { <LanguageSelector /> }
}

#[wasm_bindgen_test]
async fn test_cookie() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_cookie
    cookie::delete(COOKIE_NAME);
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();

    cookie::set(COOKIE_NAME, "es", "");
    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    unmount!();

    cookie::set(COOKIE_NAME, "en", "");
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();
}
