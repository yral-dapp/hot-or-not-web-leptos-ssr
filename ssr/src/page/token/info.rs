use candid::Principal;
use leptos::*;
use leptos_router::*;

use crate::{
    component::{
        back_btn::BackButton, canisters_prov::WithAuthCans, spinner::FullScreenSpinner,
        title::Title,
    },
    state::canisters::Canisters,
    utils::token::{token_metadata_by_root, TokenMetadata},
};

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: Principal,
}

#[component]
fn TokenField(#[prop(into)] label: String, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 w-full">
            <span class="font-semibold text-white/60">{label}</span>
            <p class="bg-white/30 text-white px-2 py-4 rounded-xl w-full">{value}</p>
        </div>
    }
}

#[component]
fn TokenInfoInner(meta: TokenMetadata) -> impl IntoView {
    let fallback_url = "/wallet".to_string();
    view! {
        <div class="w-dvw min-h-dvh bg-black pt-4 flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback=fallback_url/>
                    <span class="font-bold justify-self-center">Token details</span>
                </div>
            </Title>
            <div class="flex flex-col w-full px-6 md:px-8 gap-6 md:gap-8 items-center">
                <div class="h-20 w-20 md:w-36 md:h-36 rounded-full">
                    <img class="object-contain h-full w-full rounded-full" src=meta.logo_b64 />
                </div>
                <TokenField label="Name" value=meta.name />
                <TokenField label="Description" value=meta.description />
                <TokenField label="Symbol" value=meta.symbol />
            </div>
        </div>
    }
}

#[component]
pub fn TokenInfo() -> impl IntoView {
    let params = use_params::<TokenParams>();

    let token_metadata_fetch = move |cans: Canisters<true>| {
        create_resource(params, move |params| {
            let cans = cans.clone();
            async move {
                let Ok(params) = params else {
                    return Ok(None);
                };
                let user = cans.user_canister();
                token_metadata_by_root(&cans, user, params.token_root).await
            }
        })
    };

    view! {
        <WithAuthCans fallback=FullScreenSpinner with=token_metadata_fetch let:info>
        {match info.1 {
            Err(e) => view! { <Redirect path=format!("/error?err={e}") /> },
            Ok(None) => view! { <Redirect path="/" /> },
            Ok(Some(meta)) => view! { <TokenInfoInner meta /> },
        }}
        </WithAuthCans>
    }
}
