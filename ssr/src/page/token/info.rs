use crate::component::icons::airdrop_icon::AirdropIcon;
use crate::component::icons::arrow_left_right_icon::ArrowLeftRightIcon;
use crate::component::icons::chevron_right_icon::ChevronRightIcon;
use crate::component::icons::send_icon::SendIcon;
use crate::component::icons::share_icon::ShareIcon;
use crate::page::token::RootType;
use crate::page::token::TokenInfoParams;
use crate::page::wallet::airdrop::AirdropPage;
use crate::state::canisters::authenticated_canisters;

use crate::utils::token::icpump::IcpumpTokenInfo;
use crate::utils::web::share_url;
use crate::{
    component::{
        back_btn::BackButton, share_popup::*, spinner::FullScreenSpinner, title::TitleText,
    },
    page::wallet::transactions::Transactions,
    utils::web::copy_to_clipboard,
};
use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use yral_canisters_common::cursored_data::transaction::IndexOrLedger;
use yral_canisters_common::utils::token::TokenMetadata;
use yral_canisters_common::Canisters;

use crate::component::overlay::PopupOverlay;
use crate::utils::host::get_host;

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
pub fn ActionButton(
    href: String,
    label: String,
    children: Children,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
) -> impl IntoView {
    view! {
        <a
            aria-disabled=move || disabled().to_string()
            href=href
            class=move || {
                format!(
                    "flex flex-col gap-1 justify-center items-center text-xs transition-colors {}",
                    if !disabled.get() {
                        "group-hover:text-white text-neutral-300"
                    } else {
                        "text-neutral-600 pointer-events-none"
                    },
                )
            }
        >
            <div class="w-[1.125rem] h-[1.125rem] flex items-center justify-center">
                {children()}
            </div>

            <div class="text-[0.625rem] font-medium leading-4">{label}</div>
        </a>
    }
}

#[component]
pub fn ActionButtonLink(
    label: String,
    children: Children,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
) -> impl IntoView {
    view! {
        <button
            disabled=disabled
            class="flex flex-col gap-1 justify-center items-center text-xs transition-colors enabled:group-hover:text-white enabled:text-neutral-300 disabled:group-hover:cursor-default disabled:text-neutral-600"
        >
            <div class="w-[1.125rem] h-[1.125rem] flex items-center justify-center">
                {children()}
            </div>

            <div>{label}</div>
        </button>
    }
}

#[component]
fn TokenInfoInner(
    root: RootType,
    meta: TokenMetadata,
    key_principal: Option<Principal>,
) -> impl IntoView {
    let share_link = key_principal.map(|key_principal| generate_share_link(&root, key_principal));
    let message = share_link.clone().map(|share_link|format!(
        "Hey! Check out the token: {} I created on YRAL 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        meta.symbol,  share_link
    ));

    let decimals = meta.decimals;
    let blur_active = create_rw_signal(meta.is_nsfw);
    let root_c = root.clone();
    let is_utility_token =
        Signal::derive(move || matches!(root_c, RootType::COYNS | RootType::CENTS));

    let base_url = get_host();
    let show_fallback = create_rw_signal(false);

    let share_link_c = share_link.clone().unwrap_or_default();
    let on_share_click = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        if share_url(&share_link_c.clone()).is_none() {
            show_fallback.set(true);
        }
    };

    let share_link_c = share_link.clone().unwrap_or_default();

    let key_principal_s = key_principal.map(|p| p.to_text()).unwrap_or_default();

    view! {
        <div class="max-w-md mx-auto bg-black flex flex-col gap-4">
            <PopupOverlay show=show_fallback>
                <ShareContent
                    share_link=format!("{base_url}{share_link_c}")
                    message=message.clone().unwrap_or_default()
                    show_popup=show_fallback
                />
            </PopupOverlay>
            <TitleText justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback="/wallet" />
                    <span class="font-bold justify-self-center">Token details</span>
                </div>
            </TitleText>
            <div class="flex flex-col w-full items-center px-8 md:px-10 gap-8">
                <div class="flex flex-col bg-[#171717] rounded-lg p-3 gap-4">
                    <div class="flex items-center gap-2 w-full">
                        <div class="relative shrink-0">
                            <img
                                class=move || format!("object-cover h-12 w-12 rounded-sm cursor-pointer {}",
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
                        <div class="flex items-center justify-between grow gap-4">
                            <div class="flex flex-col gap-1">
                                <div class="text-lg font-medium text-neutral-50 line-clamp-1">{meta.name}</div>
                                <div class="font-medium text-sm line-clamp-1 text-neutral-400">Created by {meta.token_owner.clone().unwrap().principal_id.to_text()}</div>
                            </div>
                            <div class="flex flex-col gap-1">
                                <div class="text-lg font-bold text-neutral-50 shrink-0">${meta.symbol.clone()}</div>
                                <div class="font-medium text-sm text-neutral-400 shrink-0">
                                    3 Hrs ago
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-row justify-between">
                        <ActionButton disabled=is_utility_token href=format!("/token/transfer/{root}") label="Send".to_string()>
                            <SendIcon class="h-full w-full" />
                        </ActionButton>
                        <ActionButton disabled=true href="#".to_string() label="Buy/Sell".to_string()>
                        <Icon class="h-6 w-6" icon=ArrowLeftRightIcon />
                        </ActionButton>
                        <ActionButtonLink disabled=true on:click=move |_|{} label="Airdrop".to_string()>
                            <Icon class="h-6 w-6" icon=AirdropIcon />
                        </ActionButtonLink>

                        <ActionButton disabled=is_utility_token href="#".to_string() label="Share".to_string()>
                            <Icon class="h-6 w-6" icon=ShareIcon on:click=on_share_click />
                        </ActionButton>
                        <ActionButton disabled=is_utility_token href=format!("/token/info/{root}/{key_principal_s}") label="Details".to_string()>
                            <Icon class="h-6 w-6" icon=ChevronRightIcon />
                        </ActionButton>
                    </div>
                </div>
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
                        .get_airdrop_status(token_owner.canister_id, *root, cans.user_principal())
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
        <Title text="ICPump - Token Info" />
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
