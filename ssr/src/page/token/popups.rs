use leptos::*;
use leptos_icons::*;

use crate::{
    component::{overlay::PopupOverlay, token_confetti_symbol::TokenConfettiSymbol},
    page::token::create::CreateTokenCtx,
    state::canisters::auth_canisters_store,
};

#[component]
fn SuccessPopup(#[prop(into)] token_name: String, #[prop(into)] img_url: String) -> impl IntoView {
    CreateTokenCtx::reset();
    let cans = auth_canisters_store();
    let profile_url = move || {
        let Some(cans) = cans() else {
            return "/menu".into();
        };
        let profile_id = cans.user_principal();
        format!("/your-profile/{profile_id}?tab=tokens")
    };
    view! {
        <div class="flex flex-col items-center w-full h-full gap-6">
            // <TokenConfettiSymbol class="w-full"/>
            <img
                class="relative w-20 h-20 rounded-full border-2 border-primary-600 object-conver"
                style="height:15rem; width:15rem"
                 src=img_url
            />
            <span class="text-2xl md:text-3xl font-bold text-center">
                Token <span class="text-primary-600">{token_name}</span> successfully created!
            </span>
            <button
                on:click=move |_|navigate_to_profile( profile_url())
                class="w-3/4 py-4 text-lg text-center text-white bg-primary-600 rounded-full"
            >
                Back to profile
            </button>
        </div>
    }
}

fn navigate_to_profile(url: String) {
    let navigate = leptos_router::use_navigate();
    navigate(&url, Default::default());
}

#[component]
fn ErrorPopup(
    error: String,
    token_name: MaybeSignal<String>,
    close_popup: WriteSignal<bool>,
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
        <div class="flex flex-col items-center w-full h-full gap-6">
            <div class="flex flex-row items-center justify-center bg-amber-100 text-orange-400 rounded-full p-3 text-2xl md:text-3xl">
                <Icon icon=icondata::BsExclamationTriangle/>
            </div>
            <span class="text-2xl md:text-3xl font-bold text-center">
                Token <span class="text-primary-600">{token_name}</span> creation failed!
            </span>
            <textarea
                prop:value=error
                disabled
                rows=3
                class="bg-black/10 text-xs md:text-sm text-red-500 w-full md:w-2/3 resize-none p-2"
            />
            <button
                on:click=move |_| close_popup.set(true)
                class="py-3 text-lg md:text-xl w-full rounded-full bg-primary-600 text-white text-center"
            >
                Retry
            </button>
            <a href=profile_url class="py-3 text-lg md:text-xl w-full rounded-full text-black text-center bg-white border border-black">
                Back to profile
            </a>
        </div>
    }
}

#[component]
pub fn TokenCreationPopup(
    creation_action: Action<(), Result<(), String>>,
    #[prop(into)] token_name: MaybeSignal<String>,
    #[prop(into)] img_url: MaybeSignal<String>,
) -> impl IntoView {
    let close_popup = create_rw_signal(false);
    view! {
        <PopupOverlay
            action=creation_action
            loading_message="Token creation in progress"
            modal=move |res| match res {
                Ok(_) =>
                    view! {
                    <SuccessPopup img_url=img_url.get_untracked().clone() token_name=token_name.get_untracked().clone()/>
                },
                Err(e) => view! {
                    <ErrorPopup close_popup=close_popup.write_only() error=e token_name=token_name.clone()/>
                }
            }
            close=close_popup
        />
    }
}
