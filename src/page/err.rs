use crate::{
    state::{canisters::auth_canisters_store, history::HistoryCtx},
    utils::event_streaming::events::ErrorEvent,
};
use leptos::*;
use leptos_router::*;

#[derive(Clone, Params, PartialEq)]
struct ServerErrParams {
    err: String,
}

#[component]
pub fn ServerErrorPage() -> impl IntoView {
    let params = use_query::<ServerErrParams>();
    let error = Signal::derive(move || {
        params
            .get()
            .map(|p| p.err)
            .unwrap_or_else(|_| "Server Error".to_string())
    });

    let canister_store = auth_canisters_store();
    ErrorEvent.send_event(error, canister_store);

    view! { <ErrorView error/> }
}

#[component]
pub fn ErrorView(#[prop(into)] error: MaybeSignal<String>) -> impl IntoView {
    let history_ctx = expect_context::<HistoryCtx>();
    let go_back = move || {
        let back_url = history_ctx.back("/");
        use_navigate()(&back_url, Default::default());
    };

    view! {
        <div class="flex flex-col p-10 w-dvw h-dvh bg-black items-center">
            <img src="/img/error-logo.svg"/>
            <h1 class="text-2xl md:text-3xl font-bold text-white mb-2">
                "Something went wrong :("
            </h1>
            <textarea
                prop:value=error
                disabled
                rows=3
                class="bg-white/10 text-xs md:text-sm text-white/60 w-full md:w-2/3 lg:w-1/3 resize-none p-2 mb-4"
            ></textarea>
            <button
                on:click=move |_| go_back()
                class="bg-primary-600 rounded-full py-3 px-12 max-w-full text-white text-lg md:text-xl"
            >
                Go Back
            </button>
        </div>
    }
}
