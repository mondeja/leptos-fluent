pub fn get(#[allow(unused_variables)] key: &str) -> Option<String> {
    #[cfg(not(feature = "ssr"))]
    return ::leptos::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item(key)
        .unwrap();
    #[cfg(feature = "ssr")]
    return None;
}

pub fn set(
    #[allow(unused_variables)] key: &str,
    #[allow(unused_variables)] value: &str,
) {
    #[cfg(not(feature = "ssr"))]
    ::leptos::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item(key, value)
        .unwrap()
}
