use leptos::prelude::*;
use leptos_fluent::{cookie, leptos_fluent};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, mount, sleep, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const COOKIE_NAME: &str = "my-weird-cookie-name";

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../examples/csr-minimal/locales",
        initial_language_from_cookie: true,
        cookie_name: COOKIE_NAME,
        cookie_attrs: "SameSite=Lax",
        set_language_to_cookie: true,
    };

    view! { <LanguageSelector /> }
}

#[wasm_bindgen_test]
async fn test_cookie_attrs() {
    let es = move || input_by_id("es");

    mount!(App);
    es().click();
    sleep(30).await;
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    assert_eq!(cookie::get_attrs(COOKIE_NAME), "SameSite=Lax".to_string());
    unmount!();
}
