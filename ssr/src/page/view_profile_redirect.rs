use crate::component::{canisters_prov::AuthCansProvider, spinner::FullScreenSpinner};
use leptos::*;
use leptos_router::Redirect;

#[component]
fn ProfileLoading() -> impl IntoView {
    view! {
        <div class="rounded-full animate-pulse basis-4/12 aspect-square overflow-clip bg-white/20"></div>
        <div class="flex flex-col gap-2 animate-pulse basis-8/12">
            <div class="w-full h-4 rounded-full bg-white/20"></div>
            <div class="w-full h-4 rounded-full bg-white/20"></div>
        </div>
    }
}

#[component]
pub fn ProfileInfo() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canisters>
                <Redirect path=canisters.user_principal() />
        </AuthCansProvider>
    }
}
