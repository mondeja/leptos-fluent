use end2end_helpers::{element_text, input_by_id, mount, sleep_a_moment};
use gloo_utils::document_element;
use leptos::prelude::*;
use leptos_fluent::localstorage;
use leptos_fluent_csr_complete_example::App;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn csr_complete_example() {
    localstorage::delete("language");

    mount!(App);
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // translations working
    en().click();
    sleep_a_moment().await;
    assert_eq!(element_text("p"), "Select a language:");
    es().click();
    sleep_a_moment().await;
    assert!(es().checked());
    assert!(!en().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    en().click();
    sleep_a_moment().await;
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    assert!(!es().checked());

    // sync_html_tag_lang
    es().click();
    sleep_a_moment().await;
    assert!(es().checked());
    assert_eq!(
        document_element().get_attribute("lang"),
        Some("es".to_string())
    );
    en().click();
    sleep_a_moment().await;
    assert_eq!(
        document_element().get_attribute("lang"),
        Some("en".to_string())
    );

    // sync_html_tag_dir
    assert_eq!(
        document_element().get_attribute("dir"),
        Some("ltr".to_string())
    );
    es().click();
    sleep_a_moment().await;
    assert_eq!(
        document_element().get_attribute("dir"),
        Some("auto".to_string())
    );
    en().click();
    sleep_a_moment().await;

    // set_language_to_localstorage
    localstorage::delete("language");
    assert_eq!(localstorage::get("language"), None);
    es().click();
    sleep_a_moment().await;
    assert_eq!(localstorage::get("language"), Some("es".to_string()));
    en().click();
    sleep_a_moment().await;
    assert_eq!(localstorage::get("language"), Some("en".to_string()));
    localstorage::delete("language");
}
