use leptos::*;

use crate::{
    component::{
        overlay::ShadowOverlay, spinner::Spinner, token_confetti_symbol::TokenConfettiSymbol,
    },
    state::canisters::auth_canisters_store,
};

#[component]
pub fn SuccessPopup(
    #[prop(into)] show: Signal<bool>,
    #[prop(into)] token_name: Signal<String>,
) -> impl IntoView {
    let cans = auth_canisters_store();
    let profile_url = move || {
        let Some(cans) = cans() else {
            return "/menu".into();
        };
        let profile_id = cans.user_principal();
        format!("/your-profile/{profile_id}?tab=tokens")
    };

    view! {
        <ShadowOverlay show>
            <div class="flex flex-col items-center px-4 pt-4 pb-12 mx-6 w-full lg:w-1/2 max-h-[65%] rounded-xl bg-white gap-6">
                <TokenConfettiSymbol class="w-full"/>
                <span class="text-3xl font-bold text-center">
                    Token <span class="text-primary-600">{token_name}</span> successfully created!
                </span>
                <a
                    href=profile_url
                    class="w-3/4 py-4 text-lg text-center text-white bg-primary-600 rounded-full"
                >
                    Back to profile
                </a>
            </div>
        </ShadowOverlay>
    }
}

#[component]
pub fn TokenCreationPopup(#[prop(into)] show: Signal<bool>) -> impl IntoView {
    view! {
        <ShadowOverlay show>
            <div class="w-full h-full flex flex-col gap-6 items-center justify-center text-white text-center text-xl font-semibold">
                <Spinner/>
                <span>Token creation in progress</span>
                <span>Please wait...</span>
            </div>
        </ShadowOverlay>
    }
}
