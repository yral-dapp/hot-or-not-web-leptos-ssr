use leptos::*;
use stylers::{style, style_sheet};

#[component]
pub fn SomeComponent() -> impl IntoView {
    // let styles = style_sheet!("./style.css");
    let styles = style! {
        h2 {
            color: blue;
        }
    };

    view! { class=styles, <h2>"Hello World"</h2> }
}

