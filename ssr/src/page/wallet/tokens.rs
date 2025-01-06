use candid::Principal;
use codee::string::FromToStringCodec;
use leptos_use::use_cookie;
use yral_canisters_common::cursored_data::token_roots::{TokenListResponse, TokenRootList};
use yral_canisters_common::utils::token::{RootType, TokenMetadata, TokenOwner};

use crate::component::icons::{
    airdrop_icon::AirdropIcon, arrow_left_right_icon::ArrowLeftRightIcon,
    chevron_right_icon::ChevronRightIcon, send_icon::SendIcon, share_icon::ShareIcon,
};
use crate::component::overlay::PopupOverlay;
use crate::component::share_popup::ShareContent;
use crate::consts::USER_PRINCIPAL_STORE;
use crate::page::icpump::{ActionButton, ActionButtonLink};
use crate::utils::host::get_host;
use crate::utils::token::icpump::IcpumpTokenInfo;
use crate::{component::infinite_scroller::InfiniteScroller, state::canisters::unauth_canisters};

use leptos::*;
use leptos_icons::*;

#[component]
pub fn TokenViewFallback() -> impl IntoView {
    view! {
        <div class="w-full items-center h-16 rounded-xl border-2 border-neutral-700 bg-white/15 animate-pulse"></div>
    }
}

#[component]
pub fn TokenList(user_principal: Principal, user_canister: Principal) -> impl IntoView {
    let (viewer_principal, _) = use_cookie::<Principal, FromToStringCodec>(USER_PRINCIPAL_STORE);

    let provider = TokenRootList {
        viewer_principal: viewer_principal.get_untracked().unwrap(),
        canisters: unauth_canisters(),
        user_canister,
        user_principal,
        nsfw_detector: IcpumpTokenInfo,
    };

    view! {
        <div class="flex flex-col w-full gap-2 mb-2 items-center">
            <InfiniteScroller
                provider
                fetch_count=5
                children=move |TokenListResponse{token_metadata, airdrop_claimed, root}, _ref| {
                    view! {
                        <WalletCard user_principal token_meta_data=token_metadata is_airdrop_claimed=airdrop_claimed _ref=_ref.unwrap_or_default() is_utility_token=root == RootType::COYNS/>
                    }
                }
            />

        </div>
    }
}

#[derive(Clone)]
struct WalletCardOptionsContext {
    is_airdrop_claimed: bool,
    is_utility_token: bool,
    root: String,
    token_owner: Option<TokenOwner>,
    user_principal: Principal,
}

#[component]
pub fn WalletCard(
    user_principal: Principal,
    token_meta_data: TokenMetadata,
    is_airdrop_claimed: bool,
    #[prop(optional)] is_utility_token: bool,
    #[prop(optional)] _ref: NodeRef<html::A>,
) -> impl IntoView {
    let root: String = token_meta_data
        .root
        .map(|r| r.to_text())
        .unwrap_or(token_meta_data.name.to_lowercase());

    let share_link = create_rw_signal("".to_string());

    let symbol = token_meta_data.symbol.clone();
    let share_message = move || {
        format!(
        "Hey! Check out the token: {} I created on YRAL 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        token_meta_data.symbol.clone(),
        share_link.get(),
    )
    };
    let pop_up = create_rw_signal(false);
    let base_url = get_host();

    provide_context(WalletCardOptionsContext {
        is_airdrop_claimed,
        is_utility_token,
        root,
        token_owner: token_meta_data.token_owner,
        user_principal,
    });

    view! {
        <div class="flex flex-col gap-4 bg-neutral-900/90 rounded-lg w-full p-4 font-kumbh text-white">
            <div class="w-full flex items-center justify-between p-3 rounded-[4px] bg-neutral-800/70">
                <div class="flex items-center gap-2">
                    <img
                        src=token_meta_data.logo_b64
                        alt=token_meta_data.name.clone()
                        class="w-8 h-8 rounded-full object-cover"
                    />
                    <div class="text-sm font-medium uppercase truncate">{token_meta_data.name}</div>
                </div>
                <div class="flex flex-col items-end">
                    <div class="text-lg font-medium">{token_meta_data.balance.unwrap().humanize_float_truncate_to_dp(2)}</div>
                    <div class="text-xs">{symbol}</div>
                </div>
            </div>

            <WalletCardOptions pop_up=pop_up.write_only() share_link=share_link.write_only()/>

            <PopupOverlay show=pop_up >
                <ShareContent
                    share_link=format!("{base_url}{}", share_link())
                    message=share_message()
                    show_popup=pop_up
                />
            </PopupOverlay>
        </div>
    }
}

#[component]
fn WalletCardOptions(pop_up: WriteSignal<bool>, share_link: WriteSignal<String>) -> impl IntoView {
    use_context().map(|WalletCardOptionsContext { is_airdrop_claimed, is_utility_token, root, token_owner, user_principal }|{
        let share_link_coin = format!("/token/info/{root}/{user_principal}");
        view! {
            <div class="flex items-center justify-around">
            <ActionButton disabled=is_utility_token href=format!("/token/transfer/{root}") label="Send".to_string()>
                <Icon class="h-6 w-6" icon=SendIcon/>
            </ActionButton>
            <ActionButton disabled=true href="#".to_string() label="Buy/Sell".to_string()>
                <Icon class="h-6 w-6" icon=ArrowLeftRightIcon />
            </ActionButton>
            {
                match token_owner{
                    Some(token_owner) => {
                        if is_airdrop_claimed{
                            let root = root.clone();
                            view! {
                                <ActionButtonLink on:click=move |_|{pop_up.set(true); share_link.set(format!("/token/info/{}/{}?airdrop_amt=100",root, token_owner.principal_id))} label="Airdrop".to_string()>
                                    <Icon class="h-6 w-6" icon=AirdropIcon />
                                </ActionButtonLink>
                            }
                        }else{
                            view! {
                                <ActionButton href=format!("/token/info/{}/{}?airdrop_amt=100",root, token_owner.principal_id) label="Airdrop".to_string()>
                                    <Icon class="h-6 w-6" icon=AirdropIcon />
                                </ActionButton>
                            }
                        }
                    },
                    None => {
                        view! {
                            <ActionButton href="#".to_string() label="Airdrop".to_string() disabled=true>
                                <Icon class="h-6 w-6" icon=AirdropIcon />
                            </ActionButton>
                        }
                    }
                }
            }
            <ActionButton disabled=is_utility_token href="#".to_string() label="Share".to_string()>
                <Icon class="h-6 w-6" icon=ShareIcon on:click=move |_| {pop_up.set(true); share_link.set(share_link_coin.clone())}/>
            </ActionButton>
            <ActionButton disabled=is_utility_token href=format!("/token/info/{root}/{user_principal}") label="Details".to_string()>
                <Icon class="h-6 w-6" icon=ChevronRightIcon />
            </ActionButton>
        </div>
        }
    })
}
