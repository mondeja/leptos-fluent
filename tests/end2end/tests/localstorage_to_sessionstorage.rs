use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, localstorage, sessionstorage};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, mount, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const SESSIONSTORAGE_KEY: &str = "my-weird-sessionstorage-key";
const LOCALSTORAGE_KEY: &str = "my-weird-localstorage-key";

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_localstorage: true,
        localstorage_key: LOCALSTORAGE_KEY,
        initial_language_from_localstorage_to_sessionstorage: true,
        sessionstorage_key: SESSIONSTORAGE_KEY,
    };

    view! { <LanguageSelector /> }
}

#[wasm_bindgen_test]
async fn test_localstorage_to_sessionstorage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_localstorage_to_sessionstorage
    sessionstorage::delete(SESSIONSTORAGE_KEY);
    localstorage::delete(LOCALSTORAGE_KEY);
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();

    localstorage::set(LOCALSTORAGE_KEY, "es");
    sessionstorage::delete(SESSIONSTORAGE_KEY);
    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    assert_eq!(
        sessionstorage::get(SESSIONSTORAGE_KEY),
        Some("es".to_string())
    );
    unmount!();

    localstorage::set(LOCALSTORAGE_KEY, "en");
    sessionstorage::delete(SESSIONSTORAGE_KEY);
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    assert_eq!(
        sessionstorage::get(SESSIONSTORAGE_KEY),
        Some("en".to_string())
    );
    unmount!();
}
