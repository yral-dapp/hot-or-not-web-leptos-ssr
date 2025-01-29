use super::auth_providers::LoginProviders;
use crate::component::shared::assets::overlay::{ShadowOverlay, ShowOverlay};
use leptos::*;

#[component]
pub fn LoginModal(#[prop(into)] show: RwSignal<bool>) -> impl IntoView {
    let lock_closing = create_rw_signal(false);
    view! {
        <ShadowOverlay show=ShowOverlay::MaybeClosable {
            show,
            closable: lock_closing,
        }>
            <LoginProviders show_modal=show lock_closing />
        </ShadowOverlay>
    }
}
