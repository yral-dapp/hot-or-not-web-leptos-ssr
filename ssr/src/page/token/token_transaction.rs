use candid::Principal;
use leptos::*;

use crate::{
    canister::sns_root::ListSnsCanistersArg,
    component::{canisters_prov::AuthCansProvider, infinite_scroller::InfiniteScroller},
    state::canisters::{Canisters, CanistersAuthWire},
    utils::time::get_day_month,
};
use server_fn::codec::Cbor;

use super::txn::TokenTxn;

#[server(
    input = Cbor
)]
async fn token_transaction_inner(
    cans_wire: CanistersAuthWire,
    token_root: Principal,
) -> Result<Principal, ServerFnError> {
    let cans = cans_wire.canisters().unwrap();
    let root_canister = cans.sns_root(token_root).await;
    let sns_cans = root_canister
        .list_sns_canisters(ListSnsCanistersArg {})
        .await
        .unwrap();
    let index = sns_cans.index.unwrap();
    Ok(index)
}

#[component]
pub fn TokenTransactionList(
    cans: Canisters<true>,
    root: Principal,
    user_principal: Principal,
) -> impl IntoView {

    let token_resource = create_resource(
        || {},
        move |_| {
            let auth_cans_wire = cans.clone();

            async move {
                let root_canister = auth_cans_wire.sns_root(root).await;
                let sns_cans = root_canister
                    .list_sns_canisters(ListSnsCanistersArg {})
                    .await
                    .unwrap();
                let index_principal = sns_cans.index.unwrap();

                Ok(index_principal)
            }
        },
    );

    view! {
        <Suspense>
        {
            move || {
                token_resource.get().map(|res: Result<Principal, ServerFnError>| {
                    match res {
                        Ok(index_principal) => {
                        view! {
                            <div>
                            <AuthCansProvider  let:canisters>
                            <TokenTransactionListInner cans=canisters  user_principal index_principal/>
                             </AuthCansProvider>
                            </div>
                        }

                        },
                        Err(_) => {
                            view!{
                                <div>
                                <p> Some Error occured </p>
                                </div>
                            }
                        }

                    }
                })
            }
        }
        </Suspense>

    }
}

#[component]
pub fn TokenTransactionListInner(
    cans: Canisters<true>,
    user_principal: Principal,
    index_principal: Principal,
) -> impl IntoView {
    let provider =
        super::txn::provider::get_history_provider(cans, user_principal, index_principal);

    view! {
            <div class="flex flex-col w-full items-center">
            <InfiniteScroller
                provider
                fetch_count=150
                children=|info, _ref| {
                view!{
                    <TxnHistoryItem detail=info _ref=_ref.unwrap_or_default() />
                    }
                }
                empty_content=move||{
                view! {
                    <span>You do not have any transactions for this token</span>
                    }
                }
            />
            </div>
    }
}

#[component]
pub fn TokenTransactions(
    cans: Canisters<true>,
    root: Principal,
    user_principal: Principal,
) -> impl IntoView {
    view! {
        <div class="w-dvw min-h-dvh bg-neutral-800 flex flex-col gap-4">
            <span>Recent Transactions</span>
            <TokenTransactionList  cans root user_principal/>
        </div>
    }
}

#[component]
fn TxnHistoryItem(detail: TokenTxn, #[prop(into)] _ref: NodeRef<html::Div>) -> impl IntoView {
    view! {
        <div _ref=_ref class="px-2 grid grid-cols-4 grid-rows-1 items-center gap-2 w-full">
            <div class="flex flex-row col-span-3 items-center gap-4 justify-items-start">

                <div class="grid grid-cols-1 grid-rows-2">
                    <span class="text-white text-lg truncate">{detail.kind}</span>
                    <span class="text-white/50 text-sm md:text-md">
                        {get_day_month(detail.created_at_time)}
                    </span>
                    <Show when=move||detail.transfer.amount.is_some()>
                    <span class="text-white/50 text-sm md:text-md">
                         Received:{ detail.transfer.is_received }
                    </span>
                    </Show>
                </div>
            </div>
        </div>
    }
}
