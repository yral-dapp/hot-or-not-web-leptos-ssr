use leptos::*;
use stylers::style;

#[component]
pub fn SomeComponent() -> impl IntoView {
    let styles = style! {
        h2 {
            color: blue;
        }
    };

    view! { class=styles, <h2>"Hello World"</h2> }
}

