#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! lib_rs_impl {
    () => {
        pub mod app;
        #[cfg(feature = "hydrate")]
        #[wasm_bindgen::prelude::wasm_bindgen]
        pub fn hydrate() {
            use crate::app::*;
            console_error_panic_hook::set_once();
            leptos::mount::hydrate_body(App);
        }
    };
}

#[macro_export]
macro_rules! shell {
    ($app:ident) => {
        pub fn shell(options: leptos::prelude::LeptosOptions) -> impl leptos::prelude::IntoView {
            #[allow(non_snake_case)]
            let HydrationScripts = leptos::prelude::HydrationScripts;
            #[allow(non_snake_case)]
            let MetaTags = leptos_meta::MetaTags;
            view! {
                <!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="utf-8" />
                        <meta name="viewport" content="width=device-width, initial-scale=1" />
                        <HydrationScripts options />
                        <MetaTags />
                    </head>
                    <body>
                        <$app/>
                    </body>
                </html>
            }
        }
    };
}

#[macro_export]
macro_rules! shell_and_app_impl {
    ($view:ident) => {
        $crate::shell!(App);

        #[leptos::component]
        pub fn App() -> impl leptos::prelude::IntoView {
            #[allow(non_snake_case)]
            let RoutedApp = $crate::RoutedApp;
            view! {
                <RoutedApp view=$view/>
            }
        }
    };
}

#[leptos::component]
pub fn RoutedApp(
    view: impl leptos_router::ChooseView + 'static,
) -> impl leptos::prelude::IntoView {
    use leptos::prelude::*;
    use leptos_router::{
        components::{Route, Router, Routes},
        StaticSegment,
    };

    // Provides context that manages stylesheets, titles, meta tags, etc.
    leptos_meta::provide_meta_context();

    view! {
        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view />
                </Routes>
            </main>
        </Router>
    }
}

pub mod axum {
    #[macro_export]
    macro_rules! axum_main_impl {
        ($app_path:ident) => {
            #[cfg(feature = "ssr")]
            #[tokio::main]
            async fn main() {
                use axum::Router;
                use leptos::logging::log;
                use leptos::prelude::*;
                use leptos_axum::{generate_route_list, LeptosRoutes};
                use $app_path::app::{shell, App};

                let conf = get_configuration(None).unwrap();
                let addr = conf.leptos_options.site_addr;
                let leptos_options = conf.leptos_options;
                // Generate the list of routes in your Leptos App
                let routes = generate_route_list(App);

                let app = Router::new()
                    .leptos_routes(&leptos_options, routes, {
                        let leptos_options = leptos_options.clone();
                        move || shell(leptos_options.clone())
                    })
                    .fallback(leptos_axum::file_and_error_handler(shell))
                    .with_state(leptos_options);

                // run our app with hyper
                // `axum::Server` is a re-export of `hyper::Server`
                log!("listening on http://{}", &addr);
                let listener =
                    tokio::net::TcpListener::bind(&addr).await.unwrap();
                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            }

            #[cfg(not(feature = "ssr"))]
            pub fn main() {
                // no client-side main function
                // unless we want this to work with e.g., Trunk for pure client-side testing
                // see lib.rs for hydration function instead
            }
        };
    }
}
