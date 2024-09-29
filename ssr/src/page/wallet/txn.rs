use std::fmt::{self, Display, Formatter};

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
pub enum TxnTag {
    BetPlaced,
    SignupBonus,
    Referral,
    Winnings,
    Commission,
    Transfer,
    HotorNotAccountTransfer,
}

impl From<TxnTag> for TxnDirection {
    fn from(value: TxnTag) -> TxnDirection {
        use TxnTag::*;
        match value {
            BetPlaced | Transfer => TxnDirection::Deducted,
            Winnings | Commission | HotorNotAccountTransfer => TxnDirection::Added,
            SignupBonus | Referral => TxnDirection::Bonus,
        }
    }
}

impl TxnTag {
    fn to_text(self) -> &'static str {
        use TxnTag::*;
        match self {
            BetPlaced => "Vote Placement",
            SignupBonus => "Joining Bonus",
            Referral => "Referral Reward",
            Winnings => "Vote Winnings",
            Commission => "Vote Commission",
            Transfer => "Transfer",
            HotorNotAccountTransfer => "HotorNot Account Transfer",
        }
    }

    fn icondata(self) -> &'static icondata_core::IconData {
        TxnDirection::from(self).into()
    }
}

impl Display for TxnTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_text())
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct TxnInfo {
    pub tag: TxnTag,
    pub amount: u64,
    pub id: u64,
}

impl KeyedData for TxnInfo {
    type Key = u64;

    fn key(&self) -> Self::Key {
        self.id
    }
}

#[component]
pub fn TxnView(info: TxnInfo, #[prop(optional)] _ref: NodeRef<html::Div>) -> impl IntoView {
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
                    <Icon icon=info.tag.icondata()/>
                </div>
                <div class="flex flex-col">
                    <span class="text-md md:text-lg font-semibold text-white">
                        {info.tag.to_text()}
                    </span>
                    <span class="text-sm md:text-md text-white/50">{info.amount} COYNs</span>
                </div>
            </div>
            <span class=move || {
                if direction.positive() {
                    "text-green-600 justify-self-end"
                } else {
                    "text-red-600 justify-self-end"
                }
            }>{bal_res} COYNs</span>
        </div>
    }
}

pub mod provider {
    use yral_canisters_client::individual_user_template::IndividualUserTemplate;

    use crate::{
        component::infinite_scroller::{CursoredDataProvider, KeyedCursoredDataProvider},
        state::canisters::Canisters,
    };

    use super::*;

    pub fn get_history_provider<'a>(
        canisters: Canisters<true>,
    ) -> impl KeyedCursoredDataProvider<IndividualUserTemplate<'a>, Data = TxnInfo> + Clone {
        #[cfg(feature = "mock-wallet-history")]
        {
            _ = canisters;
            mock::MockHistoryProvider
        }
        #[cfg(not(feature = "mock-wallet-history"))]
        {
            canister::TxnHistory(canisters)
        }
    }

    #[cfg(not(feature = "mock-wallet-history"))]
    mod canister {
        use super::{Canisters, CursoredDataProvider, TxnInfo, TxnTag};
        use crate::component::infinite_scroller::{KeyedCursoredDataProvider, PageEntry};
        use ic_agent::AgentError;
        use yral_canisters_client::individual_user_template::{
            HotOrNotOutcomePayoutEvent, IndividualUserTemplate, MintEvent, Result15, TokenEvent,
        };

        fn event_to_txn(event: (u64, TokenEvent)) -> Option<TxnInfo> {
            let (amount, tag) = match event.1 {
                TokenEvent::Stake { amount, .. } => (amount, TxnTag::BetPlaced),
                TokenEvent::Burn => return None,
                TokenEvent::Mint {
                    amount,
                    details: MintEvent::NewUserSignup { .. },
                    ..
                } => (amount, TxnTag::SignupBonus),
                TokenEvent::Mint {
                    amount,
                    details: MintEvent::Referral { .. },
                    ..
                } => (amount, TxnTag::Referral),
                TokenEvent::Transfer { amount, .. } => (amount, TxnTag::Transfer),
                TokenEvent::Receive { amount, .. } => (amount, TxnTag::HotorNotAccountTransfer),
                TokenEvent::HotOrNotOutcomePayout {
                    amount,
                    details: HotOrNotOutcomePayoutEvent::CommissionFromHotOrNotBet { .. },
                    ..
                } => (amount, TxnTag::Commission),
                TokenEvent::HotOrNotOutcomePayout {
                    amount,
                    details: HotOrNotOutcomePayoutEvent::WinningsEarnedFromBet { .. },
                    ..
                } => (amount, TxnTag::Winnings),
            };

            Some(TxnInfo {
                tag,
                amount,
                id: event.0,
            })
        }

        #[derive(Clone)]
        pub struct TxnHistory(pub Canisters<true>);
        impl<'a> KeyedCursoredDataProvider<IndividualUserTemplate<'a>> for TxnHistory {
            async fn get_by_cursor_by_key(
                &self,
                start: usize,
                end: usize,
                user: IndividualUserTemplate<'a>,
            ) -> Result<PageEntry<Self::Data>, Self::Error> {
                let history = user
                    .get_user_utility_token_transaction_history_with_pagination(
                        start as u64,
                        end as u64,
                    )
                    .await?;
                let history = match history {
                    Result15::Ok(v) => v,
                    Result15::Err(_) => vec![],
                };
                let list_end = history.len() < (end - start);
                Ok(PageEntry {
                    data: history.into_iter().filter_map(event_to_txn).collect(),
                    end: list_end,
                })
            }
        }
        impl CursoredDataProvider for TxnHistory {
            type Data = TxnInfo;
            type Error = AgentError;

            async fn get_by_cursor(
                &self,
                start: usize,
                end: usize,
            ) -> Result<PageEntry<TxnInfo>, AgentError> {
                let user = self.0.authenticated_user().await;
                let history = user
                    .get_user_utility_token_transaction_history_with_pagination(
                        start as u64,
                        end as u64,
                    )
                    .await?;
                let history = match history {
                    Result15::Ok(v) => v,
                    Result15::Err(_) => vec![],
                };
                let list_end = history.len() < (end - start);
                Ok(PageEntry {
                    data: history.into_iter().filter_map(event_to_txn).collect(),
                    end: list_end,
                })
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
        use yral_canisters_client::individual_user_template::IndividualUserTemplate;

        use crate::{
            component::infinite_scroller::{KeyedCursoredDataProvider, PageEntry},
            utils::time::current_epoch,
        };

        use super::*;

        #[derive(Clone, Copy)]
        pub struct MockHistoryProvider;

        fn tag_from_u32(v: u32) -> TxnTag {
            match v % 5 {
                0 => TxnTag::BetPlaced,
                1 => TxnTag::SignupBonus,
                2 => TxnTag::Referral,
                3 => TxnTag::Winnings,
                4 => TxnTag::Commission,
                _ => unreachable!(),
            }
        }
        impl<'a> KeyedCursoredDataProvider<IndividualUserTemplate<'a>> for MockHistoryProvider {
            async fn get_by_cursor_by_key(
                &self,
                start: usize,
                end: usize,
                user: IndividualUserTemplate<'a>,
            ) -> Result<PageEntry<Self::Data>, Self::Error> {
                let mut rand_gen = ChaCha8Rng::seed_from_u64(current_epoch().as_nanos() as u64);
                let data = (start..end)
                    .map(|_| TxnInfo {
                        amount: rand_gen.next_u64() % 3001,
                        tag: tag_from_u32(rand_gen.next_u32()),
                        id: rand_gen.next_u64(),
                    })
                    .collect();
                Ok(PageEntry { data, end: false })
            }
        }
        impl CursoredDataProvider for MockHistoryProvider {
            type Data = TxnInfo;
            type Error = Infallible;

            async fn get_by_cursor(
                &self,
                from: usize,
                end: usize,
            ) -> Result<PageEntry<TxnInfo>, Infallible> {
                let mut rand_gen = ChaCha8Rng::seed_from_u64(current_epoch().as_nanos() as u64);
                let data = (from..end)
                    .map(|_| TxnInfo {
                        amount: rand_gen.next_u64() % 3001,
                        tag: tag_from_u32(rand_gen.next_u32()),
                        id: rand_gen.next_u64(),
                    })
                    .collect();
                Ok(PageEntry { data, end: false })
            }
        }
    }
}
