#[macro_export]
macro_rules! mount {
    ($app:ident) => {{
        ::leptos::mount_to_body(
            move || ::leptos::view! { <div id="wrapper"><$app/></div> },
        );
    }};
}

#[macro_export]
macro_rules! unmount {
    () => {{
        use wasm_bindgen::JsCast;
        ::leptos::document()
            .body()
            .unwrap()
            .remove_child(
                ::leptos::document()
                    .get_element_by_id("wrapper")
                    .unwrap()
                    .unchecked_ref(),
            )
            .unwrap();
    }};
}

pub async fn sleep(delay: i32) {
    let mut cb = |resolve: js_sys::Function, _reject: js_sys::Function| {
        ::web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, delay,
            )
            .unwrap();
    };

    let p = ::js_sys::Promise::new(&mut cb);
    ::wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
}

pub fn element_text(selector: &str) -> String {
    ::leptos::document()
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .text_content()
        .unwrap()
}

pub fn input_by_id(id: &str) -> web_sys::HtmlInputElement {
    use wasm_bindgen::JsCast;
    ::leptos::document()
        .get_element_by_id(id)
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
}

pub fn html() -> web_sys::HtmlHtmlElement {
    use wasm_bindgen::JsCast;
    ::leptos::document()
        .document_element()
        .unwrap()
        .unchecked_into::<web_sys::HtmlHtmlElement>()
}

pub mod localstorage {
    pub fn delete(key: &str) {
        ::leptos::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .remove_item(key)
            .unwrap();
    }

    pub fn set(key: &str, value: &str) {
        ::leptos::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(key, value)
            .unwrap();
    }

    pub fn get(key: &str) -> Option<String> {
        ::leptos::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(key)
            .unwrap()
    }
}
