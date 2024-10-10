use crate::error_template::{AppError, ErrorTemplate};
use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::tr;
use leptos_meta::*;
use leptos_router::*;

static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-fluent-ssr-islands-axum-example.css"/>

        <Title text=tr!("welcome-to-leptos")/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Routes>
                <Route path="" view=BodyView>
                    <Route path="" view=home::View/>
                    <Route path="/page-2" view=page_2::View/>
                </Route>
            </Routes>
        </Router>
    }
}

pub fn provide_i18n_context() {
    leptos_fluent::leptos_fluent! {
        translations: [TRANSLATIONS],
        languages: "./locales/languages.json",
        locales: "./locales",
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
    };
}

#[component]
pub fn BodyView() -> impl IntoView {
    view! {
        <header::View></header::View>
        <main>
            <Outlet/>
        </main>
    }
}

mod header {
    use leptos::*;
    use leptos_fluent::move_tr;
    use leptos_router::A;

    #[component]
    pub fn View() -> impl IntoView {
        view! {
            <header>
                <Archipelago>
                    <LargeMenu/>
                    <MobileMenu/>
                </Archipelago>
            </header>
        }
    }

    #[island]
    fn Archipelago(children: Children) -> impl IntoView {
        leptos_fluent::leptos_fluent! {
            translations: [super::TRANSLATIONS],
            languages: "./locales/languages.json",
            locales: "./locales",
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
        };
        children()
    }

    #[component]
    pub fn LargeMenu() -> impl IntoView {
        view! {
            <div class="header-large-menu">
                <A href="/">{move_tr!("home")}</A>
                <A href="/page-2">{move_tr!("page-2")}</A>
                <LanguageSelector/>
            </div>
        }
    }

    #[component]
    pub fn MobileMenu() -> impl IntoView {
        view! {
            <div class="header-mobile-menu">
                <MobileMenuButton>
                    <MobileMenuPanel>
                        <LanguageSelector/>
                    </MobileMenuPanel>
                </MobileMenuButton>
            </div>
        }
    }

    #[island]
    pub fn MobileMenuButton(children: Children) -> impl IntoView {
        let (is_mobile_menu_visible, set_mobile_menu_visibility) =
            create_signal(false);
        provide_context(is_mobile_menu_visible);

        view! {
            <button
                type="button"
                class="mobile-button"
                on:click=move |_| {
                    set_mobile_menu_visibility.update(|is_visible| *is_visible = !*is_visible);
                }
            >

                <svg
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="black"
                    aria-hidden="true"
                    style="width: 30px; height: 30px"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                    ></path>
                </svg>
            </button>
            <div
                class="mobile-menu-panel-hidder"
                class:hidden=move || !is_mobile_menu_visible.get()
                on:click=move |_| {
                    set_mobile_menu_visibility.set(false);
                }
            >
            </div>
            {children()}
        }
    }

    #[island]
    pub fn MobileMenuPanel(children: Children) -> impl IntoView {
        let is_mobile_menu_visible = expect_context::<ReadSignal<bool>>();

        view! {
            <div class="mobile-menu-panel" class:hidden=move || !is_mobile_menu_visible.get()>
                <a href="/">{move_tr!("home")}</a>
                <a href="/page-2">{move_tr!("page-2")}</a>
                {children()}
            </div>
        }
    }

    #[island]
    fn LanguageSelector() -> impl IntoView {
        let i18n = leptos_fluent::expect_i18n();

        // A page reload is necessary after changing the language because the translations are stored on the server.
        // This ensures that all content is updated to reflect the selected language. For more details, refer to the README.md.
        view! {
            <div class="language-selector">
                {move || {
                    i18n.languages
                        .iter()
                        .map(|lang| {
                            view! {
                                <div>
                                    <input
                                        type="radio"
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

                                    <label>{lang.name}</label>
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
    pub fn View() -> impl IntoView {
        view! {
            <h1>{move_tr!("home-title")}</h1>
            <HomeCounter>{move_tr!("click-me").get()}</HomeCounter>
        }
    }

    #[island]
    fn HomeCounter(children: Children) -> impl IntoView {
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
            <Page2Counter>{move_tr!("click-me").get()}</Page2Counter>
        }
    }

    #[island]
    fn Page2Counter(children: Children) -> impl IntoView {
        // Creates a reactive value to update the button
        let (count, set_count) = create_signal(0);
        let on_click = move |_| set_count.update(|count| *count += 1);

        view! { <button on:click=on_click>{children()} " - " {count}</button> }
    }
}
