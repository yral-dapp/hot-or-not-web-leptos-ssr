use leptos::*;

use crate::{
    component::{bullet_loader::BulletLoader, infinite_scroller::InfiniteScroller},
    state::canisters::{authenticated_canisters, Canisters},
    try_or_redirect_opt,
};

use super::txn::{provider::get_history_provider, TxnView};

const FETCH_CNT: usize = 15;

#[component]
pub fn TransactionList(canisters: Canisters<true>) -> impl IntoView {
    let provider = get_history_provider(canisters);
    view! {
        <div class="flex flex-col w-full items-center">
            <InfiniteScroller
                provider
                fetch_count=FETCH_CNT
                children=|info, _ref| {
                    view! { <TxnView info _ref=_ref.unwrap_or_default()/> }
                }
            />

        </div>
    }
}

#[component]
pub fn Transactions() -> impl IntoView {
    let canisters = authenticated_canisters();

    view! {
        <div class="flex items-center flex-col w-dvw min-h-dvh gap-10 bg-black pt-4 px-4 pb-12">
            <span class="text-xl text-white font-bold">Transactions</span>
            <Suspense fallback=BulletLoader>
                {move || {
                    canisters
                        .get()
                        .and_then(|c| {
                            let canisters = try_or_redirect_opt!(c)?;
                            Some(view! { <TransactionList canisters/> })
                        })
                        .unwrap_or_else(|| view! { <BulletLoader/> })
                }}

            </Suspense>
        </div>
    }
}
