use leptos::*;
use leptos_fluent::{leptos_fluent, localstorage};
use leptos_fluent_csr_minimal_example::{ChildComponent, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, mount, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const LOCALSTORAGE_KEY: &str = "foobarbaz";

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: TRANSLATIONS,
        locales: "../examples/csr-minimal/locales",
        initial_language_from_localstorage: true,
        localstorage_key: LOCALSTORAGE_KEY,
        set_language_to_localstorage: true,
    }};

    view! { <ChildComponent/> }
}

#[wasm_bindgen_test]
async fn initial_language_from_localstorage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    localstorage::set(LOCALSTORAGE_KEY, "es");
    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    unmount!();

    localstorage::set(LOCALSTORAGE_KEY, "en");
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();
}
