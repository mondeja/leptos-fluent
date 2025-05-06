use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{cookie, leptos_fluent, localstorage};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const COOKIE_NAME: &str = "my-weird-cookie-name";
const LOCALSTORAGE_KEY: &str = "my-weird-localstorage-key";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_localstorage: true,
        localstorage_key: LOCALSTORAGE_KEY,
        initial_language_from_localstorage_to_cookie: true,
        cookie_name: COOKIE_NAME,
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
async fn test_localstorage_to_cookie() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_localstorage_to_cookie
    cookie::delete(COOKIE_NAME);
    localstorage::delete(LOCALSTORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }

    localstorage::set(LOCALSTORAGE_KEY, "es");
    cookie::delete(COOKIE_NAME);
    {
        mount!(App);
        assert!(es().checked());
        assert_eq!(element_text("p"), "Selecciona un idioma:");
        assert_eq!(cookie::get(COOKIE_NAME), Some("es".to_string()));
    }

    localstorage::set(LOCALSTORAGE_KEY, "en");
    cookie::delete(COOKIE_NAME);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
        assert_eq!(cookie::get(COOKIE_NAME), Some("en".to_string()));
    }
}
