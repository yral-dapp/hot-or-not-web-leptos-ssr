use leptos::*;
use yral_canisters_common::cursored_data::transaction::IndexOrLedger;

use super::txn::{provider::get_history_provider, TxnView};
use crate::{component::infinite_scroller::InfiniteScroller, state::canisters::unauth_canisters};

const FETCH_CNT: usize = 15;

#[component]
pub fn TransactionList(source: IndexOrLedger, symbol: String, decimals: u8) -> impl IntoView {
    let provider = get_history_provider(unauth_canisters(), source, decimals);
    view! {
        <div class="flex flex-col w-full gap-3 justify-between items-stretch">
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
pub fn Transactions(source: IndexOrLedger, symbol: String, decimals: u8) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 p-3 bg-[#171717] rounded-lg w-full">
            <div class="w-full text-white flex items-center justify-center rounded-md bg-neutral-950 text-sm font-bold py-3">
                Transactions
            </div>
            <TransactionList source=source symbol decimals/>
        </div>
    }
}
