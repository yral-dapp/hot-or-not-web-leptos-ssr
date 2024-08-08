use candid::Principal;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{
    canister::sns_governance::GetMetadataArg,
    component::{back_btn::BackButton, spinner::FullScreenSpinner, title::Title},
    state::canisters::{unauth_canisters, Canisters},
};

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    user_id: Principal,
    id: usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct TokenMetadata {
    user_id: Principal,
    logo_b64: String,
    name: String,
    description: String,
    symbol: String,
}

async fn get_token_metadata(
    cans: Canisters<false>,
    user_id: Principal,
    id: usize,
) -> Result<Option<TokenMetadata>, ServerFnError> {
    let Some(user_can) = cans
        .get_individual_canister_by_user_principal(user_id)
        .await?
    else {
        return Ok(None);
    };
    let user = cans.individual_user(user_can).await?;
    let tokens = user.deployed_cdao_canisters().await?;
    let Some(token_cans) = tokens.get(id) else {
        return Ok(None);
    };

    let governance = cans.sns_governance(token_cans.governance).await?;
    let metadata = governance.get_metadata(GetMetadataArg {}).await?;

    let ledger = cans.sns_ledger(token_cans.ledger).await?;
    let symbol = ledger.icrc_1_symbol().await?;

    Ok(Some(TokenMetadata {
        user_id,
        logo_b64: metadata.logo.unwrap_or_default(),
        name: metadata.name.unwrap_or_default(),
        description: metadata.description.unwrap_or_default(),
        symbol,
    }))
}

#[component]
fn TokenField(#[prop(into)] label: String, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1">
            <span class="font-semibold text-white/40">{label}</span>
            <p class="bg-white/10">{value}</p>
        </div>
    }
}

#[component]
fn TokenInfoInner(meta: TokenMetadata) -> impl IntoView {
    let fallback_url = format!("/profile/{}", meta.user_id);
    view! {
        <div class="w-dvw min-h-dvh bg-black pt-4 flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback=fallback_url/>
                    <span class="font-bold justify-self-center">Token details</span>
                </div>
            </Title>
            <div class="flex flex-col w-full px-6 md:px-8 gap-6 md:gap-8">
                <div class="h-20 w-20 md:w-36 md:h-36 rounded-full">
                    <img class="object-contain h-full w-full rounded-full" src=meta.logo_b64/>
                </div>
                <TokenField label="Name" value=meta.name/>
                <TokenField label="Description" value=meta.description/>
                <TokenField label="Symbol" value=meta.symbol/>
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
        get_token_metadata(cans, params.user_id, params.id).await
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                token_info()
                    .map(|res| {
                        match res {
                            Err(e) => view! { <Redirect path=format!("/error?err={e}")/> },
                            Ok(None) => view! { <Redirect path="/"/> },
                            Ok(Some(meta)) => view! { <TokenInfoInner meta/> },
                        }
                    })
            }}

        </Suspense>
    }
}
