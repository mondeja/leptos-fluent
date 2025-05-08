use end2end_helpers::{element_text, input_by_id, mount};
use leptos::prelude::*;
use leptos_fluent::{leptos_fluent, sessionstorage};
use leptos_fluent_csr_minimal_example::{LanguageSelector, TRANSLATIONS};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const SESSIONSTORAGE_KEY: &str = "foobarbaz";

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "../../examples/csr-minimal/locales",
        initial_language_from_sessionstorage: true,
        sessionstorage_key: SESSIONSTORAGE_KEY,
        set_language_to_sessionstorage: true,
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
pub async fn initial_language_from_sessionstorage() {
    let es = move || input_by_id("es");
    let en = move || input_by_id("en");

    sessionstorage::set(SESSIONSTORAGE_KEY, "es");
    {
        mount!(App);
        assert!(es().checked());
        assert_eq!(element_text("p"), "Selecciona un idioma:");
    }

    sessionstorage::set(SESSIONSTORAGE_KEY, "en");
    {
        mount!(App);
        assert!(en().checked());
        assert_eq!(element_text("p"), "Select a language:");
    }
}
