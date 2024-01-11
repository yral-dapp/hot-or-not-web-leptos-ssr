use std::error::Error;

use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct ServerErrParams {
    err: String,
}

#[macro_export]
macro_rules! try_or_redirect {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => {
                use $crate::page::err::failure_redirect;
                failure_redirect(e);
                return;
            }
        }
    };
}

pub fn failure_redirect<E: Error>(err: E) {
    let nav = use_navigate();
    nav(&format!("/error?err={err}"), Default::default());
}

#[component]
pub fn ServerErrorPage() -> impl IntoView {
    let params = use_query::<ServerErrParams>();
    let error = move || {
        params.with(|p| {
            p.as_ref()
                .map(|p| p.err.clone())
                .unwrap_or("Server Error".to_string())
        })
    };
    view! {
        <div>
            <h1>Server Error</h1>
            <h3>{error}</h3>
        </div>
    }
}
