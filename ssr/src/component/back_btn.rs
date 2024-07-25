use gloo::history::{BrowserHistory, History};
use leptos::*;
use leptos_icons::Icon;
use leptos_router::use_navigate;
use reqwest::Url;

use crate::state::history::HistoryCtx;

#[component]
pub fn BackButton(#[prop(into)] fallback: MaybeSignal<String>) -> impl IntoView {
    let history_ctx = expect_context::<HistoryCtx>();
    let go_back = Callback::new(move |_| {
        let win = window();
        let referrer = win
            .document()
            .map(|d| d.referrer())
            .and_then(|r| Url::parse(&r).ok());
        let cur_url = Url::parse(&win.location().href().unwrap_or_default()).ok();

        // HACK: completely remove history ctx eventually
        if cur_url.as_ref().and_then(|u| u.host_str())
            == referrer.as_ref().and_then(|r| r.host_str())
            || history_ctx.len() > 1
        {
            let history = BrowserHistory::new();
            history.back();
        } else {
            use_navigate()(&fallback.get_untracked(), Default::default());
        }
    });

    view! {
        <button on:click=go_back class="items-center">
            <Icon icon=icondata::AiLeftOutlined/>
        </button>
    }
}
