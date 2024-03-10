use cfg_if::cfg_if;

pub fn get(
    #[cfg_attr(feature = "ssr", allow(unused_variables))] k: &str,
) -> Option<String> {
    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        use leptos_router::Url;

        let query = ::leptos::window().location().search().unwrap();
        if !query.starts_with('?') {
            return None;
        }
        for (key, value) in Url::try_from(query.as_str()).unwrap().search_params.0 {
            if key != k {
                continue;
            }
            if value.is_empty() {
                return None;
            } else {
                return Some(value);
            }
        }
    }}
    None
}
