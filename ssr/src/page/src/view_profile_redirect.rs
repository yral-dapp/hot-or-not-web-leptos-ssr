use component::{canisters_prov::AuthCansProvider, spinner::FullScreenSpinner};
use leptos::prelude::*;
use leptos_router::components::Redirect;

#[component]
pub fn ProfileInfo() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canisters>
            <Redirect path=canisters.user_principal() />
        </AuthCansProvider>
    }
}
