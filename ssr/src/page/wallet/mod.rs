pub mod transactions;
mod txn;
use leptos::*;

use crate::{
    component::{
        back_btn::BackButton,
        bullet_loader::BulletLoader,
        canisters_prov::{AuthCansProvider, WithAuthCans},
        connect::ConnectLogin,
        infinite_scroller::{CursoredDataProvider, KeyedData},
    },
    state::{auth::account_connected_reader, canisters::Canisters},
    utils::profile::ProfileDetails,
};
use txn::{provider::get_history_provider, TxnView};

#[component]
fn ProfileGreeter(details: ProfileDetails) -> impl IntoView {
    let (is_connected, _) = account_connected_reader();

    view! {
        <div class="flex flex-col">
            <span class="text-white/50 text-md">Welcome!</span>
            <span class="text-white text-lg md:text-xl truncate"
                class=("md:w-5/12", move || !is_connected())>
                {details.display_name_or_fallback()}
            </span>
        </div>
        <div class="w-16 aspect-square overflow-clip justify-self-end rounded-full">
            <img class="h-full w-full object-cover" src=details.profile_pic_or_random()/>
        </div>
    }
}

#[component]
fn FallbackGreeter() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <span class="text-white/50 text-md">Welcome!</span>
            <div class="w-3/4 rounded-full py-2 bg-white/40 animate-pulse"></div>
        </div>
        <div class="w-16 aspect-square overflow-clip rounded-full justify-self-end bg-white/40 animate-pulse"></div>
    }
}

const RECENT_TXN_CNT: usize = 10;

#[component]
fn BalanceFallback() -> impl IntoView {
    view! { <div class="w-1/4 rounded-full py-3 mt-1 bg-white/30 animate-pulse"></div> }
}

#[component]
fn BalanceFetch(cans: Canisters<true>) -> impl IntoView {
    let balance_resource = create_resource(
        || (),
        move |_| {
            let canisters = cans.clone();
            async move {
                let Ok(user) = canisters.authenticated_user().await else {
                    return "Error".to_string();
                };

                user.get_utility_token_balance()
                    .await
                    .map(|b| b.to_string())
                    .unwrap_or("Error".to_string())
            }
        },
    );

    view! {
        {move || {
            balance_resource().map(|bal| view! { <span class="text-xl lg:text-2xl">{bal}</span> })
        }}
    }
}

#[component]
pub fn Wallet() -> impl IntoView {
    let (is_connected, _) = account_connected_reader();

    let balance_fetch = |cans: Canisters<true>| async move {
        let Ok(user) = cans.authenticated_user().await else {
            return "Error".to_string();
        };

        user.get_utility_token_balance()
            .await
            .map(|b| b.to_string())
            .unwrap_or("Error".to_string())
    };
    let history_fetch = |cans: Canisters<true>| {
        let history_prov = get_history_provider(cans);
        async move {
            let page = history_prov.get_by_cursor(0, RECENT_TXN_CNT).await;

            page.map(|p| p.data).unwrap_or(vec![])
        }
    };

    view! {
        <div>
            <div class="top-0 bg-black text-white w-full items-center z-50 pt-4 pl-4">
                <div class="flex flex-row justify-start">
                    <BackButton fallback="/".to_string()/>
                </div>
            </div>
            <div class="flex flex-col w-dvw min-h-dvh bg-black gap-4 px-4 pt-4 pb-12">
                <div class="grid grid-cols-2 grid-rows-1 items-center w-full">
                    <AuthCansProvider fallback=FallbackGreeter let:cans>
                        <ProfileGreeter details=cans.profile_details()/>
                    </AuthCansProvider>
                </div>
                <div class="flex flex-col w-full items-center mt-6 text-white">
                    <span class="text-md lg:text-lg uppercase">Your Coyns Balance</span>
                    <WithAuthCans fallback=BalanceFallback with=balance_fetch let:bal>
                        <span class="text-xl lg:text-2xl">{bal.1}</span>
                    </WithAuthCans>
                </div>
                <Show when=move || !is_connected()>
                    <div class="flex flex-col w-full py-5 items-center">
                        <div class="flex flex-row w-9/12 md:w-5/12 items-center">
                            <ConnectLogin
                                login_text="Login to claim your COYNs"
                                cta_location="wallet"
                            />
                        </div>
                    </div>
                </Show>
                <div class="flex flex-col w-full gap-2">
                    <div class="flex flex-row w-full items-end justify-between">
                        <span class="text-white text-sm md:text-md">Recent Transactions</span>
                        <a href="/transactions" class="text-white/50 text-md md:text-lg">
                            See All
                        </a>
                    </div>
                    <div class="flex flex-col divide-y divide-white/10">
                        <WithAuthCans fallback=BulletLoader with=history_fetch let:history>
                            <For each=move || history.1.clone() key=|inf| inf.key() let:info>
                                <TxnView info/>
                            </For>
                        </WithAuthCans>
                    </div>
                </div>
            </div>
        </div>
    }
}
