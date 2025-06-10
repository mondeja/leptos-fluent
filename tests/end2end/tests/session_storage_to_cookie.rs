use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{cookie, leptos_fluent, session_storage};
use leptos_fluent_csr_minimal_example::LanguageSelector;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const COOKIE_NAME: &str = "my-weird-cookie-name";
const SESSION_STORAGE_KEY: &str = "my-weird-session-storage-key";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_session_storage: true,
        session_storage_key: SESSION_STORAGE_KEY,
        initial_language_from_session_storage_to_cookie: true,
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
pub async fn test_session_storage_to_cookie() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_session_storage_to_cookie
    cookie::delete(COOKIE_NAME);
    session_storage::delete(SESSION_STORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }

    session_storage::set(SESSION_STORAGE_KEY, "es");
    cookie::delete(COOKIE_NAME);
    {
        mount!(App);
        assert!(es().checked());
        assert_eq!(element_text("p"), "Selecciona un idioma:");
        assert_eq!(cookie::get(COOKIE_NAME), Some("es".to_string()));
    }

    session_storage::set(SESSION_STORAGE_KEY, "en");
    cookie::delete(COOKIE_NAME);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
        assert_eq!(cookie::get(COOKIE_NAME), Some("en".to_string()));
    }
}
