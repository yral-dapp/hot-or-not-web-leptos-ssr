use crate::component::canisters_prov::AuthCansProvider;
use crate::utils::profile::ProfileDetails;
use leptos::*;
use leptos_router::Redirect;

#[component]
fn ProfileLoading() -> impl IntoView {
    view! {
        <div class="basis-4/12 aspect-square overflow-clip rounded-full bg-white/20 animate-pulse"></div>
        <div class="basis-8/12 flex flex-col gap-2 animate-pulse">
            <div class="w-full h-4 bg-white/20 rounded-full"></div>
            <div class="w-full h-4 bg-white/20 rounded-full"></div>
        </div>
    }
}

#[component]
pub fn ProfileInfo() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=ProfileLoading let:canisters>
            <ProfileRedirect user_details=canisters.profile_details() />
        </AuthCansProvider>
    }
}
#[component]
pub fn ProfileRedirect(user_details: ProfileDetails) -> impl IntoView {
    // Ensure the method is called on an Option<String>
    let url = user_details.username_or_principal();
    view! {
        <Redirect path=url />
    }
}
