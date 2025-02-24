use gloo::history::{BrowserHistory, History};
use http::status::StatusCode;
use leptos::prelude::*;
use thiserror::Error;

#[cfg(feature = "ssr")]
use leptos_axum::ResponseOptions;

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

    let go_back = move || {
        let history = BrowserHistory::new();

        //go back
        history.back();
    };

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    println!("Errors: {errors:#?}");

    let error_string = if !errors.is_empty() {
        "It looks like our system is taking a coffee break. Try again in a bit, and we'll have it back to work!".to_string()
    } else {
        String::new()
    };

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    #[cfg(feature = "ssr")]
    {
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }

    view! {
        <div class="flex flex-col w-dvw h-dvh bg-black justify-center items-center">
            <img src="/img/error-logo.svg" />
            <h1 class="p-2 text-2xl md:text-3xl font-bold text-white">"oh no!"</h1>
            <div class="text-center text-xs md:text-sm text-white/60 w-full md:w-2/3 lg:w-1/3 resize-none px-8 mb-4">
                {error_string.clone()}
            </div>
            <button
                on:click=move |_| go_back()
                class="bg-primary-600 rounded-full mt-6 py-4 px-12 max-w-full text-white text-lg md:text-xl"
            >
                Go back
            </button>
        </div>
    }
}
