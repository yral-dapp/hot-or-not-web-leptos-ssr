use candid::{Nat, Principal};
use codee::string::FromToStringCodec;
use leptos_use::use_cookie;
use yral_canisters_common::cursored_data::token_roots::{TokenListResponse, TokenRootList};
use yral_canisters_common::utils::token::{RootType, TokenMetadata, TokenOwner};
use yral_canisters_common::Canisters;

use crate::component::icons::{
    airdrop_icon::AirdropIcon, arrow_left_right_icon::ArrowLeftRightIcon,
    chevron_right_icon::ChevronRightIcon, send_icon::SendIcon, share_icon::ShareIcon,
};
use crate::component::overlay::PopupOverlay;
use crate::component::share_popup::ShareContent;
use crate::consts::USER_PRINCIPAL_STORE;
use crate::page::icpump::{ActionButton, ActionButtonLink};
use crate::state::canisters::authenticated_canisters;
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
use crate::page::wallet::airdrop::AirdropPopup;
#[component]
pub fn WalletCard(
    user_principal: Principal,
    token_meta_data: TokenMetadata,
    is_airdrop_claimed: bool,
    #[prop(optional)] is_utility_token: bool,
    #[prop(optional)] _ref: NodeRef<html::Div>,
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

    let airdrop_popup = create_rw_signal(false);
    let buffer_signal = create_rw_signal(false);
    let claimed = create_rw_signal(is_airdrop_claimed);
    view! {
        <div node_ref=_ref class="flex flex-col gap-4 bg-neutral-900/90 rounded-lg w-full font-kumbh text-white p-4">
            <div class="w-full flex items-center justify-between p-3 rounded-[4px] bg-neutral-800/70">
                <div class="flex items-center gap-2">
                    <img
                        clone:token_meta_data
                        src=token_meta_data.logo_b64.clone()
                        alt=token_meta_data.name.clone()
                        class="w-8 h-8 rounded-full object-cover"
                    />
                    <div class="text-sm font-medium uppercase truncate">{token_meta_data.name.clone()}</div>
                </div>
                <div class="flex flex-col items-end">
                    {
                        token_meta_data.balance.map(|b| view! {
                            <div class="text-lg font-medium">{b.humanize_float_truncate_to_dp(2)}</div>
                        })
                    }
                    <div class="text-xs">{symbol}</div>
                </div>
            </div>

            <WalletCardOptions pop_up=pop_up.write_only() share_link=share_link.write_only() airdrop_popup buffer_signal claimed/>

            <PopupOverlay show=pop_up >
                <ShareContent
                    share_link=format!("{base_url}{}", share_link())
                    message=share_message()
                    show_popup=pop_up
                />
            </PopupOverlay>

            <Show when=airdrop_popup>
                <div class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 max-w-[560px] max-h-[634px] min-w-[343px] min-h-[480px] backdrop-blur-lg rounded-lg">
                    <div class="rounded-lg z-50">
                        <AirdropPopup
                            name=token_meta_data.name.clone()
                            logo=token_meta_data.logo_b64.clone()
                            buffer_signal
                            claimed
                            airdrop_popup
                        />
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn Test() -> impl IntoView {
    let buffer_signal = create_rw_signal(true);
    let claimed = create_rw_signal(false);
    let airdrop_popup = create_rw_signal(false);
    view! {
        <div class="w-screen h-screen">
        <Show when=move || true>
            <div class="fixed top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 max-w-[560px] max-h-[634px] min-w-[343px] min-h-[480px] backdrop-blur-lg rounded-lg">
                <div class="rounded-lg z-50">
                    <AirdropPopup
                        name="SHIT".to_string()
                        logo="".to_string()
                        buffer_signal
                        claimed
                        airdrop_popup
                    />
                </div>
            </div>
        </Show>
    </div>
    }
}

#[component]
fn WalletCardOptions(
    pop_up: WriteSignal<bool>,
    share_link: WriteSignal<String>,
    airdrop_popup: RwSignal<bool>,
    buffer_signal: RwSignal<bool>,
    claimed: RwSignal<bool>,
) -> impl IntoView {
    use_context().map(|WalletCardOptionsContext { is_airdrop_claimed, is_utility_token, root, token_owner, user_principal }|{
        let share_link_coin = format!("/token/info/{root}/{user_principal}");
        let token_owner_c = token_owner.clone();
        let root_c = root.clone();
        let cans_res = authenticated_canisters();
        let airdrop_action = create_action(move |&()| {
            let cans_res = cans_res.clone();
            let token_owner_cans_id = token_owner_c.clone().unwrap().canister_id;
            airdrop_popup.set(true);
            let root = Principal::from_text(root_c.clone()).unwrap();

            async move {
                if claimed.get() && !buffer_signal.get() {
                    return Ok(());
                }
                buffer_signal.set(true);
                let cans_wire = cans_res.wait_untracked().await?;
                let cans = Canisters::from_wire(cans_wire, expect_context())?;
                let token_owner = cans.individual_user(token_owner_cans_id).await;
                token_owner
                    .request_airdrop(
                        root,
                        None,
                        Into::<Nat>::into(100u64) * 10u64.pow(8),
                        cans.user_canister(),
                    )
                    .await?;
                let user = cans.individual_user(cans.user_canister()).await;
                user.add_token(root).await?;
                buffer_signal.set(false);
                claimed.set(true);
                Ok::<_, ServerFnError>(())
            }
        });


        view! {
            <div class="flex items-center justify-around">
            <ActionButton disabled=is_utility_token href=format!("/token/transfer/{root}") label="Send".to_string()>
                <Icon class="h-6 w-6" icon=SendIcon/>
            </ActionButton>
            <ActionButton disabled=true href="#".to_string() label="Buy/Sell".to_string()>
                <Icon class="h-6 w-6" icon=ArrowLeftRightIcon />
            </ActionButton>
            <ActionButtonLink disabled=token_owner.is_some() && is_airdrop_claimed || token_owner.is_none() on:click=move |_|{airdrop_action.dispatch(());} label="Airdrop".to_string()>
                <Icon class="h-6 w-6" icon=AirdropIcon />
            </ActionButtonLink>

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
