use leptos::*;

use super::txn::{provider::get_history_provider, TxnView};
use crate::page::wallet::txn::IndexOrLedger;
use crate::{
    component::{back_btn::BackButton, infinite_scroller::InfiniteScroller, title::Title},
    state::canisters::unauth_canisters,
};
use candid::Principal;

const FETCH_CNT: usize = 15;

#[component]
pub fn TransactionList(principal: Option<Principal>, source: IndexOrLedger) -> impl IntoView {
    let provider = get_history_provider(unauth_canisters(), principal, source);
    view! {
        <div class="flex flex-col w-full items-center">
            <InfiniteScroller
                provider
                fetch_count=FETCH_CNT
                children=|info, _ref| {
                    view! { <TxnView info _ref=_ref.unwrap_or_default() /> }
                }
            />

        </div>
    }
}

#[component]
pub fn Transactions(source: IndexOrLedger, key_principal: Option<Principal>) -> impl IntoView {
    view! {
        <div class="flex items-center flex-col w-dvw min-h-dvh gap-10 bg-black pt-4 px-4 pb-12">
            <Title justify_center=false>
                <div class="flex flex-row justify-between">
                    <BackButton fallback="/wallet".to_string() />
                    <span class="text-xl text-white font-bold">Transactions</span>
                    <div></div>
                </div>
            </Title>
            <div class="flex flex-col divide-y divide-white/10">
                <TransactionList principal=key_principal source=source/>
            </div>
        </div>
    }
}
