use leptos::*;

use super::txn::{provider::get_history_provider, TxnView};
use crate::page::wallet::txn::IndexOrLedger;
use crate::{component::infinite_scroller::InfiniteScroller, state::canisters::unauth_canisters};
use candid::Principal;
const FETCH_CNT: usize = 15;

#[component]
pub fn TransactionList(
    principal: Option<Principal>,
    source: IndexOrLedger,
    symbol: String,
) -> impl IntoView {
    let provider = get_history_provider(unauth_canisters(), principal, source);
    view! {
        <div class="flex flex-col w-full justify-between items-stretch">
            <InfiniteScroller
                provider
                fetch_count=FETCH_CNT
                children=move |info, _ref| {
                    view! { <TxnView info _ref=_ref.unwrap_or_default() symbol=symbol.clone() /> }
                }
            />
        </div>
    }
}

#[component]
pub fn Transactions(
    source: IndexOrLedger,
    key_principal: Option<Principal>,
    symbol: String,
) -> impl IntoView {
    view! {

    <span class="text-xl w-full text-white font-bold">Transactions</span>

        <div class="flex items-center flex-col gap- pb-12 w-full">
            <div class="flex flex-col divide-y divide-white/10 w-full">
                <TransactionList principal=key_principal source=source symbol/>
            </div>
        </div>
    }
}
