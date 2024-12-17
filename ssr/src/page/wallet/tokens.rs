use candid::Principal;
use yral_canisters_common::cursored_data::token_roots::TokenRootList;
use yral_canisters_common::utils::token::{RootType, TokenMetadata};

use crate::component::icons::{
    airdrop_icon::AirdropIcon, arrow_left_right_icon::ArrowLeftRightIcon,
    chevron_right_icon::ChevronRightIcon, send_icon::SendIcon, share_icon::ShareIcon,
};
use crate::component::overlay::PopupOverlay;
use crate::component::share_popup::ShareContent;
use crate::page::icpump::ActionButton;
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
                    view! { <WalletCard user_principal token_meta_data=info.clone() /> }
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
pub fn TokenList(user_principal: Principal, user_canister: Principal) -> impl IntoView {
    let canisters: yral_canisters_common::Canisters<false> = unauth_canisters();

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
                    view! {
                        <TokenView
                            user_principal
                            token_root=token_root
                            _ref=_ref.unwrap_or_default()
                        />
                    }
                }
            />

        </div>
    }
}

#[component]
pub fn WalletCard(user_principal: Principal, token_meta_data: TokenMetadata) -> impl IntoView {
    let root: String = token_meta_data
        .root
        .map(|r| r.to_text())
        .unwrap_or(token_meta_data.name.to_lowercase());

    let share_link = generate_share_link_from_metadata(&token_meta_data, user_principal);
    let share_link_s = store_value(share_link);
    let share_message = format!(
        "Hey! Check out the token: {} I created on YRAL 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        token_meta_data.symbol,
        share_link_s(),
    );
    let share_message_s = store_value(share_message);
    let pop_up = create_rw_signal(false);
    let base_url = get_host();
    view! {
        <div class="flex flex-col gap-4 bg-neutral-900/90 rounded-lg w-full p-4 font-kumbh text-white">
            <div class="w-full flex items-center justify-between p-3 rounded-[4px] bg-neutral-800/70">
                <div class="flex items-center gap-2">
                    <img
                        src=token_meta_data.logo_b64
                        alt=token_meta_data.name.clone()
                        class="w-8 h-8 rounded-full object-cover"
                    />
                    <div class="text-sm font-medium uppercase">{token_meta_data.name}</div>
                </div>
                <div class="flex flex-col items-end">
                    <div class="text-lg font-medium">{token_meta_data.balance.unwrap().humanize_float_truncate_to_dp(2)}</div>
                    <div class="text-xs">{token_meta_data.symbol}</div>
                </div>
            </div>
            <div class="flex items-center justify-around">
                <ActionButton href=format!("/token/transfer{root}") label="Send".to_string()>
                    <Icon class="h-6 w-6" icon=SendIcon/>
                </ActionButton>
                <ActionButton disabled=true href="#".to_string() label="Swap".to_string()>
                    <Icon class="h-6 w-6" icon=ArrowLeftRightIcon />
                </ActionButton>
                <ActionButton disabled=true href="#".to_string() label="Airdrop".to_string()>
                    <Icon class="h-6 w-6" icon=AirdropIcon />
                </ActionButton>
                <ActionButton href="#".to_string() label="Share".to_string()>
                    <Icon class="h-6 w-6" icon=ShareIcon on:click=move |_| pop_up.set(true) />
                </ActionButton>
                <ActionButton href=format!("/token/{root}/{user_principal}") label="Details".to_string()>
                    <Icon class="h-6 w-6" icon=ChevronRightIcon />
                </ActionButton>
            </div>

            <PopupOverlay show=pop_up >
                <ShareContent
                    share_link=format!("{base_url}{}", share_link_s())
                    message=share_message_s()
                    show_popup=pop_up
                />
            </PopupOverlay>
        </div>
    }
}
