use http::status::StatusCode;
use leptos::prelude::*;
use leptos_fluent::{move_tr, tr};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => RwSignal::new(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    #[allow(clippy::print_stdout)]
    {
        println!("Errors: {errors:#?}");
    };

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());

            let header_name = axum::http::header::HeaderName::from_static(
                "some-localization",
            );
            let header_value = axum::http::header::HeaderValue::from_str(
                tr!("a-translated-header").as_str(),
            )
            .unwrap();
            response.insert_header(header_name, header_value);
        }
    }

    view! {
        <h1>
            {if errors.len() > 1 { tr!("some-errors-happened") } else { tr!("an-error-happened") }}
        </h1>
        <For
            // a function that returns the items we're iterating over; a signal is fine
            each=move || { errors.clone().into_iter().enumerate() }
            // a unique key for each item as a reference
            key=|(index, _error)| *index
            // renders each item to a view
            children=move |error| {
                let error_string = error.1.to_string();
                let error_code = error.1.status_code().to_string();
                view! {
                    <p>{move_tr!("error-msg", { "msg" => * error_string })}</p>
                    <p>{move_tr!("error-code", { "code" => error_code.clone() })}</p>
                }
            }
        />
    }
}
