use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::{expect_i18n, leptos_fluent, tr, I18n};
use leptos_meta::*;
use leptos_router::{
    components::{Outlet, ParentRoute, Route, Router, Routes},
    path,
};

static_loader! {
    static SERVER_TRANSLATIONS = {
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

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-fluent-ssr-islands-axum-example.css" />

        <Title text=tr!("welcome-to-leptos") />

        <Router>
            <Routes fallback=|| "Page not found.".into_view()>
                <ParentRoute path=path!("") view=BodyView>
                    <Route path=path!("") view=home::View />
                    <Route path=path!("/page-2") view=page_2::View />
                </ParentRoute>
            </Routes>
        </Router>
    }
}

pub fn provide_i18n_context() {
    leptos_fluent! {
        translations: [SERVER_TRANSLATIONS],
        languages: "./locales/languages.json",
        locales: "./locales/server",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
        initial_language_from_cookie: true,
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_cookie: true,
        set_language_to_url_param: true,
        initial_language_from_accept_language_header: true,
    };
}

#[component]
pub fn BodyView() -> impl IntoView {
    let i18n = expect_i18n();

    // Reproviding the context after the header makes server
    // translations available for `Outlet`

    view! {
        <header::View />
        {provide_context::<I18n>(i18n)}
        <main>
            <Outlet />
        </main>
    }
}

mod header {
    use leptos::prelude::*;
    use leptos_fluent::{expect_i18n, leptos_fluent, tr};
    use leptos_router::components::A;

    #[component]
    pub fn View() -> impl IntoView {
        view! {
            <header>
                <Archipelago>
                    <LargeMenu />
                    <MobileMenu />
                </Archipelago>
            </header>
        }
    }

    #[island]
    fn Archipelago(children: Children) -> impl IntoView {
        leptos_fluent! {
            translations: [super::CLIENT_TRANSLATIONS],
            languages: "./locales/languages.json",
            locales: "./locales/client",
            sync_html_tag_lang: true,
            sync_html_tag_dir: true,
            cookie_name: "lang",
            cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
            initial_language_from_cookie: true,
            set_language_to_cookie: true,
            url_param: "lang",
            initial_language_from_url_param: true,
            initial_language_from_url_param_to_cookie: true,
            set_language_to_url_param: true,
        };

        // Children will be executed when the i18n context is ready.
        // See https://book.leptos.dev/islands.html#passing-context-between-islands
        //
        // > That’s why in `HomePage`, I made `let tabs = move ||` a function, and
        // > called it like `{tabs()}`: creating the tabs lazily this way meant that
        // > the `Tabs` island would already have provided the `selected` context by
        // > the time each `Tab` went looking for it.
        children()
    }

    #[component]
    pub fn LargeMenu() -> impl IntoView {
        view! {
            <div class="header-large-menu">
                <A href="/">{tr!("home")}</A>
                <A href="/page-2">{tr!("page-2")}</A>
                <LanguageSelector name="desktop".into() />
            </div>
        }
    }

    #[component]
    pub fn MobileMenu() -> impl IntoView {
        view! {
            <div class="header-mobile-menu">
                <MobileMenuButton>
                    <MobileMenuPanel>
                        <LanguageSelector name="mobile".into() />
                    </MobileMenuPanel>
                </MobileMenuButton>
            </div>
        }
    }

    #[island]
    pub fn MobileMenuButton(children: Children) -> impl IntoView {
        let (is_mobile_menu_visible, set_mobile_menu_visibility) =
            signal(false);
        provide_context(is_mobile_menu_visible);

        view! {
            <button
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
            ></div>
            {children()}
        }
    }

    #[island]
    pub fn MobileMenuPanel(children: Children) -> impl IntoView {
        let is_mobile_menu_visible = expect_context::<ReadSignal<bool>>();

        view! {
            <div class="mobile-menu-panel" class:hidden=move || !is_mobile_menu_visible.get()>
                <a href="/">{tr!("home")}</a>
                <a href="/page-2">{tr!("page-2")}</a>
                {children()}
            </div>
        }
    }

    #[island]
    fn LanguageSelector(name: String) -> impl IntoView {
        let i18n = expect_i18n();

        // A page reload is necessary after changing the language because
        // the translations are stored on the server. This ensures that all
        // content is updated to reflect the selected language.
        view! {
            <div class="language-selector">
                {i18n
                    .languages
                    .iter()
                    .map(|lang| {
                        view! {
                            <div>
                                <input
                                    id=format!("language-{}-{}", name, lang.name)
                                    type="radio"
                                    name=format!("language-{}", name)
                                    value=lang
                                    checked=lang.is_active()
                                    on:click=move |_| {
                                        i18n.language.set(lang);
                                        // window()
                                        //     .location()
                                        //     .reload()
                                        //     .expect("Failed to reload the page");
                                    }
                                />

                                <label for=format!("language-{}-{}", name, lang.name)>{lang.name}</label>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}

            </div>
        }
    }
}

mod home {
    use leptos::prelude::*;
    use leptos_fluent::tr;

    #[component]
    pub fn View() -> impl IntoView {
        view! {
            <h1>{tr!("home-title")}</h1>
            <HomeCounter>{tr!("click-me")}</HomeCounter>
        }
    }

    #[island]
    fn HomeCounter(children: Children) -> impl IntoView {
        let (count, set_count) = signal(0);
        let on_click = move |_| *set_count.write() += 1;

        view! { <button on:click=on_click>""{children()} " - " {count}</button> }
    }
}

mod page_2 {
    use leptos::prelude::*;
    use leptos_fluent::tr;

    #[component]
    pub fn View() -> impl IntoView {
        view! {
            <h1>{tr!("page-2-title")}</h1>
            <Page2Counter>{tr!("click-me")}</Page2Counter>
        }
    }

    #[island]
    fn Page2Counter(children: Children) -> impl IntoView {
        let (count, set_count) = signal(0);
        let on_click = move |_| *set_count.write() += 1;

        view! { <button on:click=on_click>""{children()} " - " {count}</button> }
    }
}
