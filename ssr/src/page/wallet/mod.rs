pub mod airdrop;
pub mod tokens;
pub mod transactions;
pub mod txn;

use crate::component::icons::notification_icon::NotificationIcon;
use crate::{
    component::share_popup::ShareButtonWithFallbackPopup, state::canisters::unauth_canisters,
};
use candid::Principal;
use leptos::*;
use leptos_router::Params;
use leptos_router::{use_params, Redirect};
use tokens::TokenList;
use yral_canisters_common::utils::profile::ProfileDetails;
use yral_canisters_common::Canisters;

use crate::{
    component::{canisters_prov::AuthCansProvider, connect::ConnectLogin},
    state::{auth::account_connected_reader, canisters::authenticated_canisters},
    try_or_redirect_opt,
};

#[component]
fn ProfileCard(details: ProfileDetails, is_own_account: bool) -> impl IntoView {
    view! {
        <div class="w-full flex flex-col bg-neutral-900 rounded-lg p-4 gap-4">
            <div class="flex items-center gap-4">
                <img
                    src=details.profile_pic_or_random()
                    alt="Profile picture"
                    class="w-12 h-12 rounded-full object-cover shrink-0"
                />
                <span class="line-clamp-1 text-lg font-kumbh font-semibold text-neutral-50">
                    // TEMP: Workaround for hydration bug until leptos 0.7
                    // class=("md:w-5/12", move || !is_connected())
                    {details.display_name_or_fallback()}
                </span>
            </div>

            <Show when=move || !is_own_account>
                <ConnectLogin
                    login_text="Login to claim tokens"
                    cta_location="wallet"
                />
            </Show>
        </div>
    }
}

fn ProfileCardLoading() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col bg-neutral-900 rounded-lg p-4 gap-4">
            <div class="flex items-center gap-4">
                <div
                    class="w-12 h-12 rounded-full bg-loading shrink-0"
                />
                <div class="line-clamp-1 flex-1 bg-loading rounded-lg h-7">
                </div>
            </div>
        </div>
    }
}

#[component]
fn Header(details: ProfileDetails, is_own_account: bool) -> impl IntoView {
    let share_link = {
        let principal = details.principal();
        format!("/wallet/{}", principal)
    };
    let message = format!(
        "Hey! Check out my YRAL profile 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        share_link
    );

    view! {
        <div class="w-full flex items-center justify-between px-4 py-3 gap-10 ">
            <div class="text-white font-kumbh text-xl font-bold">My Wallet</div>
            <div class="flex items-center gap-8">
                <ShareButtonWithFallbackPopup share_link message />
                <Show when=move || is_own_account>
                    <a href="/wallet/notifications">
                        <NotificationIcon show_dot=false classes="w-6 h-6 text-[#D4D4D4]".to_string() />
                    </a>
                </Show>
            </div>
        </div>
    }
}

fn HeaderLoading() -> impl IntoView {
    view! {
        <div class="w-full flex items-center justify-between px-4 py-3 gap-10 ">
            <div class="text-white font-kumbh text-xl font-bold">My Wallet</div>
            <div class="flex items-center gap-8">
                <div class="w-6 h-6 rounded-full bg-loading"></div>
                <div class="w-6 h-6 rounded-full bg-loading"></div>
            </div>
        </div>
    }
}

#[component]
fn FallbackGreeter() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <span class="text-white/50 text-md">Welcome!</span>
            <div class="py-2 w-3/4 rounded-full animate-pulse bg-white/40"></div>
        </div>
        <div class="justify-self-end w-16 rounded-full animate-pulse aspect-square overflow-clip bg-white/40"></div>
    }
}

#[component]
fn BalanceFallback() -> impl IntoView {
    view! { <div class="py-3 mt-1 w-1/4 rounded-full animate-pulse bg-white/30"></div> }
}

#[derive(Params, PartialEq)]
struct WalletParams {
    id: String,
}
#[component]
pub fn Wallet() -> impl IntoView {
    let params = use_params::<WalletParams>();
    let param_principal = move || {
        params.with(|p| {
            let WalletParams { id, .. } = p.as_ref().ok()?;
            Principal::from_text(id).ok()
        })
    };

    view! {
        {move || {
            match param_principal() {
                Some(principal) => view! { <WalletImpl principal /> },
                None => {
                    view! {
                        <AuthCansProvider let:cans>
                            {move || {
                                view! {
                                    <Redirect path=format!("/wallet/{}", cans.user_principal()) />
                                }
                            }}
                        </AuthCansProvider>
                    }
                }
            }
        }}
    }
}
#[component]
pub fn WalletImpl(principal: Principal) -> impl IntoView {
    let (is_connected, _) = account_connected_reader();

    let auth_cans = authenticated_canisters();

    let profile_info_res = auth_cans.derive(
        move || principal,
        move |cans_wire, principal| async move {
            let cans_wire = cans_wire?;
            let canisters = Canisters::from_wire(cans_wire, expect_context())?;

            let Some(user_canister) = canisters
                .get_individual_canister_by_user_principal(principal)
                .await?
            else {
                return Err(ServerFnError::new("Failed to get user canister"));
            };
            let user = canisters.individual_user(user_canister).await;
            let user_details = user.get_profile_details().await?;
            Ok::<ProfileDetails, ServerFnError>(user_details.into())
        },
    );

    let is_own_account = auth_cans.derive(
        move || principal,
        move |cans_wire, principal| async move {
            let cans_wire = cans_wire?;
            let canisters = Canisters::from_wire(cans_wire, expect_context())?;
            Ok::<_, ServerFnError>(canisters.user_principal() == principal)
        },
    );

    let canister_id = create_resource(
        move || principal,
        move |principal| async move {
            let canisters = unauth_canisters();
            let Some(user_canister) = canisters
                .get_individual_canister_by_user_principal(principal)
                .await?
            else {
                return Err(ServerFnError::new("Failed to get user canister"));
            };
            Ok((user_canister, principal))
        },
    );
    view! {
        <div class="flex flex-col gap-4 pt-4 pb-12 bg-black min-h-dvh font-kumbh mx-auto max-w-md">
             <Suspense fallback=move || view! { <HeaderLoading/> }>
                {move || {
                    let profile_details = try_or_redirect_opt!(profile_info_res()?);
                    let is_own_account = try_or_redirect_opt!(is_own_account()?);
                    Some(
                        view! {
                            <Header details=profile_details is_own_account=is_own_account/>
                        },
                    )
                }}
            </Suspense>

            <div class="flex h-full w-full flex-col items-center justify-center max-w-md mx-auto px-4 gap-4">
                <Suspense fallback=move || view! { <ProfileCardLoading/> }>
                    {move || {
                        let profile_details = try_or_redirect_opt!(profile_info_res()?);
                        let is_own_account = try_or_redirect_opt!(is_own_account()?);
                        Some(
                            view! { <ProfileCard details=profile_details is_own_account=is_own_account /> },
                        )
                    }}
                </Suspense>
                <Suspense>
                    {move || {
                        let canister_id = try_or_redirect_opt!(canister_id() ?);
                        Some(
                            view! {
                                <div class="font-kumbh self-start pt-3 font-bold text-lg text-white">
                                    My tokens
                                </div>
                                <TokenList user_principal=canister_id.1 user_canister=canister_id.0 />
                            },
                        )
                    }}
                </Suspense>
            </div>
        </div>
    }
}
