use leptos::*;
use stylers::style;

#[component]
pub fn SomeComponent() -> impl IntoView {
    let styles = style! {
        h2 {
            color: var(--blue-6);
            background-color: var(--yellow-6);
            padding: var(--size-7);
        }
    };

    view! { class=styles, <h2>"Hello World"</h2> }
}

