use leptos::{html::Iframe, *};
use leptos_use::{
    storage::use_local_storage, use_event_listener, use_window,
    utils::FromToStringCodec,
};
use reqwest::Url;

use crate::{
    consts::{self, ACCOUNT_CONNECTED_STORE, AUTH_URL},
    state::auth::{auth_state, types::SessionResponse},
};

#[component]
pub fn ConnectLogin() -> impl IntoView {
    let (_, write_account_connected, _) =
        use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
    let logging_in = create_rw_signal(false);
    let auth = auth_state().identity;

    let iframe_ref = create_node_ref::<Iframe>();

    create_effect(move |_| {
        if auth.with(|a| a.is_none()) {
            return;
        }
        _ = use_event_listener(use_window(), ev::message, move |msg| {
            if Url::parse(&msg.origin())
                .map(|u| u.origin() != consts::AUTH_URL.origin())
                .unwrap_or_default()
            {
                return;
            }
            let data = msg.data().as_string().unwrap();
            let res: SessionResponse = serde_json::from_str(&data).unwrap();
            let identity = res.delegation_identity;
            auth.set(Some(identity));
            logging_in.set(false);
            write_account_connected.set(true);
        });
    });

    _ = use_event_listener(iframe_ref, ev::load, move |_| {
        let iframe_node = iframe_ref().unwrap();
        let iframe_w = iframe_node.content_window().unwrap();
        _ = iframe_w.post_message(&("login".into()), "*");
    });

    view! {
        <button
            on:click=move |ev| {
                ev.prevent_default();
                logging_in.set(true);
            }

            class="font-bold rounded-full bg-primary-600 py-2 md:py-3 w-full text-center text-lg md:text-xl text-white"
            disabled=move || logging_in() || auth.with(|a| a.is_none())
        >
            {move || if logging_in() { "Connecting..." } else { "Login" }}

        </button>
        <Show when=logging_in>
            <iframe _ref=iframe_ref class="hidden w-0 h-0" src=AUTH_URL.join("auth_init").unwrap().to_string() />
        </Show>
    }
}
