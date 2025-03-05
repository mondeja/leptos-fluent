use leptos::prelude::*;
use leptos_fluent::{cookie, leptos_fluent, url};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use tests_helpers::{element_text, input_by_id, mount, unmount};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const URL_PARAM: &str = "my-weird-url-param";
const COOKIE_NAME: &str = "my-weird-cookie-name";

#[component]
pub fn App() -> impl IntoView {
    leptos_fluent! {
        translations: [TRANSLATIONS],
        locales: "../examples/csr-minimal/locales",
        initial_language_from_url_param: true,
        url_param: URL_PARAM,
        initial_language_from_url_param_to_cookie: true,
        cookie_name: COOKIE_NAME,
    };

    view! { <LanguageSelector /> }
}

#[wasm_bindgen_test]
async fn test_url_param_to_cookie() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_url_param_to_cookie
    url::param::delete(URL_PARAM);
    cookie::delete(COOKIE_NAME);
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    unmount!();

    url::param::set(URL_PARAM, "es");
    cookie::delete(COOKIE_NAME);
    mount!(App);
    assert!(es().checked());
    assert_eq!(element_text("p"), "Selecciona un idioma:");
    assert_eq!(cookie::get(COOKIE_NAME), Some("es".to_string()));
    unmount!();

    url::param::set(URL_PARAM, "en");
    cookie::delete(COOKIE_NAME);
    mount!(App);
    assert!(en().checked());
    assert_eq!(element_text("p"), "Select a language:");
    assert_eq!(cookie::get(COOKIE_NAME), Some("en".to_string()));
    unmount!();
}
