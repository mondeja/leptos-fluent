use cfg_if::cfg_if;

pub fn get(#[allow(unused_variables)] k: &str) -> Option<String> {
    cfg_if! { if #[cfg(not(feature = "ssr"))] {
        if let Ok(search) = leptos::window().location().search() {
            if let Ok(search_params) = web_sys::UrlSearchParams::new_with_str(&search) {
                return search_params.get(k);
            }
        }
    }};
    None
}
