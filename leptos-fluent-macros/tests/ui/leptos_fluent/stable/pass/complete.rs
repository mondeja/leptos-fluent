use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent_macros::leptos_fluent;

static_loader! {
    pub static TRANSLATIONS = {
        locales: "../../../../examples/csr-minimal/locales",
        fallback_language: "en",
    };
}

#[component]
fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS],
        locales: "../../../../examples/csr-minimal/locales",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; Path=/; Max-Age=600",
        set_language_to_cookie: true,
        initial_language_from_cookie: true,
        initial_language_from_cookie_to_local_storage: true,
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_local_storage: true,
        initial_language_from_url_param_to_cookie: true,
        set_language_to_url_param: true,
        local_storage_key: "language",
        initial_language_from_local_storage: true,
        initial_language_from_local_storage_to_cookie: true,
        set_language_to_local_storage: true,
        initial_language_from_navigator: true,
        customise: |bundle| bundle.set_transform(Some(|s| Cow::from(s)))
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18n>
            <p></p>
        </I18n>
    }
}

fn main() {}
