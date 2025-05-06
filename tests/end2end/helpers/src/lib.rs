pub struct Mounter;

impl Drop for Mounter {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;
        ::leptos::prelude::document()
            .body()
            .unwrap()
            .remove_child(
                ::leptos::prelude::document()
                    .get_element_by_id("wrapper")
                    .unwrap()
                    .unchecked_ref(),
            )
            .unwrap();
    }
}

/// Mounts the app to the body of the document in a wrapper container.
#[macro_export]
macro_rules! mount {
    ($app:ident) => {
        let ___app_mounter = $crate::Mounter {};
        ::leptos::mount::mount_to_body(
            move || ::leptos::view! { <div id="wrapper"><$app/></div> },
        );
    };
}

/// Wait a moment for the DOM to update.
pub async fn sleep_a_moment() {
    gloo_timers::future::sleep(std::time::Duration::from_millis(30)).await;
}

pub fn element_text(selector: &str) -> String {
    ::leptos::prelude::document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .text_content()
        .unwrap()
}

pub fn input_by_id(id: &str) -> web_sys::HtmlInputElement {
    use wasm_bindgen::JsCast;
    leptos::prelude::document()
        .get_element_by_id(id)
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
}

pub fn html() -> web_sys::HtmlHtmlElement {
    use wasm_bindgen::JsCast;
    leptos::prelude::document()
        .document_element()
        .unwrap()
        .unchecked_into::<web_sys::HtmlHtmlElement>()
}
