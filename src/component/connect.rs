use leptos::*;

use crate::state::auth::auth_state;

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
        use crate::utils::event_streaming::events::LoginJoinOverlayViewed;
        LoginJoinOverlayViewed.send_event();
    }

    let login_click_action = create_action(move |()| async move {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            use crate::utils::event_streaming::events::LoginCta;
            LoginCta.send_event(cta_location.to_string());
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
