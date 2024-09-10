use super::{auth_providers::LoginProviders, overlay::ShadowOverlay};
use leptos::*;

#[component]
pub fn LoginModal(#[prop(into)] show: RwSignal<bool>) -> impl IntoView {
    let lock_closing = create_rw_signal(false);
    view! {
        <ShadowOverlay show lock_closing>
            <LoginProviders show_modal=show lock_closing/>
        </ShadowOverlay>
    }
}