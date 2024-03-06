use std::fmt::{self, Display, Formatter};

use leptos::*;
use leptos_icons::Icon;
use serde::{Deserialize, Serialize};

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
}

impl From<TxnTag> for TxnDirection {
    fn from(value: TxnTag) -> TxnDirection {
        use TxnTag::*;
        match value {
            BetPlaced => TxnDirection::Deducted,
            Winnings | Commission => TxnDirection::Added,
            SignupBonus | Referral => TxnDirection::Bonus,
        }
    }
}

impl TxnTag {
    fn to_text(self) -> &'static str {
        use TxnTag::*;
        match self {
            BetPlaced => "Bet Placement",
            SignupBonus => "Sign Up Bonus",
            Referral => "Referral Reward",
            Winnings => "Bet Winnings",
            Commission => "Bet Commission",
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

#[component]
pub fn TxnView(info: TxnInfo) -> impl IntoView {
    let direction = TxnDirection::from(info.tag);
    let bal_res = format!(
        "{} {}",
        if direction.positive() { "+" } else { "-" },
        info.amount
    );

    view! {
        <div class="grid grid-cols-2 grid-rows-1 w-full items-center py-4">
            <div class="flex flex-row gap-2">
                <div class="grid grid-cols-1 place-items-center place-content-center p-2 rounded-full text-orange-600 text-xl lg:text-2xl">
                    <Icon icon=info.tag.icondata()/>
                </div>
                <div class="flex flex-col">
                    <span class="text-md md:text-lg font-semibold text-white">
                        {info.tag.to_text()}
                    </span>
                    <span class="text-sm md:text-md text-white/50">{info.amount} Coins</span>
                </div>
            </div>
            <span class=move || {
                if direction.positive() {
                    "text-green-600 justify-self-end"
                } else {
                    "text-red-600 justify-self-end"
                }
            }>{bal_res} Coins</span>
        </div>
    }
}

pub mod provider {
    use ic_agent::AgentError;

    use crate::state::canisters::Canisters;

    use super::*;

    pub trait HistoryProvider {
        async fn get_history(
            &self,
            start: u64,
            end: u64,
        ) -> Result<(Vec<TxnInfo>, bool), AgentError>;
    }

    pub fn get_history_provider(canisters: Canisters<true>) -> impl HistoryProvider {
        #[cfg(feature = "mock-wallet-history")]
        {
            _ = canisters;
            mock::MockHistoryProvider
        }
        #[cfg(not(feature = "mock-wallet-history"))]
        {
            canisters
        }
    }

    #[cfg(not(feature = "mock-wallet-history"))]
    mod canister {
        use super::*;
        use crate::canister::individual_user_template::{
            HotOrNotOutcomePayoutEvent, MintEvent, TokenEvent,
        };
        use crate::canister::individual_user_template::Result5;

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
                TokenEvent::Transfer => return None,
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

        impl HistoryProvider for Canisters<true> {
            async fn get_history(
                &self,
                start: u64,
                end: u64,
            ) -> Result<(Vec<TxnInfo>, bool), AgentError> {
                let user = self.authenticated_user();
                let history = user
                    .get_user_utility_token_transaction_history_with_pagination(start, end)
                    .await?;
                let history = match history {
                    Result5::Ok(v) => v,
                    Result5::Err(_) => vec![],
                };
                let list_end = history.len() < (end - start) as usize;
                Ok((
                    history.into_iter().filter_map(event_to_txn).collect(),
                    list_end,
                ))
            }
        }
    }

    #[cfg(feature = "mock-wallet-history")]
    mod mock {
        use rand_chacha::{
            rand_core::{RngCore, SeedableRng},
            ChaCha8Rng,
        };

        use crate::utils::current_epoch;

        use super::*;

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

        impl HistoryProvider for MockHistoryProvider {
            async fn get_history(
                &self,
                from: u64,
                end: u64,
            ) -> Result<(Vec<TxnInfo>, bool), AgentError> {
                let mut rand_gen = ChaCha8Rng::seed_from_u64(current_epoch().as_nanos() as u64);
                Ok((
                    (from..end)
                        .map(|_| TxnInfo {
                            amount: rand_gen.next_u64() % 3001,
                            tag: tag_from_u32(rand_gen.next_u32()),
                            id: rand_gen.next_u64(),
                        })
                        .collect(),
                    false,
                ))
            }
        }
    }
}
