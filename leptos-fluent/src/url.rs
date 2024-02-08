use leptos::window;
use leptos_router::Url;

pub fn get(k: &str) -> Option<String> {
    let query = window().location().search().unwrap();
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
    None
}
