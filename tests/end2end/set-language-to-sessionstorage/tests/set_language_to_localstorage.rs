use end2end_helpers::{element_text, input_by_id, mount, sleep};
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, sessionstorage};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const SESSIONSTORAGE_KEY: &str = "sltc";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "../../../examples/csr-minimal/locales",
        initial_language_from_navigator: true,
        sessionstorage_key: SESSIONSTORAGE_KEY,
        set_language_to_sessionstorage: true,
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
async fn test_set_language_to_sessionstorage() {
    let en = move || input_by_id("en");
    let es = move || input_by_id("es");

    mount!(App);
    sessionstorage::delete(SESSIONSTORAGE_KEY);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");

    es().click();
    sleep(30).await;
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    assert_eq!(
        sessionstorage::get(SESSIONSTORAGE_KEY),
        Some("es".to_string())
    );
}
