pub fn get(key: &str) -> Option<String> {
    #[cfg(not(feature = "ssr"))]
    return ::leptos::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item(key)
        .unwrap();

    #[cfg(feature = "ssr")]
    {
        _ = key;
        None
    }
}

pub fn set(key: &str, value: &str) {
    #[cfg(not(feature = "ssr"))]
    ::leptos::window()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item(key, value)
        .unwrap();

    #[cfg(feature = "ssr")]
    {
        _ = key;
        _ = value;
    };
}
