use leptos::*;
use serde_json::json;

use crate::{
    state::{auth::auth_state, canisters::AuthProfileCanisterResource},
    utils::event_streaming::EventHistory,
};

use super::login_modal::LoginModal;

#[component]
pub fn ConnectLogin(
    #[prop(optional, default = "Login")] login_text: &'static str,
    #[prop(optional, default = "menu")] cta_location: &'static str,
) -> impl IntoView {
    let auth = auth_state();
    let show_login = create_rw_signal(false);

    #[cfg(all(feature = "hydrate", feature = "ga4"))]
    {
        use crate::utils::event_streaming::send_event;

        let event_history: EventHistory = expect_context();
        let profile_and_canister_details: AuthProfileCanisterResource = expect_context();
        let user_id = move || {
            profile_and_canister_details()
                .flatten()
                .map(|(q, _)| q.principal)
        };

        create_effect(move |_| {
            send_event(
                "login_join_overlay_viewed",
                &json!({
                    "user_id_viewer": user_id(),
                    "previous_event": event_history.event_name.get(),
                }),
            );
        });
    }

    let login_click_action = create_action(move |()| async move {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            use crate::utils::event_streaming::send_event;

            let event_history: EventHistory = expect_context();

            send_event(
                "login_cta",
                &json!({
                    "previous_event": event_history.event_name.get(),
                    "cta_location": cta_location,
                }),
            );
        }
    });

    view! {
        <button
            on:click=move |ev| {
                ev.stop_propagation();
                show_login.set(true);
                login_click_action.dispatch(());
            }

            class="font-bold rounded-full bg-primary-600 py-2 md:py-3 w-full text-center text-lg md:text-xl text-white"
            disabled=move || auth.identity.with(|a| a.is_none())
        >
            {move || if show_login() { "Connecting..." } else { login_text }}

        </button>
        <LoginModal show=show_login/>
    }
}
