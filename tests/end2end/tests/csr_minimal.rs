use end2end_helpers::{element_text, html, input_by_id, mount, sleep, unmount};
use leptos::prelude::*;
use leptos_fluent::localstorage;
use leptos_fluent_csr_minimal_example::App;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn csr_minimal_example() {
    mount!(App);
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // localstorage not activated
    localstorage::set("language", "es");

    // translations working
    assert_eq!(element_text("p"), "Select a language:");
    es().click();
    sleep(30).await;
    assert!(es().checked());
    assert!(!en().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");

    // set_language_to_localstorage not activated
    localstorage::delete("language");
    assert_eq!(localstorage::get("language"), None);
    en().click();
    sleep(30).await;
    assert_eq!(localstorage::get("language"), None);
    es().click();
    sleep(30).await;
    assert_eq!(localstorage::get("language"), None);

    // sync_html_tag_lang not activated
    sleep(30).await;
    html().remove_attribute("lang").unwrap();
    assert_eq!(html().lang(), "".to_string());
    es().click();
    sleep(30).await;
    assert!(es().checked());
    assert_eq!(html().lang(), "".to_string());
    en().click();
    sleep(30).await;
    assert!(en().checked());
    assert_eq!(html().lang(), "".to_string());

    unmount!();
}
