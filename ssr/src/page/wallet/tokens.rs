use candid::Principal;
use yral_canisters_common::cursored_data::token_roots::TokenRootList;
use yral_canisters_common::utils::token::{RootType, TokenMetadata};

use crate::page::wallet::ShareButtonWithFallbackPopup;
use crate::utils::token::icpump::IcpumpTokenInfo;
use crate::{component::infinite_scroller::InfiniteScroller, state::canisters::unauth_canisters};
use leptos::*;

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
