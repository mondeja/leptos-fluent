use leptos_fluent_csr_complete_example::App as CompleteExampleApp;
use tests_helpers::{
    element_text, html, input_by_id, localstorage, mount, sleep, unmount,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn csr_complete_example() {
    localstorage::delete("language");

    mount!(CompleteExampleApp);
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // translations working
    en().click();
    assert_eq!(element_text("p"), "Select a language:");
    es().click();
    assert!(es().checked());
    assert!(!en().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    en().click();
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    assert!(!es().checked());

    // sync_html_tag_lang
    es().click();
    sleep(30).await;
    assert!(es().checked());
    assert_eq!(html().lang(), "es".to_string());
    en().click();
    sleep(30).await;
    assert_eq!(html().lang(), "en".to_string());

    // set_language_to_localstorage
    localstorage::delete("language");
    assert_eq!(localstorage::get("language"), None);
    es().click();
    assert_eq!(localstorage::get("language"), Some("es".to_string()));
    en().click();
    assert_eq!(localstorage::get("language"), Some("en".to_string()));
    localstorage::delete("language");

    unmount!();
}
