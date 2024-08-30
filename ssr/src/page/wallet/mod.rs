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
    // let (is_connected, _) = account_connected_reader();

    view! {
        <div class="flex flex-col">
            <span class="text-white/50 text-md">Welcome!</span>
            <span
                class="text-lg text-white md:text-xl truncate"
                // TEMP: Workaround for hydration bug until leptos 0.7
                // class=("md:w-5/12", move || !is_connected())
            >
                {details.display_name_or_fallback()}
            </span>
        </div>
        <div class="justify-self-end w-16 rounded-full aspect-square overflow-clip">
            <img class="object-cover w-full h-full" src=details.profile_pic_or_random()/>
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

const RECENT_TXN_CNT: usize = 10;

#[component]
fn BalanceFallback() -> impl IntoView {
    view! { <div class="py-3 mt-1 w-1/4 rounded-full animate-pulse bg-white/30"></div> }
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
            <div class="top-0 z-50 items-center pt-4 pl-4 w-full text-white bg-black">
                <div class="flex flex-row justify-start">
                    <BackButton fallback="/".to_string()/>
                </div>
            </div>
            <div class="flex flex-col gap-4 px-4 pt-4 pb-12 bg-black w-dvw min-h-dvh">
                <div class="grid grid-cols-2 grid-rows-1 items-center w-full">
                    <AuthCansProvider fallback=FallbackGreeter let:cans>
                        <ProfileGreeter details=cans.profile_details()/>
                    </AuthCansProvider>
                </div>
                <div class="flex flex-col items-center mt-6 w-full text-white">
                    <span class="uppercase lg:text-lg text-md">Your Coyns Balance</span>
                    <WithAuthCans fallback=BalanceFallback with=balance_fetch let:bal>
                        <div class="text-xl lg:text-2xl">{bal.1}</div>
                    </WithAuthCans>
                </div>
                <Show when=move || !is_connected()>
                    <div class="flex flex-col items-center py-5 w-full">
                        <div class="flex flex-row items-center w-9/12 md:w-5/12">
                            <ConnectLogin
                                login_text="Login to claim your COYNs"
                                cta_location="wallet"
                            />
                        </div>
                    </div>
                </Show>
                <div class="flex flex-col gap-2 w-full">
                    <div class="flex flex-row justify-between items-end w-full">
                        <span class="text-sm text-white md:text-md">Recent Transactions</span>
                        <a href="/transactions" class="md:text-lg text-white/50 text-md">
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
