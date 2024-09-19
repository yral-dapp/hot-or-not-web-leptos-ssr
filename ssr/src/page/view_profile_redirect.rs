use crate::component::{canisters_prov::AuthCansProvider, spinner::FullScreenSpinner};
use leptos::*;
use leptos_router::Redirect;
#[component]
pub fn ProfileInfo() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canisters>
            <Redirect path=canisters.user_principal()/>
        </AuthCansProvider>
    }
}
