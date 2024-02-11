pub fn get(key: &str) -> Option<String> {
    ::leptos::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item(key)
        .unwrap()
}

pub fn set(key: &str, value: &str) {
    ::leptos::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item(key, value)
        .unwrap()
}
