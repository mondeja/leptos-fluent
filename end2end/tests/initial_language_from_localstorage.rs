use leptos::*;
use leptos_fluent::leptos_fluent;
use leptos_fluent_csr_minimal_example::{ChildComponent, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, localstorage, mount, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {{
        translations: TRANSLATIONS,
        locales: "../examples/csr-minimal/locales",
        initial_language_from_localstorage: true,
    }};

    view! { <ChildComponent/> }
}

#[wasm_bindgen_test]
async fn initial_language_from_localstorage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    localstorage::set("lang", "es");
    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    unmount!();

    localstorage::set("lang", "en");
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();
}
