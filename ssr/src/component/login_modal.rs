use super::{
    auth_providers::LoginProviders,
    overlay::{ShadowOverlay, ShowOverlay},
};
use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use web_sys::window;
use yral_canisters_common::Canisters;

#[component]
pub fn LoginModal(#[prop(into)] show: RwSignal<bool>) -> impl IntoView {
    let lock_closing = create_rw_signal(false);
    let navigate = use_navigate();

    // Create an effect to handle navigation after successful login
    create_effect(move |_| {
        if !show() {
            // Small delay to ensure auth state is updated
            let value = navigate.clone();
            set_timeout(
                move || {
                    if let Some(canisters) = use_context::<RwSignal<Option<Canisters<true>>>>()
                        .and_then(|s| s.get_untracked())
                    {
                        let user_principal = canisters.user_principal();
                        if let Some(window) = window() {
                            let _ = window.location().set_href(
                                &format!("/profile/{}/tokens", user_principal)
                            );
                        } else {
                            value(
                                &format!("/profile/{}/tokens", user_principal),
                                NavigateOptions::default(),
                            );
                        }
                    }
                },
                std::time::Duration::from_millis(100),
            );
        }
    });

    view! {
        <ShadowOverlay show=ShowOverlay::MaybeClosable {
            show,
            closable: lock_closing,
        }>
            <LoginProviders show_modal=show lock_closing />
        </ShadowOverlay>
    }
}
