use leptos::*;
use leptos_fluent::{cookie, leptos_fluent, localstorage};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, mount, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const COOKIE_NAME: &str = "my-weird-cookie-name";
const LOCALSTORAGE_KEY: &str = "my-weird-localstorage-key";

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../examples/csr-minimal/locales",
        initial_language_from_cookie: true,
        cookie_name: COOKIE_NAME,
        initial_language_from_cookie_to_localstorage: true,
        localstorage_key: LOCALSTORAGE_KEY,
    };

    view! { <LanguageSelector /> }
}

#[wasm_bindgen_test]
async fn test_cookie_to_localstorage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_cookie_to_localstorage
    cookie::delete(COOKIE_NAME);
    localstorage::delete(LOCALSTORAGE_KEY);
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();

    cookie::set(COOKIE_NAME, "es", "");
    localstorage::delete(LOCALSTORAGE_KEY);
    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    assert_eq!(localstorage::get(LOCALSTORAGE_KEY), Some("es".to_string()));
    unmount!();

    cookie::set(COOKIE_NAME, "en", "");
    localstorage::delete(LOCALSTORAGE_KEY);
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    assert_eq!(localstorage::get(LOCALSTORAGE_KEY), Some("en".to_string()));
    unmount!();
}
