use leptos::*;

use crate::component::buttons::HighlightedButton;
use crate::utils::event_streaming::events::{LoginCta, LoginJoinOverlayViewed};

use super::login_modal::LoginModal;

#[component]
pub fn ConnectLogin(
    #[prop(optional, default = "Login")] login_text: &'static str,
    #[prop(optional, default = "menu")] cta_location: &'static str,
    #[prop(optional, default = false.into())] show_login: RwSignal<bool>,
) -> impl IntoView {
    LoginJoinOverlayViewed.send_event();

    let login_click_action = create_action(move |()| async move {
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
