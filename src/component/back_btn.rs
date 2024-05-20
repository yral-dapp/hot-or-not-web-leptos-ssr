use gloo::history::{BrowserHistory, History};
use leptos::{component, expect_context, view, window, Callback, IntoView};
use leptos_icons::Icon;

#[component]
pub fn BackButton(fallback: String) -> impl IntoView {
    let go_back = Callback::new(move |_| {
        let history = BrowserHistory::new();
        history.back();
    });

    view! {
        <button on:click=go_back class="items-center">
            <Icon icon=icondata::AiLeftOutlined/>
        </button>
    }
}
