use leptos::{html::Div, *};
use leptos_use::{use_intersection_observer_with_options, UseIntersectionObserverOptions};

use crate::{
    component::bullet_loader::BulletLoader,
    state::canisters::{authenticated_canisters, Canisters},
    try_or_redirect_opt,
};

use super::txn::{
    provider::{get_history_provider, HistoryProvider},
    TxnInfo, TxnView,
};

const FETCH_CNT: u64 = 15;

#[component]
pub fn TransactionList(canisters: Canisters<true>) -> impl IntoView {
    let transactions = create_rw_signal(Vec::<TxnInfo>::new());
    let end = create_rw_signal(false);
    let cursor = create_rw_signal(0);
    let txn_fetch_resource = create_resource(cursor, move |cursor| {
        let canisters = canisters.clone();
        let provider = get_history_provider(canisters);

        async move {
            let (txns, list_end) = match provider.get_history(cursor, cursor + FETCH_CNT).await {
                Ok(t) => t,
                Err(e) => {
                    log::warn!("failed to fetch tnxs err {e}");
                    (vec![], true)
                }
            };
            transactions.update(|t| t.extend(txns));
            end.set(list_end);
        }
    });
    let upper_txns = move || {
        with!(|transactions| transactions
            .iter()
            .take(transactions.len().saturating_sub(1))
            .cloned()
            .collect::<Vec<_>>())
    };
    let last_txn = move || with!(|transactions| transactions.last().cloned());
    let last_elem = create_node_ref::<Div>();
    use_intersection_observer_with_options(
        last_elem,
        move |entry, _| {
            let Some(_visible) = entry.first().filter(|entry| entry.is_intersecting()) else {
                return;
            };
            if end.get_untracked() {
                return;
            }
            cursor.update(|c| *c += FETCH_CNT);
        },
        UseIntersectionObserverOptions::default().thresholds(vec![0.1]),
    );

    view! {
        <div class="flex flex-col w-full items-center">
            <For each=upper_txns key=|t| t.id let:info>
                <TxnView info/>
            </For>
            {move || {
                last_txn()
                    .map(|info| {
                        view! {
                            <div _ref=last_elem class="w-full">
                                <TxnView info/>
                            </div>
                        }
                    })
            }}

            <Show when=txn_fetch_resource.loading()>
                <BulletLoader/>
            </Show>
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
