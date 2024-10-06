use std::fmt::{self, Display, Formatter};

use candid::Principal;
use leptos::*;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};

use crate::component::infinite_scroller::KeyedData;

#[derive(Clone, Copy)]
pub enum TxnDirection {
    Bonus,
    Added,
    Deducted,
}
#[derive(Clone)]
pub enum IndexOrLedger {
    Index(Principal),
    Ledger(Principal),
}
impl TxnDirection {
    fn positive(self) -> bool {
        use TxnDirection::*;
        match self {
            Bonus => true,
            Added => true,
            Deducted => false,
        }
    }
}

impl From<TxnDirection> for &'static icondata_core::IconData {
    fn from(val: TxnDirection) -> Self {
        use TxnDirection::*;
        match val {
            Bonus => icondata::AiPlusCircleOutlined,
            Added => icondata::AiUpCircleOutlined,
            Deducted => icondata::AiDownCircleOutlined,
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TxnInfoType {
    Mint { to: Principal },
    Sent { to: Principal }, // only for keyed
    Burn { from: Principal },
    Received { from: Principal },                // only for keyed
    Transfer { from: Principal, to: Principal }, // only for public transaction
}
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct TxnInfoWallet {
    tag: TxnInfoType,
    timestamp: u64,
    amount: u64,
    id: u64,
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
            _ => unimplemented!(),
        }
    }
}

impl TxnInfoType {
    fn to_text(self) -> &'static str {
        match self {
            TxnInfoType::Burn { .. } => "Burned",
            TxnInfoType::Mint { .. } => "Minted",
            TxnInfoType::Received { .. } => "Sent",
            TxnInfoType::Sent { .. } => "Received",
            TxnInfoType::Transfer { .. } => "Transfered",
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
pub fn TxnView(info: TxnInfoWallet, #[prop(optional)] _ref: NodeRef<html::Div>) -> impl IntoView {
    let direction = TxnDirection::from(info.tag);
    let bal_res = format!(
        "{} {}",
        if direction.positive() { "+" } else { "-" },
        info.amount
    );

    view! {
        <div _ref=_ref class="grid grid-cols-2 grid-rows-1 w-full items-center py-4">
            <div class="flex flex-row gap-2">
                <div class="grid grid-cols-1 place-items-center place-content-center p-2 rounded-full text-primary-600 text-xl lg:text-2xl">
                    <Icon icon=info.tag.icondata() />
                </div>
                <div class="flex flex-col">
                    <span class="text-md md:text-lg font-semibold text-white">
                        {info.tag.to_text()}
                    </span>
                    <span class="text-sm md:text-md text-white/50">{info.amount}COYNs</span>
                </div>
            </div>
            <span class=move || {
                if direction.positive() {
                    "text-green-600 justify-self-end"
                } else {
                    "text-red-600 justify-self-end"
                }
            }>{bal_res}COYNs</span>
        </div>
    }
}

pub mod provider {

    use candid::Principal;

    use crate::{component::infinite_scroller::CursoredDataProvider, state::canisters::Canisters};

    use super::*;
    #[allow(private_interfaces)]
    pub fn get_history_provider(
        canisters: Canisters<false>,
        user_principal: Option<Principal>,
        source: IndexOrLedger,
    ) -> impl CursoredDataProvider<Data = TxnInfoWallet> + Clone {
        // #[cfg(feature = "mock-wallet-history")]
        // {
        //     _ = canisters;
        //     _ = user_canister;
        //     mock::MockHistoryProvider
        // }
        // #[cfg(not(feature = "mock-wallet-history"))]
        {
            canister::TxnHistory {
                canisters,
                user_principal,
                source,
            }
        }
    }
    // #[cfg(not(feature = "mock-wallet-history"))]
    mod canister {
        use super::{Canisters, CursoredDataProvider, IndexOrLedger, TxnInfoType, TxnInfoWallet};
        use crate::component::infinite_scroller::PageEntry;
        use candid::Principal;
        use ic_agent::AgentError;
        use leptos::ServerFnError;
        use yral_canisters_client::{
            sns_index::{
                Account, GetAccountTransactionsArgs, GetTransactionsResult, Transaction,
                TransactionWithId,
            },
            sns_ledger::GetTransactionsRequest,
        };

        fn parse_transactions(
            txn: TransactionWithId,
            user_principal: Principal,
        ) -> Result<TxnInfoWallet, ServerFnError> {
            let timestamp = txn.transaction.timestamp;
            let id = txn.id.0.to_u64_digits()[0]; // some weird hack need to do this properly

            match txn.transaction {
                Transaction {
                    mint: Some(mint), ..
                } => Ok(TxnInfoWallet {
                    tag: TxnInfoType::Mint { to: mint.to.owner },
                    timestamp,
                    amount: mint.amount.0.to_u64_digits()[0],
                    id,
                }),
                Transaction {
                    burn: Some(burn), ..
                } => Ok(TxnInfoWallet {
                    tag: TxnInfoType::Burn {
                        from: user_principal,
                    },
                    timestamp,
                    amount: burn.amount.0.to_u64_digits()[0],
                    id,
                }),
                Transaction {
                    transfer: Some(transfer),
                    ..
                } => {
                    if user_principal == transfer.to.owner {
                        Ok(TxnInfoWallet {
                            tag: TxnInfoType::Sent {
                                to: transfer.to.owner,
                            },
                            timestamp,
                            amount: transfer.amount.0.to_u64_digits()[0],
                            id,
                        })
                    } else if user_principal == transfer.from.owner {
                        Ok(TxnInfoWallet {
                            tag: TxnInfoType::Received {
                                from: transfer.from.owner,
                            },
                            timestamp,
                            amount: transfer.amount.0.to_u64_digits()[0],
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
                    amount: mint.amount.0.to_u64_digits()[0],
                    id,
                }),
                yral_canisters_client::sns_ledger::Transaction {
                    burn: Some(burn), ..
                } => Ok(TxnInfoWallet {
                    tag: TxnInfoType::Burn {
                        from: burn.from.owner,
                    },
                    timestamp,
                    amount: burn.amount.0.to_u64_digits()[0],
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
                    amount: transfer.amount.0.to_u64_digits()[0],
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
                        let index = self.canisters.sns_index(*index).await;
                        let Some(user_principal) = self.user_principal else {
                            return Err(AgentError::PrincipalError(
                                ic_agent::export::PrincipalError::CheckSequenceNotMatch(),
                            ));
                        };
                        let history = index
                            .get_account_transactions(GetAccountTransactionsArgs {
                                max_results: (end - start).into(),
                                start: Some(start.into()),
                                account: Account {
                                    owner: user_principal,
                                    subaccount: None,
                                },
                            })
                            .await?;

                        let history = match history {
                            GetTransactionsResult::Ok(v) => v.transactions,
                            GetTransactionsResult::Err(_) => vec![],
                        };

                        let list_end = history.len() < (end - start);
                        Ok(PageEntry {
                            data: history
                                .into_iter()
                                .filter_map(|txn| parse_transactions(txn, user_principal).ok())
                                .collect(),
                            end: list_end,
                        })
                    }
                    IndexOrLedger::Ledger(ledger) => {
                        let ledger = self.canisters.sns_ledger(*ledger).await;
                        let history = ledger
                            .get_transactions(GetTransactionsRequest {
                                start: start.into(),
                                length: (end - start).into(),
                            })
                            .await?;
                        let list_end = history.log_length < (end - start);
                        Ok(PageEntry {
                            data: history
                                .transactions
                                .into_iter()
                                .enumerate()
                                .filter_map(|(i, txn)| {
                                    parse_transactions_ledger(
                                        txn,
                                        (history.first_index.clone() + i).0.to_u64_digits()[0],
                                    )
                                    .ok()
                                })
                                .collect(),
                            end: list_end,
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
            match v % 3 {
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
                        amount: rand_gen.next_u64() % 3001,
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
