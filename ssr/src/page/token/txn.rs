use crate::component::infinite_scroller::KeyedData;

#[derive(Clone)]
pub struct TokenTxn {
    id: u128,
    pub kind: String,
    pub created_at_time: u64,
    pub transfer: TrasferDetails,
}

#[derive(Clone)]
pub struct TrasferDetails {
    // pub to_principal: Option<Principal>,
    // pub from_principal: Option<Principal>,
    pub amount: Option<u128>,
    pub is_received: bool,
}

impl KeyedData for TokenTxn {
    type Key = u128;

    fn key(&self) -> Self::Key {
        self.id
    }
}

pub(super) mod provider {
    use crate::canister::sns_index::{Account, GetAccountTransactionsArgs, TransactionWithId};
    use crate::state::canisters::Canisters;

    use crate::component::infinite_scroller::PageEntry;
    use crate::component::infinite_scroller::*;
    use candid::{Nat, Principal};
    use ic_agent::AgentError;

    use super::TokenTxn;

    pub fn get_history_provider(
        cans: Canisters<true>,
        user_prinicipal: Principal,
        index_principal: Principal,
    ) -> impl CursoredDataProvider<Data = TokenTxn> + Clone {
        TokenTxnHistory(cans, user_prinicipal, index_principal)
    }

    #[derive(Clone)]
    pub struct TokenTxnHistory(pub Canisters<true>, Principal, Principal);

    impl super::TokenTxn {
        fn into_token_txn(txn: TransactionWithId, user_prinicipal: Principal) -> Self {
            let id: u128 = txn.id.0.try_into().unwrap();
            let txn = txn.transaction;
            let to_principal = txn.transfer.as_ref().map(|f| f.to.owner.clone());
            // let from_principal: Option<Principal> = txn.transfer.as_ref().map(|f| f.from.owner.clone());
            let amount: Option<_> = txn.transfer.map(|f| f.amount.clone().0.try_into().unwrap());

            Self {
                id,
                kind: txn.kind,
                created_at_time: txn.timestamp,
                transfer: super::TrasferDetails {
                    // to_principal,
                    // from_principal,
                    amount,
                    is_received: to_principal.unwrap() == user_prinicipal
                },
            }
        }
    }

    impl CursoredDataProvider for TokenTxnHistory {
        type Data = super::TokenTxn;
        type Error = AgentError;

        async fn get_by_cursor(
            &self,
            start: usize,
            end: usize,
        ) -> Result<PageEntry<Self::Data>, AgentError> {
            let sns_index = &self.0.sns_index(self.2).await;
            let res = sns_index
                .get_account_transactions(GetAccountTransactionsArgs {
                    max_results: Nat::from((end - start) as u8),
                    account: Account {
                        owner: self.1.clone(),
                        subaccount: None,
                    },
                    start: None,
                })
                .await
                .unwrap();
            match res {
                crate::canister::sns_index::GetTransactionsResult::Ok(get_transactions) => {
                    let list_end = get_transactions.transactions.len() < (end - start);

                    Ok(PageEntry {
                        data: get_transactions
                            .transactions
                            .into_iter()
                            .map(|f| super::TokenTxn::into_token_txn(f, self.1.clone()))
                            .collect(),
                        end: list_end,
                    })
                }
                crate::canister::sns_index::GetTransactionsResult::Err(get_transactions_err) => {
                    Err(AgentError::MessageError(get_transactions_err.message))
                }
            }
        }
    }
}
