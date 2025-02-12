use crate::page::token::RootType;
use crate::page::token::TokenInfoParams;
use crate::page::wallet::airdrop::AirdropPage;
use crate::state::canisters::authenticated_canisters;

use crate::utils::token::icpump::get_token_timestamp_by_id;
use crate::utils::token::icpump::IcpumpTokenInfo;
use crate::{
    component::{back_btn::BackButton, share_popup::*, spinner::FullScreenSpinner, title::Title},
    page::wallet::transactions::Transactions,
    utils::web::copy_to_clipboard,
};
use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use yral_canisters_common::cursored_data::transaction::IndexOrLedger;
use yral_canisters_common::utils::token::TokenMetadata;
use yral_canisters_common::Canisters;

#[component]
fn TokenField(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(optional, default = false)] copy: bool,
) -> impl IntoView {
    let copy_payload = value.clone();
    let copy_clipboard = move |_| {
        copy_to_clipboard(&copy_payload);
    };
    view! {
        <div class="flex flex-col gap-1 w-full">
            <span class="text-white text-sm md:text-base">{label}</span>
            <div class="bg-white/5 text-base md:text-lg text-white/50 px-2 py-4 rounded-xl w-full flex justify-between">
                <div>{value}</div>
                <Show when=move || copy>
                    <button on:click=copy_clipboard.clone()>
                        <Icon
                            class="w-6 h-6 text-white/50 cursor-pointer hover:text-white/80"
                            icon=icondata::BiCopyRegular
                        />
                    </button>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn TokenDetails(meta: TokenMetadata) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full gap-6 p-4 rounded-xl bg-white/5">
            <TokenField label="Ledger Id" value=meta.ledger.to_text() copy=true />
            <TokenField label="Description" value=meta.description />
            <TokenField label="Symbol" value=meta.symbol />
        </div>
    }
}

pub fn generate_share_link(root: &RootType, key_principal: Principal) -> String {
    format!("/token/info/{}/{key_principal}?airdrop_amt=100", root)
}

#[component]
fn TokenInfoInner(
    root: RootType,
    meta: TokenMetadata,
    key_principal: Option<Principal>,
    is_user_principal: bool,
) -> impl IntoView {
    let meta_c1 = meta.clone();
    let meta_c = meta.clone();
    let detail_toggle = create_rw_signal(false);
    let view_detail_icon = Signal::derive(move || {
        if detail_toggle() {
            icondata::AiUpOutlined
        } else {
            icondata::AiDownOutlined
        }
    });
    let share_link = key_principal.map(|key_principal| generate_share_link(&root, key_principal));
    let message = share_link.clone().map(|share_link|format!(
        "Hey! Check out the token: {} I created on YRAL 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        meta.symbol,  share_link
    ));

    let decimals = meta.decimals;
    let blur_active = create_rw_signal(meta.is_nsfw);

    view! {
        <div class="w-dvw min-h-dvh bg-neutral-800  flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback="/wallet" />
                    <span class="font-bold justify-self-center">Token details</span>
                </div>
            </Title>
            <div class="flex flex-col w-full items-center px-8 md:px-10 gap-8">
                <div class="flex flex-col justify-self-start w-full gap-6 md:gap-8 items-center">
                    <div class="flex flex-col gap-4 w-full bg-white/5 p-4 drop-shadow-lg rounded-xl">
                        <div class="flex flex-row justify-between items-center">
                            <div class="flex flex-row gap-2 items-center">
                                <div class="relative">
                                    <img
                                        class=move || format!("object-cover h-14 w-14 md:w-18 md:h-18 rounded-full cursor-pointer {}",
                                            if blur_active() { "blur-md" } else { "" }
                                        )
                                        src=meta.logo_b64
                                        on:click=move |_| {
                                            if meta.is_nsfw {
                                                blur_active.update(|b| *b = !*b);
                                            }
                                        }
                                    />
                                    <Show when=move || blur_active()>
                                        <div class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
                                            on:click=move |_| {
                                                if meta.is_nsfw {
                                                    blur_active.update(|b| *b = !*b);
                                                }
                                            }>
                                            <Icon
                                                class="w-6 h-6 text-white/80"
                                                icon=icondata::AiEyeInvisibleOutlined
                                            />
                                        </div>
                                    </Show>
                                </div>
                                <span class="text-base md:text-lg font-semibold text-white">
                                    {meta.name}
                                </span>
                            </div>
                            {share_link
                                .zip(message)
                                .map(|(share_link, message)| {
                                    view! {
                                        <ShareButtonWithFallbackPopup
                                            share_link
                                            message
                                            style="w-12 h-12".into()
                                        />
                                    }
                                })}
                        </div>

                        <Show when= move|| key_principal.clone().is_some()>
                            <div class="flex flex-row justify-between border-b p-1 border-white items-center">
                                <span class="text-xs md:text-sm text-green-500">Balance</span>
                                <span class="text-lg md:text-xl text-white">
                                    {meta
                                        .balance.clone()
                                        .map(|balance| {
                                            view! {
                                                <span class="font-bold">
                                                    {format!("{} ", balance.humanize_float_truncate_to_dp(2))}
                                                </span>
                                                <span>{meta_c1.symbol.clone()}</span>
                                    }
                                    })}
                                </span>
                            </div>
                        </Show>
                        <button
                            on:click=move |_| detail_toggle.update(|t| *t = !*t)
                            class="w-full bg-transparent p-1 flex flex-row justify-center items-center gap-2 text-white"
                        >
                            <span class="text-xs md:text-sm">View details</span>
                            <div class="p-1 bg-white/15 rounded-full">
                                <Icon class="text-xs md:text-sm text-white" icon=view_detail_icon />
                            </div>
                        </button>
                    </div>
                    <Show when=detail_toggle>
                        <TokenDetails meta=meta_c.clone() />
                    </Show>
                </div>
                    <Show when= move || is_user_principal>
                        <a
                            href=format!("/token/transfer/{}", root.to_string())
                            class="fixed bottom-20 left-4 right-4 p-3 bg-primary-600 text-white text-center md:text-lg rounded-full z-50"
                        >
                            Send
                        </a>
                    </Show>
                {if let Some(key_principal) = key_principal {
                    view! { <Transactions source=IndexOrLedger::Index { key_principal, index: meta.index } symbol=meta.symbol.clone() decimals/> }
                } else {
                    view! {
                        <Transactions
                            source=IndexOrLedger::Ledger(meta.ledger)
                            symbol=meta.symbol.clone()
                            decimals
                        />
                    }
                }}
            </div>
        </div>
    }
}

#[derive(Params, PartialEq, Clone, Serialize, Deserialize)]
pub struct TokenKeyParam {
    key_principal: Principal,
}

#[derive(Params, PartialEq, Clone, Serialize, Deserialize, Debug)]
struct AirdropParam {
    airdrop_amt: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct TokenInfoResponse {
    meta: TokenMetadata,
    root: RootType,
    #[serde(default)]
    key_principal: Option<Principal>,
    is_user_principal: bool,
    is_token_viewer_airdrop_claimed: bool,
}

#[component]
pub fn TokenInfo() -> impl IntoView {
    let params = use_params::<TokenInfoParams>();
    let key_principal = use_params::<TokenKeyParam>();
    let airdrop_param = use_query::<AirdropParam>();
    let key_principal = move || key_principal.with(|p| p.as_ref().map(|p| p.key_principal).ok());

    let token_metadata_fetch = authenticated_canisters().derive(
        move || (params(), key_principal()),
        move |cans_wire, (params_result, key_principal)| async move {
            let params = match params_result {
                Ok(p) => p,
                Err(_) => return Ok::<_, ServerFnError>(None),
            };

            let cans = Canisters::from_wire(cans_wire?, expect_context())?;

            let meta = cans
                .token_metadata_by_root_type(
                    &IcpumpTokenInfo,
                    key_principal,
                    params.token_root.clone(),
                )
                .await
                .ok()
                .flatten();

            let token_root = &params.token_root;
            let res = match (meta, token_root) {
                (Some(m), RootType::Other(root)) => {
                    let token_owner = m
                        .token_owner
                        .clone()
                        .ok_or(ServerFnError::new("Token owner not found for yral token"))?;
                    
                    let is_airdrop_claimed = cans
                        .get_airdrop_status(token_owner.canister_id, *root, cans.user_principal(), m.timestamp)
                        .await?;

                    Some(TokenInfoResponse {
                        meta: m,
                        root: token_root.clone(),
                        key_principal,
                        is_user_principal: Some(cans.user_principal()) == key_principal,
                        is_token_viewer_airdrop_claimed: is_airdrop_claimed,
                    })
                }
                (Some(m), _) => Some(TokenInfoResponse {
                    meta: m,
                    root: token_root.clone(),
                    key_principal,
                    is_user_principal: Some(cans.user_principal()) == key_principal,
                    is_token_viewer_airdrop_claimed: true,
                }),
                _ => None,
            };

            Ok(res)
        },
    );

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                token_metadata_fetch.get()
                    .map(|info| {
                        match info {
                            Ok(Some(TokenInfoResponse { meta, root, key_principal, is_user_principal, is_token_viewer_airdrop_claimed })) => {
                                if let Ok(AirdropParam { airdrop_amt }) = airdrop_param.get(){
                                    if !is_token_viewer_airdrop_claimed && meta.token_owner.clone().map(|t| t.principal_id) == key_principal && !is_user_principal{
                                        return view! {
                                            <AirdropPage airdrop_amount=airdrop_amt meta/>
                                        }
                                    }
                                }
                                view! {
                                    <TokenInfoInner
                                        root
                                        key_principal
                                        meta
                                        is_user_principal=is_user_principal
                                    />
                                }
                            }
                            _ => view! { <Redirect path="/wallet" /> },
                        }
                    })
            }}

        </Suspense>
    }
}
