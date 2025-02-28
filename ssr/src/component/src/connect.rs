use leptos::prelude::*;

use crate::buttons::HighlightedButton;
use utils::event_streaming::events::{LoginCta, LoginJoinOverlayViewed};

use super::login_modal::LoginModal;

#[component]
pub fn ConnectLogin(
    #[prop(optional, default = "Login")] login_text: &'static str,
    #[prop(optional, default = "menu")] cta_location: &'static str,
) -> impl IntoView {
    let show_login = RwSignal::new(false);

    LoginJoinOverlayViewed.send_event();

    let login_click_action = Action::new(move |()| async move {
        LoginCta.send_event(cta_location.to_string());
    });

    view! {
        <HighlightedButton
        classes="w-full".to_string()
        alt_style=false
        disabled=false
        on_click=move || {
            show_login.set(true);
            login_click_action.dispatch(());
        }
        >
            {move || if show_login() { "Connecting..." } else { login_text }}
        </HighlightedButton>
        <LoginModal show=show_login />
    }
}
