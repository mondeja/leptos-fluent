use crate::error_template::{AppError, ErrorTemplate};
use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::tr;
use leptos_meta::*;
use leptos_router::*;

static_loader! {
    pub static SERVER_TRANSLATIONS = {
        locales: "./locales/server",
        fallback_language: "en",
    };
}

static_loader! {
    static CLIENT_TRANSLATIONS = {
        locales: "./locales/client",
        fallback_language: "en",
    };
}

#[macro_export]
macro_rules! i18n {
    ($translations:expr, $locales:expr$(,)?) => {{
        ::leptos_fluent::leptos_fluent! {
            translations: $translations,
            languages: "./locales/languages.json",
            locales: $locales,
            sync_html_tag_lang: true,
            sync_html_tag_dir: true,
            cookie_name: "lang",
            cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
            initial_language_from_cookie: true,
            initial_language_from_cookie_to_localstorage: true,
            set_language_to_cookie: true,
            url_param: "lang",
            initial_language_from_url_param: true,
            initial_language_from_url_param_to_localstorage: true,
            initial_language_from_url_param_to_cookie: true,
            set_language_to_url_param: true,
            localstorage_key: "language",
            initial_language_from_localstorage: true,
            initial_language_from_localstorage_to_cookie: true,
            set_language_to_localstorage: true,
            initial_language_from_navigator: true,
            initial_language_from_navigator_to_localstorage: true,
            initial_language_from_accept_language_header: true,
        }
    }};
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    i18n!([SERVER_TRANSLATIONS], "./locales/server");

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-fluent-ssr-islands-axum-2-example.css"/>

        <Title text=tr!("welcome-to-leptos")/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Routes>
                <Route path="" view=BodyView>
                    <Route path="" view=home::HomeView/>
                    <Route path="/page-2" view=page_2::View/>
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
pub fn BodyView() -> impl IntoView {
    view! {
        <header::HeaderView></header::HeaderView>
        <main>
            <Outlet/>
        </main>
    }
}

mod header {
    use leptos::*;
    use leptos_fluent::{expect_i18n, move_tr};
    use leptos_router::A;

    #[component]
    pub fn HeaderView() -> impl IntoView {
        view! {
            <header>
                <A href="/">{move_tr!("home")}</A>
                <A href="/page-2">{move_tr!("page-2")}</A>
                <LanguageSelector/>
            </header>
        }
    }

    #[island]
    fn LanguageSelector() -> impl IntoView {
        i18n!([super::SERVER_TRANSLATIONS], "./locales/client");
        let i18n = expect_i18n();
        view! {
            <div style="display: inline-flex; margin-left: 10px">
                {move || {
                    i18n.languages
                        .iter()
                        .map(|lang| {
                            view! {
                                <div>
                                    <input
                                        type="radio"
                                        id=lang
                                        name="language"
                                        value=lang
                                        checked=lang.is_active()
                                        on:click=move |_| {
                                            i18n.language.set(lang);
                                            window()
                                                .location()
                                                .reload()
                                                .expect("Failed to reload the page");
                                        }
                                    />

                                    <label for=lang>{lang.name}</label>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()
                }}

            </div>
        }
    }
}

mod home {
    use leptos::*;
    use leptos_fluent::move_tr;

    #[component]
    pub fn HomeView() -> impl IntoView {
        view! {
            <h1>{move_tr!("home-title")}</h1>
            <Counter1>{move_tr!("click-me").get()}</Counter1>
            <Counter2>{move_tr!("click-me").get()}</Counter2>
            <p>{move_tr!("home-title")}</p>
            <Counter3>{move_tr!("click-me").get()}</Counter3>
        }
    }

    #[island]
    fn Counter1(children: Children) -> impl IntoView {
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{children()} " - " {count}</button> }
    }

    #[island]
    fn Counter2(children: Children) -> impl IntoView {
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{children()} " - " {count}</button> }
    }

    #[island]
    fn Counter3(children: Children) -> impl IntoView {
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{children()} " - " {count}</button> }
    }
}

mod page_2 {
    use leptos::*;
    use leptos_fluent::move_tr;

    #[component]
    pub fn View() -> impl IntoView {
        view! {
            <h1>{move_tr!("page-2-title")}</h1>
            <Counter4>{move_tr!("click-me").get()}</Counter4>
            <p>{move_tr!("page-2-title")}</p>
        }
    }

    #[island]
    fn Counter4(children: Children) -> impl IntoView {
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{children()} " - " {count}</button> }
    }
}
