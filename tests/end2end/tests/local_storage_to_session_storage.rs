use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, local_storage, session_storage};
use leptos_fluent_csr_minimal_example::LanguageSelector;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const SESSIONSTORAGE_KEY: &str = "my-weird-session-storage-key";
const LOCALSTORAGE_KEY: &str = "my-weird-local-storage-key";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_local_storage: true,
        local_storage_key: LOCALSTORAGE_KEY,
        initial_language_from_local_storage_to_session_storage: true,
        session_storage_key: SESSIONSTORAGE_KEY,
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
pub async fn test_local_storage_to_session_storage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    // initial_language_from_local_storage_to_session_storage
    session_storage::delete(SESSIONSTORAGE_KEY);
    local_storage::delete(LOCALSTORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }

    local_storage::set(LOCALSTORAGE_KEY, "es");
    session_storage::delete(SESSIONSTORAGE_KEY);
    {
        mount!(App);
        assert!(es().checked());
        assert_eq!(element_text("p"), "Selecciona un idioma:");
        assert_eq!(
            session_storage::get(SESSIONSTORAGE_KEY),
            Some("es".to_string())
        );
    }

    local_storage::set(LOCALSTORAGE_KEY, "en");
    session_storage::delete(SESSIONSTORAGE_KEY);
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
        assert_eq!(
            session_storage::get(SESSIONSTORAGE_KEY),
            Some("en".to_string())
        );
    }
}
