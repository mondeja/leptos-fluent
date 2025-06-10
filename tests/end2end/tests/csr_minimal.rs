use end2end_helpers::{element_text, input_by_id, mount};
use gloo_utils::document_element;
use leptos::prelude::*;
use leptos_fluent::localstorage;
use leptos_fluent_csr_minimal_example::App;
use wasm_bindgen_test::*;
use web_sys_ec::{Ec, Wait};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub async fn csr_minimal_example() {
    mount!(App);
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // localstorage not active
    localstorage::set("language", "es");

    // translations working
    assert_eq!(element_text("p"), "Select a language:");
    es().click();
    Wait(1)
        .until(("p", Ec::InnerTextContains("Selecciona un idioma:")))
        .await;
    assert!(es().checked());
    assert!(!en().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");

    // set_language_to_localstorage not active
    localstorage::delete("language");
    assert_eq!(localstorage::get("language"), None);
    en().click();
    Wait(1)
        .until(("p", Ec::InnerTextContains("Select a language:")))
        .await;
    assert_eq!(localstorage::get("language"), None);
    es().click();
    Wait(1)
        .until(("p", Ec::InnerTextContains("Selecciona un idioma:")))
        .await;
    assert_eq!(localstorage::get("language"), None);

    // sync_html_tag_lang not active
    document_element().remove_attribute("lang").unwrap();
    assert_eq!(document_element().get_attribute("lang"), None);
    es().click();
    Wait(1)
        .until(("p", Ec::InnerTextContains("Selecciona un idioma:")))
        .await;
    assert!(es().checked());
    assert_eq!(document_element().get_attribute("lang"), None);
    en().click();
    Wait(1)
        .until(("p", Ec::InnerTextContains("Select a language:")))
        .await;
    assert!(en().checked());
    assert_eq!(document_element().get_attribute("lang"), None);
}
