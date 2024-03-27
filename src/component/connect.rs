use leptos::*;

use crate::state::auth::auth_state;

use super::login_modal::LoginModal;

#[component]
pub fn ConnectLogin(
    #[prop(optional, default = "Login")] login_text: &'static str,
) -> impl IntoView {
    let auth = auth_state();
    let show_login = create_rw_signal(false);

    view! {
        <button
            on:click=move |ev| {
                ev.stop_propagation();
                show_login.set(true);
            }

            class="font-bold rounded-full bg-primary-600 py-2 md:py-3 w-full text-center text-lg md:text-xl text-white"
            disabled=move || auth.identity.with(|a| a.is_none())
        >
            {move || if show_login() { "Connecting..." } else { login_text }}

        </button>
        <LoginModal show=show_login/>
    }
}
