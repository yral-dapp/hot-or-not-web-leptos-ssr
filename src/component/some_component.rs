use leptos::*;
use stylers::style;

#[component]
pub fn SomeComponent() -> impl IntoView {
    // let styles = style_sheet!("./style.css");
    let styles = style! {
        h2 {
            color: var(--blue-6);
            background-color: var(--green-6);
        }
    };

    view! { class=styles, <h2>"Hello World"</h2> }
}

