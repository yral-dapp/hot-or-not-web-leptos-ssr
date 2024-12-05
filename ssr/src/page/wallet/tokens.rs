use candid::Principal;
use yral_canisters_common::cursored_data::token_roots::TokenRootList;
use yral_canisters_common::utils::token::{RootType, TokenMetadata};

use crate::page::wallet::ShareButtonWithFallbackPopup;
use crate::utils::token::icpump::IcpumpTokenInfo;
use crate::{component::infinite_scroller::InfiniteScroller, state::canisters::unauth_canisters};
use leptos::*;
use leptos_icons::*;
use crate::component::icons::{send_icon::SendIcon, arrow_left_right_icon::ArrowLeftRightIcon, airdrop_icon::AirdropIcon, share_icon::ShareIcon, chevron_right_icon::ChevronRightIcon};

#[component]
pub fn TokenViewFallback() -> impl IntoView {
    view! {
        <div class="w-full items-center h-16 rounded-xl border-2 border-neutral-700 bg-white/15 animate-pulse"></div>
    }
}

#[component]
pub fn TokenView(
    user_principal: Principal,
    token_root: RootType,
    #[prop(optional)] _ref: NodeRef<html::A>,
) -> impl IntoView {
    let info = create_resource(
        move || (token_root.clone(), user_principal),
        move |(token_root, user_principal)| async move {
            let cans = unauth_canisters();
            // TODO: remove these unwraps
            cans.token_metadata_by_root_type(&IcpumpTokenInfo, Some(user_principal), token_root)
                .await
                .unwrap()
                .unwrap()
        },
    );

    view! {
        <Suspense fallback=TokenViewFallback>
            {move || {
                info.map(|info| {
                    view! {
                        <TokenTile
                            user_principal
                            token_meta_data=info.clone()
                        />
                    }
                })
            }}

        </Suspense>
    }
}

fn generate_share_link_from_metadata(
    token_meta_data: &TokenMetadata,
    user_principal: Principal,
) -> String {
    format!(
        "/token/info/{}/{user_principal}?airdrop_amt=100",
        token_meta_data
            .root
            .map(|r| r.to_text())
            .unwrap_or(token_meta_data.name.to_lowercase())
    )
}

#[component]
pub fn TokenTile(user_principal: Principal, token_meta_data: TokenMetadata) -> impl IntoView {
    let share_link = generate_share_link_from_metadata(&token_meta_data, user_principal);
    let share_link_s = store_value(share_link);
    let share_message = format!(
        "Hey! Check out the token: {} I created on YRAL 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        token_meta_data.symbol,
        share_link_s(),
    );
    let share_message_s = store_value(share_message);
    let info = token_meta_data;
    view! {
        <div class="flex  w-full items-center h-16 rounded-xl border-2 border-neutral-700 bg-white/15 gap-1">
            <a
                href=share_link_s()
                // _ref=_ref
                class="flex flex-1  p-y-4"
            >
                <div class="flex flex-2 items-center space-x-2 px-2">
                    <div class="relative">
                        <img
                            class=move || {
                                let mut classes = "w-12 h-12 rounded-full".to_string();
                                if info.is_nsfw {
                                    classes.push_str(" blur-md");
                                }
                                classes
                            }
                            src=info.logo_b64.clone()
                        />
                        <Show
                            when=move || info.is_nsfw
                        >
                            <Icon
                                icon=icondata::AiEyeInvisibleOutlined
                                class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-6 h-6 text-white"
                            />
                        </Show>
                    </div>
                    <span class="text-white text-xs truncate">{info.name.clone()}</span>
                </div>
                <div class="flex flex-1 flex-col">
                    <span class="flex flex-1  items-center justify-end text-xs text-white">
                        // remove the unwrap if global token listing but its a list of token so it can safely be unwrapped
                        {info.balance.unwrap().humanize_float_truncate_to_dp(2)}
                    </span>
                    <span class="flex flex-1  items-center justify-end text-xs text-white truncate">
                        {info.symbol.clone()}
                    </span>
                </div>

            </a>
            <div>
                <ShareButtonWithFallbackPopup
                    share_link=share_link_s()
                    message=share_message_s()
                    style="w-12 h-12".into()
                />
            </div>

        </div>
    }
}

#[component]
pub fn TokenList(user_principal: Principal, user_canister: Principal) -> impl IntoView {
    let canisters = unauth_canisters();

    let provider = TokenRootList {
        canisters,
        user_canister,
        user_principal,
        nsfw_detector: IcpumpTokenInfo,
    };

    view! {
        <div class="flex flex-col w-full gap-2 items-center">
            <InfiniteScroller
                provider
                fetch_count=10
                children=move |token_root, _ref| {
                    view! { <TokenView user_principal token_root=token_root _ref=_ref.unwrap_or_default() /> }
                }
            />

        </div>
    }
}



#[component]
fn WalletCard(
    icon_url:String,
    token_name:String,
    token_symbol:String,
    balance:f64,
) -> impl IntoView {
    view! {

        <div class="flex flex-col gap-4 bg-[#131313] rounded-lg w-full p-4 font-kumbh">
        <div class="w-full flex items-center justify-between p-3 rounded-[4px] bg-[#202125]">
            <div class="flex items-center gap-2">
                <img src={icon_url} alt={token_name.clone()} class="w-8 h-8 rounded-full object-cover" />
                <div class="text-sm font-medium uppercase">{token_name}</div>
            </div>
            <div class="flex flex-col items-end">
                <div class="text-lg font-medium">{balance}</div>
                <div class="text-xs">{token_symbol}</div>
            </div>
        </div>
        <div class="flex items-center justify-around">
           <ActionButton href="#".to_string() label="Send".to_string() >
           <SendIcon classes="h-6 w-6".to_string() />
           </ActionButton>
           <ActionButton href="#".to_string() label="Swap".to_string() >
            <ArrowLeftRightIcon classes="h-6 w-6".to_string() />
           </ActionButton>
           <ActionButton href="#".to_string() label="Airdrop".to_string() >
            <AirdropIcon classes="h-6 w-6".to_string() />
           </ActionButton>
            <ActionButton href="#".to_string() label="Share".to_string() >
            <ShareIcon classes="h-6 w-6".to_string() />
           </ActionButton>
           <ActionButton href="#".to_string() label="Details".to_string() >
            <ChevronRightIcon classes="h-6 w-6".to_string() />
           </ActionButton>
        </div>
    </div>
    
    }
}

#[component]
fn ActionButton(
    children:Children,
    href: String,
    label: String,
) -> impl IntoView {
    view! {
        <a href=href class="flex flex-col flex-1 gap-1 items-center justify-center text-[#A0A1A6]">
            <div class="flex items-center justify-center gap-4">
                {children()}
            </div>
            <div class="text-sm">{label}</div>
        </a>
    }
}