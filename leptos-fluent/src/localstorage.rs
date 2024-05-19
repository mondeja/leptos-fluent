use cfg_if::cfg_if;

pub fn get(#[allow(unused_variables)] key: &str) -> Option<String> {
    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        ::leptos::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item(key)
            .unwrap()
    } else {
        None
    }}
}

pub fn set(
    #[allow(unused_variables)] key: &str,
    #[allow(unused_variables)] value: &str,
) {
    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        ::leptos::window()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item(key, value)
            .unwrap()
    }}
}
