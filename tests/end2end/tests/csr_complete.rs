use end2end_helpers::{element_text, input_by_id, mount};
use gloo_utils::document_element;
use leptos::prelude::*;
use leptos_fluent::localstorage;
use leptos_fluent_csr_complete_example::App;
use wasm_bindgen_test::*;
use web_sys_ec::{By, Ec, Wait};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn csr_complete_example() {
    localstorage::delete("language");

    mount!(App);
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // translations working
    en().click();
    Wait(1)
        .until((
            By::TagName("p"),
            Ec::InnerTextContains("Select a language:"),
        ))
        .await;
    es().click();
    Wait(1)
        .until((
            By::TagName("p"),
            Ec::InnerTextContains("Selecciona un idioma:"),
        ))
        .await;
    assert!(es().checked());
    assert!(!en().checked());
    en().click();
    Wait(1)
        .until((
            By::TagName("p"),
            Ec::InnerTextContains("Select a language:"),
        ))
        .await;
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    assert!(!es().checked());

    // sync_html_tag_lang
    es().click();
    Wait(1)
        .until((By::TagName("html"), Ec::AttributeValueIs("lang", "es")))
        .await;
    assert!(es().checked());
    en().click();
    Wait(1)
        .until((By::TagName("html"), Ec::AttributeValueIs("lang", "en")))
        .await;

    // sync_html_tag_dir
    assert_eq!(
        document_element().get_attribute("dir"),
        Some("ltr".to_string())
    );
    es().click();
    Wait(1)
        .until((By::TagName("html"), Ec::AttributeValueIs("dir", "auto")))
        .await;
    en().click();
    Wait(1)
        .until((By::TagName("html"), Ec::AttributeValueIs("dir", "auto")))
        .await;

    // set_language_to_localstorage
    localstorage::delete("language");
    assert_eq!(localstorage::get("language"), None);
    es().click();
    Wait(1)
        .until(Ec::LocalStorageAttributeValueIs("language", "es"))
        .await;
    en().click();
    Wait(1)
        .until(Ec::LocalStorageAttributeValueIs("language", "en"))
        .await;
    localstorage::delete("language");
}
