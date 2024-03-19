use leptos::{component, expect_context, view, Callback, IntoView};
use leptos_icons::Icon;
use leptos_router::use_navigate;

use crate::state::history::HistoryCtx;

#[component]
pub fn BackButton(fallback: String) -> impl IntoView {
    let history_ctx = expect_context::<HistoryCtx>();

    let go_back = Callback::new(move |_| {
        let back_url = history_ctx.back(&fallback);
        use_navigate()(&back_url, Default::default());
    });

    view! {
        <button on:click=go_back class="items-center">
            <Icon icon=icondata::AiLeftOutlined/>
        </button>
    }
}
