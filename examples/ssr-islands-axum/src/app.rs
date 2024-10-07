use crate::error_template::{AppError, ErrorTemplate};
use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::tr;
use leptos_meta::*;
use leptos_router::*;

static_loader! {
    pub static COMPONENTS_TRANSLATIONS = {
        locales: "./locales/server",
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

    i18n!([COMPONENTS_TRANSLATIONS], "./locales/server");

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-fluent-ssr-hydrate-axum-islands-example.css" />

        <Title text=tr!("welcome-to-leptos") />

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <Routes>
                <Route path="" view=BodyView>
                    <Route path="" view=home::View />
                    <Route path="/page-2" view=page_2::View />
                </Route>
            </Routes>
        </Router>
    }
}

#[component]
pub fn BodyView() -> impl IntoView {
    view! {
        <header::View></header::View>
        <main>
            <Outlet />
        </main>
    }
}

mod header {
    use fluent_templates::static_loader;
    use leptos::*;
    use leptos_fluent::{expect_i18n, move_tr};
    use leptos_router::A;

    static_loader! {
        static TRANSLATIONS = {
            locales: "./locales/header",
            fallback_language: "en",
        };
    }

    #[component]
    pub fn View() -> impl IntoView {
        let i18n = expect_i18n();
        view! {
            <header>
                <A href="/">{move_tr!(i18n, "home")}</A>
                <LanguageSelector />
                <A href="/page-2">{move_tr!(i18n, "page-2")}</A>
            </header>
        }
    }

    #[island]
    fn LanguageSelector() -> impl IntoView {
        let i18n = i18n!([TRANSLATIONS], "./locales/header");
        view! {
            <div style="display: inline-flex; margin-left: 10px">
                {move_tr!(i18n, "select-language")} ": "
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
                                        on:click=move |_| i18n.language.set(lang)
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
    use fluent_templates::static_loader;
    use leptos::*;
    use leptos_fluent::move_tr;

    static_loader! {
        static TRANSLATIONS = {
            locales: "./locales/home",
            fallback_language: "en",
        };
    }

    #[component]
    pub fn View() -> impl IntoView {
        let i18n =
            i18n!([crate::app::COMPONENTS_TRANSLATIONS], "./locales/server");
        view! {
            <h1>{move_tr!(i18n, "home-title")}</h1>
            <Archipelago>
                <Counter1 />
                <Counter2 />
            </Archipelago>
            <p>{move_tr!(i18n, "home-title")}</p>
            <Archipelago>
                <Counter3 />
            </Archipelago>
        }
    }

    #[island]
    fn Archipelago(children: Children) -> impl IntoView {
        i18n!([TRANSLATIONS], "./locales/home");

        // Children will be executed when the i18n context is ready.
        // See https://book.leptos.dev/islands.html#passing-context-between-islands
        //
        // > That’s why in `HomePage`, I made `let tabs = move ||` a function, and
        // > called it like `{tabs()}`: creating the tabs lazily this way meant that
        // > the `Tabs` island would already have provided the `selected` context by
        // > the time each `Tab` went looking for it.
        children()
    }

    #[island]
    fn Counter1() -> impl IntoView {
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{move_tr!("click-me")} " - " {count}</button> }
    }

    #[island]
    fn Counter2() -> impl IntoView {
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{move_tr!("click-me")} " - " {count}</button> }
    }

    #[island]
    fn Counter3() -> impl IntoView {
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{move_tr!("click-me")} " - " {count}</button> }
    }
}

mod page_2 {
    use fluent_templates::static_loader;
    use leptos::*;
    use leptos_fluent::move_tr;

    static_loader! {
        static TRANSLATIONS = {
            locales: "./locales/home",
            fallback_language: "en",
        };
    }

    #[component]
    pub fn View() -> impl IntoView {
        let i18n =
            i18n!([crate::app::COMPONENTS_TRANSLATIONS], "./locales/server");
        view! {
            <h1>{move_tr!(i18n, "page-2-title")}</h1>
            <Counter4 />
            <p>{move_tr!(i18n, "page-2-title")}</p>
        }
    }

    #[island]
    fn Counter4() -> impl IntoView {
        i18n!([TRANSLATIONS], "./locales/home");
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{move_tr!("click-me")} " - " {count}</button> }
    }
}
