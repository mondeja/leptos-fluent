use leptos::prelude::*;
use leptos_fluent::{cookie, leptos_fluent, localstorage};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, mount, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const LOCALSTORAGE_KEY: &str = "ilfn";
const COOKIE: &str = "ilfn";

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_localstorage: true,
        initial_language_from_navigator_to_cookie: true,
        localstorage_key: LOCALSTORAGE_KEY,
        cookie_name: COOKIE,
    };

    view! { <LanguageSelector /> }
}

#[wasm_bindgen_test]
async fn test_initial_language_from_navigator() {
    let es = move || input_by_id("es");

    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");

    // *_to_localstorage
    assert_eq!(localstorage::get(LOCALSTORAGE_KEY), Some("es".to_string()));
    // *_to_cookie
    assert_eq!(cookie::get(COOKIE), Some("es".to_string()));

    unmount!();
}
