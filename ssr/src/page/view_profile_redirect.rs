use crate::component::shared::assets::spinner::FullScreenSpinner;
use crate::component::shared::components::canisters_prov::AuthCansProvider;
use leptos::*;
use leptos_router::Redirect;
#[component]
pub fn ProfileInfo() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canisters>
            <Redirect path=canisters.user_principal() />
        </AuthCansProvider>
    }
}
