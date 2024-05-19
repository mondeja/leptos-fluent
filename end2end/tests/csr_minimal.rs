use tests_helpers::{
    element_text, html, input_by_id, localstorage, mount, sleep, unmount,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn csr_minimal_example() {
    use leptos_fluent_csr_minimal_example::App as MinimalExampleApp;
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

    // language change not reflected in html tag
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
