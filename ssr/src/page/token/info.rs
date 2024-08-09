use candid::Principal;
use leptos::*;
use leptos_router::*;

use crate::{
    canister::sns_root::ListSnsCanistersArg,
    component::{back_btn::BackButton, spinner::FullScreenSpinner, title::Title},
    state::canisters::{unauth_canisters, Canisters},
    utils::token::{get_token_metadata, TokenMetadata},
};

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: Principal,
}

async fn fetch_token_metadata(
    cans: Canisters<false>,
    token_root: Principal,
) -> Result<Option<TokenMetadata>, ServerFnError> {
    let root = cans.sns_root(token_root).await?;
    let sns_cans = root.list_sns_canisters(ListSnsCanistersArg {}).await?;
    let Some(governance) = sns_cans.governance else {
        return Ok(None);
    };
    let Some(ledger) = sns_cans.ledger else {
        return Ok(None);
    };

    Ok(Some(get_token_metadata(&cans, governance, ledger).await?))
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

    let token_info = create_resource(params, |params| async move {
        let Ok(params) = params else {
            return Ok(None);
        };
        let cans = unauth_canisters();
        fetch_token_metadata(cans, params.token_root).await
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
        {move || token_info().map(|res| {
            match res {
                Err(e) => view! { <Redirect path=format!("/error?err={e}") /> },
                Ok(None) => view! { <Redirect path="/" /> },
                Ok(Some(meta)) => view! { <TokenInfoInner meta /> },
            }
        })}
        </Suspense>
    }
}
