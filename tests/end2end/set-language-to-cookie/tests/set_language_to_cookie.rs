use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{cookie, leptos_fluent};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use wasm_bindgen_test::*;
use web_sys_ec::{By, Ec, Wait};

wasm_bindgen_test_configure!(run_in_browser);

const COOKIE: &str = "sltc";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "../../../examples/csr-minimal/locales",
        initial_language_from_navigator: true,
        cookie_name: COOKIE,
        set_language_to_cookie: true,
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18n>
            <LanguageSelector />
        </I18n>
    }
}

#[wasm_bindgen_test]
async fn test_set_language_to_cookie() {
    let en = move || input_by_id("en");
    let es = move || input_by_id("es");

    mount!(App);
    cookie::delete(COOKIE);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");

    es().click();
    Wait(1)
        .until((
            By::TagName("p"),
            Ec::InnerTextContains("Selecciona un idioma:"),
        ))
        .await;
    assert!(es().checked());
    assert_eq!(cookie::get(COOKIE), Some("es".to_string()));
}
