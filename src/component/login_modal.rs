use super::{auth_providers::LoginProviders, overlay::ShadowOverlay};
use leptos::*;

#[component]
pub fn LoginModal(#[prop(into)] show: RwSignal<bool>) -> impl IntoView {
    view! {
        <ShadowOverlay show>
            <div class="flex flex-col py-12 px-16 items-center gap-2 bg-neutral-900 text-white cursor-auto">
                <h1 class="text-xl">Login to Yral</h1>
                <img class="h-32 w-32 object-contain my-8" src="/img/logo.webp"/>
                <span class="text-md">Continue with</span>
                <LoginProviders show_modal=show/>
            </div>
        </ShadowOverlay>
    }
}
