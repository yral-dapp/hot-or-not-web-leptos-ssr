use leptos::*;

use crate::utils::event_streaming::events::{LoginCta, LoginJoinOverlayViewed};

use super::login_modal::LoginModal;

#[component]
pub fn ConnectLogin(
    #[prop(optional, default = "Login")] login_text: &'static str,
    #[prop(optional, default = "menu")] cta_location: &'static str,
    #[prop(into)] show_login: RwSignal<bool>,
) -> impl IntoView {
    LoginJoinOverlayViewed.send_event();

    let login_click_action = create_action(move |()| async move {
        LoginCta.send_event(cta_location.to_string());
    });

    view! {
        <button
            on:click=move |ev| {
                ev.stop_propagation();
                show_login.set(true);
                login_click_action.dispatch(());
            }

            class="font-bold rounded-full bg-primary-600 py-2 md:py-3 w-full text-center text-lg md:text-xl text-white"
        >
            {move || if show_login() { "Connecting..." } else { login_text }}

        </button>
        <LoginModal show=show_login />
    }
}
