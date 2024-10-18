use std::fmt::{self, Display, Formatter};

use candid::Principal;
use leptos::*;
use leptos_icons::Icon;
use leptos_router::use_params;
use serde::{Deserialize, Serialize};

use crate::{
    component::infinite_scroller::KeyedData,
    page::token::info::TokenKeyParam,
    utils::{time::parse_ns_to_datetime, token::TokenBalance},
};

#[derive(Clone, Copy)]
pub enum TxnDirection {
    Transaction,
    Added,
    Deducted,
}
#[derive(Clone)]
pub enum IndexOrLedger {
    Index(Principal),
    Ledger(Principal),
}

impl From<TxnDirection> for &'static icondata_core::IconData {
    fn from(val: TxnDirection) -> Self {
        use TxnDirection::*;
        match val {
            Transaction => icondata::LuArrowLeftRight,
            Added => icondata::FaArrowDownSolid,
            Deducted => icondata::FaArrowUpSolid,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum TxnInfoType {
    Mint { to: Principal },
    Sent { to: Principal }, // only for keyed
    Burn { from: Principal },
    Received { from: Principal },                // only for keyed
    Transfer { from: Principal, to: Principal }, // only for public transaction
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TxnInfoWallet {
    pub tag: TxnInfoType,
    pub timestamp: u64,
    pub amount: TokenBalance,
    pub id: u64,
}

impl KeyedData for TxnInfoWallet {
    type Key = u64;

    fn key(&self) -> Self::Key {
        self.id
    }
}
impl From<TxnInfoType> for TxnDirection {
    fn from(value: TxnInfoType) -> TxnDirection {
        match value {
            TxnInfoType::Burn { .. } | TxnInfoType::Sent { .. } => TxnDirection::Deducted,
            TxnInfoType::Mint { .. } | TxnInfoType::Received { .. } => TxnDirection::Added,
            TxnInfoType::Transfer { .. } => TxnDirection::Transaction,
        }
    }
}

impl TxnInfoType {
    fn to_text(self) -> &'static str {
        match self {
            TxnInfoType::Burn { .. } => "Burned",
            TxnInfoType::Mint { .. } => "Minted",
            TxnInfoType::Received { .. } => "Received",
            TxnInfoType::Sent { .. } => "Sent",
            TxnInfoType::Transfer { .. } => "Transferred",
        }
    }

    fn icondata(self) -> &'static icondata_core::IconData {
        TxnDirection::from(self).into()
    }
}

impl Display for TxnInfoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_text())
    }
}

#[component]
pub fn TxnView(
    info: TxnInfoWallet,
    #[prop(optional)] _ref: NodeRef<html::Div>,
    symbol: String,
) -> impl IntoView {
    let params = use_params::<TokenKeyParam>();
    let direction = TxnDirection::from(info.tag);
    let bal_res = format!(
        "{}{}",
        match direction {
            TxnDirection::Added => "+",
            TxnDirection::Deducted => "-",
            TxnDirection::Transaction => "",
        },
        info.amount.humanize_float_truncate_to_dp(2)
    );

    view! {
        <div _ref=_ref class="grid grid-cols-2 grid-rows-1 w-full py-3 border-b-2 border-white/10 justify-between">
            <div class="flex flex-row gap-2">
                {
                    match direction{
                        TxnDirection::Added => {
                            view! {
                                <div class="flex items-center justify-center w-7 h-7 lg:w-10 lg:h-10 rounded-md text-green-600 bg-green-600/5 text-lg lg:text-xl">
                                    <Icon icon=info.tag.icondata() />
                                </div>
                            }
                        },
                        TxnDirection::Deducted => {
                            view! {
                                <div class="flex items-center justify-center w-7 h-7 lg:w-10 lg:h-10 rounded-md text-red-600 bg-red-600/5 text-lg lg:text-xl">
                                    <Icon icon=info.tag.icondata() />
                                </div>
                            }
                        },
                        TxnDirection::Transaction => {
                            view! {
                                <div class="flex items-center justify-center w-7 h-7 lg:w-10 lg:h-10 rounded-md text-white bg-blue-600/5 text-lg lg:text-xl">
                                    <Icon icon=info.tag.icondata() />
                                </div>
                            }
                        },
                    }
                }
                <div class="flex flex-col">
                    <span class="text-md md:text-lg font-semibold text-white">
                        {info.tag.to_text()}
                    </span>
                    {
                        move || {
                            match info.tag{
                                TxnInfoType::Mint { to } => {
                                    match params.get(){
                                        Ok(_) => None,
                                        Err(_) => Some(view! {<div class="text-sm md:text-md text-white/50">{format!("To: {}", to)}</div>})
                                    }
                                },
                                TxnInfoType::Burn { from } => {
                                    match params.get(){
                                        Ok(_) => None,
                                        Err(_) => Some(view! {<div class="text-sm md:text-md text-white/50">{format!("From: {}", from)}</div>})
                                    }
                                },
                                TxnInfoType::Received { from } => Some(view! {<div class="text-sm md:text-md text-white/50">{format!("From: {}", from)}</div>}),
                                TxnInfoType::Sent { to } => Some(view! {<div class="text-sm md:text-md text-white/50">{format!("To: {}", to)}</div>}),
                                TxnInfoType::Transfer { from, to } => Some(view! {
                                    <div class="flex flex-col space-y-1">
                                    <div class="text-sm md:text-md text-white/50">{format!("From: {}", from)}</div>
                                    <div class="text-sm md:text-md text-white/50">{format!("To: {}", to)}</div>
                                    </div>
                                })
                            }
                        }
                    }
                </div>
            </div>
            <div class="flex flex-col top-0 text-right">
            <span class=move || {
                match direction {
                    TxnDirection::Added => "text-green-600 font-semibold",
                    _ => "text-white font-semibold",
                }
            }>{format!("{} {}", bal_res, symbol)}</span>
            <span class="text-sm md:text-md text-white/50">
                {parse_ns_to_datetime(info.timestamp).ok()}
            </span>
            </div>
        </div>
    }
}

pub mod provider {

    use candid::Principal;

    use crate::{component::infinite_scroller::CursoredDataProvider, state::canisters::Canisters};

    use super::*;

    pub(crate) fn get_history_provider(
        canisters: Canisters<false>,
        user_principal: Option<Principal>,
        source: IndexOrLedger,
    ) -> impl CursoredDataProvider<Data = TxnInfoWallet> + Clone {
        #[cfg(feature = "mock-wallet-history")]
        {
            _ = canisters;
            _ = user_principal;
            _ = source;
            mock::MockHistoryProvider
        }
        #[cfg(not(feature = "mock-wallet-history"))]
        {
            canister::TxnHistory {
                canisters,
                user_principal,
                source,
            }
        }
    }

    #[cfg(not(feature = "mock-wallet-history"))]
    mod canister {
        use super::{
            Canisters, CursoredDataProvider, IndexOrLedger, TokenBalance, TxnInfoType,
            TxnInfoWallet,
        };
        use crate::component::infinite_scroller::PageEntry;
        use candid::{Nat, Principal};
        use ic_agent::AgentError;
        use leptos::ServerFnError;
        use yral_canisters_client::{
            sns_index::{
                Account, GetAccountTransactionsArgs, GetTransactionsResult, Transaction,
                TransactionWithId,
            },
            sns_ledger::{self, GetTransactionsRequest, SnsLedger},
        };

        async fn recursively_fetch_transactions<'a>(
            ledger: SnsLedger<'a>,
            start: u32,
        ) -> Result<Vec<sns_ledger::Transaction>, ServerFnError> {
            let mut transactions = Vec::new();
            let mut start = start;
            loop {
                let history = ledger
                    .get_transactions(GetTransactionsRequest {
                        start: start.into(),
                        length: 1000u32.into(),
                    })
                    .await?;
                transactions.extend(history.transactions);
                if history.log_length < 1000u32 {
                    break;
                }
                start += 1000u32;
            }
            Ok(transactions)
        }

        fn parse_transactions(
            txn: TransactionWithId,
            user_principal: Principal,
        ) -> Result<TxnInfoWallet, ServerFnError> {
            let timestamp = txn.transaction.timestamp;
            let id = txn.id.0.to_u64_digits()[0];

            match txn.transaction {
                Transaction {
                    mint: Some(mint), ..
                } => Ok(TxnInfoWallet {
                    tag: TxnInfoType::Mint { to: mint.to.owner },
                    timestamp,
                    amount: TokenBalance::new_cdao(mint.amount),
                    id,
                }),
                Transaction {
                    burn: Some(burn), ..
                } => Ok(TxnInfoWallet {
                    tag: TxnInfoType::Burn {
                        from: user_principal,
                    },
                    timestamp,
                    amount: TokenBalance::new_cdao(burn.amount),
                    id,
                }),
                Transaction {
                    transfer: Some(transfer),
                    ..
                } => {
                    if user_principal == transfer.from.owner {
                        // User is sending funds
                        Ok(TxnInfoWallet {
                            tag: TxnInfoType::Sent {
                                to: transfer.to.owner,
                            },
                            timestamp,
                            amount: TokenBalance::new_cdao(transfer.amount),
                            id,
                        })
                    } else if user_principal == transfer.to.owner {
                        // User is receiving funds
                        Ok(TxnInfoWallet {
                            tag: TxnInfoType::Received {
                                from: transfer.from.owner,
                            },
                            timestamp,
                            amount: TokenBalance::new_cdao(transfer.amount),
                            id,
                        })
                    } else {
                        Err(ServerFnError::new(
                            "Transfer details do not match the user principal",
                        ))
                    }
                }
                _ => Err(ServerFnError::new("Unable to parse transaction details")),
            }
        }

        fn parse_transactions_ledger(
            txn: yral_canisters_client::sns_ledger::Transaction,
            id: u64,
        ) -> Result<TxnInfoWallet, ServerFnError> {
            let timestamp = txn.timestamp;

            match txn {
                yral_canisters_client::sns_ledger::Transaction {
                    mint: Some(mint), ..
                } => Ok(TxnInfoWallet {
                    tag: TxnInfoType::Mint { to: mint.to.owner },
                    timestamp,
                    amount: TokenBalance::new_cdao(mint.amount),
                    id,
                }),
                yral_canisters_client::sns_ledger::Transaction {
                    burn: Some(burn), ..
                } => Ok(TxnInfoWallet {
                    tag: TxnInfoType::Burn {
                        from: burn.from.owner,
                    },
                    timestamp,
                    amount: TokenBalance::new_cdao(burn.amount),
                    id,
                }),
                yral_canisters_client::sns_ledger::Transaction {
                    transfer: Some(transfer),
                    ..
                } => Ok(TxnInfoWallet {
                    tag: TxnInfoType::Transfer {
                        from: transfer.from.owner,
                        to: transfer.to.owner,
                    },
                    timestamp,
                    amount: TokenBalance::new_cdao(transfer.amount),
                    id,
                }),
                _ => Err(ServerFnError::new("Unable to parse transaction details")),
            }
        }

        #[derive(Clone)]
        pub struct TxnHistory {
            pub canisters: Canisters<false>,
            pub user_principal: Option<Principal>,
            pub source: IndexOrLedger,
        }

        impl CursoredDataProvider for TxnHistory {
            type Data = TxnInfoWallet;
            type Error = AgentError;

            async fn get_by_cursor(
                &self,
                start: usize,
                end: usize,
            ) -> Result<PageEntry<TxnInfoWallet>, AgentError> {
                match &self.source {
                    IndexOrLedger::Index(index) => {
                        let index_canister = self.canisters.sns_index(*index).await;

                        let Some(user_principal) = self.user_principal else {
                            return Err(AgentError::PrincipalError(
                                ic_agent::export::PrincipalError::CheckSequenceNotMatch(),
                            ));
                        };

                        // Fetch transactions up to the 'end' index
                        let max_results = end; // Fetch enough transactions to cover 'end'

                        let history = index_canister
                            .get_account_transactions(GetAccountTransactionsArgs {
                                max_results: Nat::from(max_results),
                                start: None, // No cursor, fetch the latest transactions
                                account: Account {
                                    owner: user_principal,
                                    subaccount: None,
                                },
                            })
                            .await?;

                        let transactions = match history {
                            GetTransactionsResult::Ok(v) => v.transactions,
                            GetTransactionsResult::Err(_) => {
                                return Err(AgentError::PrincipalError(
                                    ic_agent::export::PrincipalError::CheckSequenceNotMatch(),
                                ));
                            }
                        };

                        let transactions = transactions.into_iter().skip(start).take(end - start);
                        let txns_len = transactions.len();
                        let data: Vec<TxnInfoWallet> = transactions
                            .filter_map(|txn| parse_transactions(txn, user_principal).ok())
                            .collect();

                        let is_end = txns_len < (end - start);

                        Ok(PageEntry { data, end: is_end })
                    }
                    IndexOrLedger::Ledger(ledger) => {
                        let ledger = self.canisters.sns_ledger(*ledger).await;
                        // let history = ledger
                        //     .get_transactions(GetTransactionsRequest {
                        //         start: start.into(),
                        //         length: (end - start).into(),
                        //     })
                        //     .await?;
                        // let list_end = history.log_length < (end - start);
                        // Ok(PageEntry {
                        //     data: history
                        //         .transactions
                        //         .into_iter()
                        //         .enumerate()
                        //         .filter_map(|(i, txn)| {
                        //             let idx = (history.first_index.clone() + i).0.to_u64_digits();
                        //             if idx.is_empty() {
                        //                 None
                        //             } else {
                        //                 parse_transactions_ledger(txn, idx[0]).ok()
                        //             }
                        //         })
                        //         .collect(),
                        //     end: list_end,
                        // })

                        let history = recursively_fetch_transactions(ledger, 0)
                            .await
                            .map_err(|e| AgentError::MessageError(e.to_string()))?;

                        Ok(PageEntry {
                            data: history
                                .into_iter()
                                .enumerate()
                                .filter_map(|(i, txn)| {
                                    parse_transactions_ledger(txn, i as u64).ok()
                                })
                                .rev()
                                .collect(),
                            end: true,
                        })
                    }
                }
            }
        }
    }

    #[cfg(feature = "mock-wallet-history")]
    mod mock {
        use std::convert::Infallible;

        use rand_chacha::{
            rand_core::{RngCore, SeedableRng},
            ChaCha8Rng,
        };

        use crate::{component::infinite_scroller::PageEntry, utils::time::current_epoch};

        use super::*;

        #[derive(Clone, Copy)]
        pub struct MockHistoryProvider;

        fn tag_from_u32(v: u32) -> TxnInfoType {
            match v % 4 {
                0 => TxnInfoType::Mint {
                    to: Principal::anonymous(),
                },
                1 => TxnInfoType::Burn {
                    from: Principal::anonymous(),
                },
                2 => TxnInfoType::Received {
                    from: Principal::anonymous(),
                },
                3 => TxnInfoType::Sent {
                    to: Principal::anonymous(),
                },
                4 => TxnInfoType::Transfer {
                    from: Principal::anonymous(),
                    to: Principal::anonymous(),
                },
                _ => unreachable!(),
            }
        }
        impl CursoredDataProvider for MockHistoryProvider {
            type Data = TxnInfoWallet;
            type Error = Infallible;

            async fn get_by_cursor(
                &self,
                from: usize,
                end: usize,
            ) -> Result<PageEntry<TxnInfoWallet>, Infallible> {
                let mut rand_gen = ChaCha8Rng::seed_from_u64(current_epoch().as_nanos() as u64);
                let data = (from..end)
                    .map(|_| TxnInfoWallet {
                        amount: TokenBalance::new_cdao((rand_gen.next_u64() % 3001).into()),
                        timestamp: rand_gen.next_u64(),
                        tag: tag_from_u32(rand_gen.next_u32()),
                        id: rand_gen.next_u64(),
                    })
                    .collect();
                Ok(PageEntry { data, end: false })
            }
        }
    }
}
