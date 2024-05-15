use leptos::*;

use crate::{
    component::{
        back_btn::BackButton, bullet_loader::BulletLoader, canisters_prov::AuthCansProvider,
        infinite_scroller::InfiniteScroller, title::Title,
    },
    state::canisters::Canisters,
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
    view! {
        <div class="flex items-center flex-col w-dvw min-h-dvh gap-10 bg-black pt-4 px-4 pb-12">
            <Title justify_center=false>
                <div class="flex flex-row justify-between">
                    <BackButton/>
                    <span class="text-xl text-white font-bold">Transactions</span>
                    <div></div>
                </div>
            </Title>
            <AuthCansProvider fallback=BulletLoader let:canisters>
                <TransactionList canisters/>
            </AuthCansProvider>
        </div>
    }
}
