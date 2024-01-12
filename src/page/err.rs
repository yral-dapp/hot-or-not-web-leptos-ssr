use leptos::*;
use leptos_router::*;

#[derive(Clone, Params, PartialEq)]
struct ServerErrParams {
    err: String,
}

#[component]
pub fn ServerErrorPage() -> impl IntoView {
    let params = use_query::<ServerErrParams>();
    let error = move || {
        params
            .get()
            .map(|p| p.err)
            .unwrap_or_else(|_| "Server Error".to_string())
    };

    view! {
        <div>
            <h1>Server Error</h1>
            <h3>{error}</h3>
        </div>
    }
}
