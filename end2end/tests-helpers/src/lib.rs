#[macro_export]
macro_rules! mount {
    ($app:ident) => {{
        ::leptos::mount::mount_to_body(
            move || ::leptos::view! { <div id="wrapper"><$app/></div> },
        );
    }};
}

#[macro_export]
macro_rules! unmount {
    () => {{
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
    }};
}

pub async fn sleep(delay: i32) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        ::leptos::prelude::window()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, delay,
            )
            .unwrap();
    };

    let p = ::js_sys::Promise::new(&mut cb);
    ::wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
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
