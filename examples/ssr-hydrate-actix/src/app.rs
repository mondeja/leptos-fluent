use fluent_templates::static_loader;
use leptos::*;
use leptos_fluent::{expect_i18n, leptos_fluent, move_tr, tr, Language};
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
    provide_meta_context();
    leptos_fluent! {{
        translations: [TRANSLATIONS],
        locales: "./locales",
        check_translations: "./src/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
        initial_language_from_cookie: true,
        initial_language_from_cookie_to_localstorage: true,
        initial_language_from_cookie_to_server_function: set_language_server_function,
        set_language_to_cookie: true,
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_localstorage: true,
        initial_language_from_url_param_to_cookie: true,
        set_language_to_url_param: true,
        localstorage_key: "language",
        initial_language_from_localstorage: true,
        initial_language_from_localstorage_to_cookie: true,
        initial_language_from_localstorage_to_server_function: set_language_server_function,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_cookie: true,
        initial_language_from_navigator_to_server_function: set_language_server_function,
        initial_language_from_accept_language_header: true,
        initial_language_from_server_function: initial_language_server_function,
        set_language_to_server_function: set_language_server_function,
    }};

    view! {
        <Title text=move || tr!("welcome-to-leptos")/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let i18n = expect_i18n();

    view! {
        <h1>{move_tr!("welcome-to-leptos")}</h1>
        <fieldset>
            {move || {
                i18n.languages.iter().map(|lang| render_language(lang)).collect::<Vec<_>>()
            }}

        </fieldset>
    }
}

fn render_language(lang: &'static Language) -> impl IntoView {
    // Passed as atrribute, `Language` is converted to their code,
    // so `<input id=lang` becomes `<input id=lang.id.to_string()`

    // Call on click to server action with a client-side translated
    // "hello-world" message
    view! {
        <div>
            <input
                id=lang
                name="language"
                type="radio"
                value=lang
                checked=lang.is_active()
                on:click=move |_| {
                    lang.activate();
                    spawn_local(async {
                        _ = show_hello_world(
                                tr!("hello-world"),
                                tr!("language", { "lang" => lang.name.to_string() }),
                            )
                            .await;
                    });
                }
            />

            <label for=lang>{lang.name}</label>
        </div>
    }
}

/// Server function to set the initial language
#[server(InitialLanguage, "/api")]
pub async fn initial_language_server_function(
) -> Result<Option<String>, ServerFnError> {
    // .. replace with your own logic
    Ok(Some("es".to_string()))
}

/// Server function to update the current language
#[server(SetLanguage, "/api")]
pub async fn set_language_server_function(
    _language: String,
) -> Result<(), ServerFnError> {
    // .. replace with your own logic
    Ok(())
}

/// Server action showing client-side translated message on console
#[server(ShowHelloWorld, "/api")]
pub async fn show_hello_world(
    translated_hello_world: String,
    language: String,
) -> Result<(), ServerFnError> {
    #[allow(clippy::print_stdout)]
    {
        println!("{} ({})", translated_hello_world, language);
    };
    Ok(())
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>{move_tr!("not-found")}</h1> }
}
