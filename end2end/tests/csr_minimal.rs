use leptos_fluent_csr_minimal_example::App as MinimalExampleApp;
use tests_helpers::{
    element_text, html, input_by_id, localstorage, mount, sleep, unmount,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn csr_minimal_example() {
    mount!(MinimalExampleApp);
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // localstorage not activated
    localstorage::set("language", "es");

    // translations working
    assert_eq!(element_text("p"), "Select a language:");
    es().click();
    assert!(es().checked());
    assert!(!en().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");

    // set_language_to_localstorage not activated
    localstorage::delete("language");
    assert_eq!(localstorage::get("language"), None);
    en().click();
    assert_eq!(localstorage::get("language"), None);
    es().click();
    assert_eq!(localstorage::get("language"), None);

    // sync_html_tag_lang not activated
    sleep(30).await;
    html().remove_attribute("lang").unwrap();
    assert_eq!(html().lang(), "".to_string());
    es().click();
    assert!(es().checked());
    assert_eq!(html().lang(), "".to_string());
    en().click();
    assert!(en().checked());
    assert_eq!(html().lang(), "".to_string());

    unmount!();
}
