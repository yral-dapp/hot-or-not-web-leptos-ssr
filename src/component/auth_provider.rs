use crate::{
    consts::AUTH_URL,
    state::auth::{auth_state, DelegationIdentity, SessionResponse},
};
use leptos::*;
use leptos_use::{use_event_listener, use_window};
use reqwest::Url;

#[component]
pub fn AuthFrame(auth: RwSignal<Option<DelegationIdentity>>) -> impl IntoView {
    _ = use_event_listener(use_window(), ev::message, move |m| {
        if Url::parse(&m.origin())
            .map(|u| u.origin() != AUTH_URL.origin())
            .unwrap_or_default()
        {
            return;
        }
        let data = m.data().as_string().unwrap();
        let res: SessionResponse = serde_json::from_str(&data).unwrap();
        let identity = res.delegation_identity;
        auth.set(Some(identity))
    });
    view! {
        <iframe
            class="h-0 w-0 hidden"
            src=AUTH_URL.join("/anonymous_identity").unwrap().to_string()
        ></iframe>
    }
}

#[component]
pub fn AuthProvider() -> impl IntoView {
    let auth = auth_state().identity;
    view! {
        <Show when=move || auth.with(|a| a.is_none())>
            <AuthFrame auth/>
        </Show>
    }
}
