use leptos::*;
use std::time::Duration;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    mount_to_body(|| view! { <p id="foo">hello</p> });
    set_timeout(
        move || {
            let p = document().get_element_by_id("foo").unwrap();
            assert_eq!(p.text_content().unwrap(), "hello");
        },
        Duration::from_millis(100),
    );
}
