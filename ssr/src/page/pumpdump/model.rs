use candid::{Nat, Principal};
use leptos::Params;
use leptos_router::Params;
use yral_pump_n_dump_common::rest::UserBetsResponse;

use crate::consts::PUMP_AND_DUMP_WORKER_URL;

/// Convert e8s to gdolr
/// backend returns dolr in e8s, and 1dolr = 100gdolr
pub(super) fn convert_e8s_to_gdolr(num: Nat) -> u128 {
    (num * 100u64 / 10u64.pow(8))
        .0
        .try_into()
        .expect("gdolr, scoped at individual player, to be small enough to fit in a u128")
}

// this data is kept out of GameState so that mutating pumps and dumps doesn't
// cause the whole game card to rerender
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct GameRunningData {
    pub(super) pumps: u64,
    pub(super) dumps: u64,
    pub(super) winning_pot: Option<u64>,
    pub(super) player_count: u64,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub(super) enum GameState {
    Playing,
    Pending,
    ResultDeclared(GameResult),
}

impl GameState {
    pub(super) fn winnings(&self) -> Option<u128> {
        match self {
            GameState::ResultDeclared(GameResult::Win { amount }) => Some(*amount),
            _ => None,
        }
    }

    pub(super) fn lossings(&self) -> Option<u128> {
        match self {
            GameState::ResultDeclared(GameResult::Loss { amount }) => Some(*amount),
            _ => None,
        }
    }

    pub(super) fn has_lost(&self) -> bool {
        matches!(self, GameState::ResultDeclared(GameResult::Loss { .. }))
    }

    pub(super) fn has_won(&self) -> bool {
        matches!(self, GameState::ResultDeclared(GameResult::Win { .. }))
    }

    pub(super) fn is_running(&self) -> bool {
        matches!(self, GameState::Playing)
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub(super) enum GameResult {
    Win { amount: u128 },
    Loss { amount: u128 },
}

impl GameState {
    pub(super) async fn load(
        _owner_canister: Principal,
        _root_principal: Principal,
    ) -> Result<Self, String> {
        Ok(Self::Playing)
    }
}

impl GameRunningData {
    pub(super) fn new(pumps: u64, dumps: u64, player_count: u64, winning_pot: Option<u64>) -> Self {
        Self {
            pumps,
            dumps,
            player_count,
            winning_pot,
        }
    }

    // #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    pub(super) async fn load(
        owner: Principal,
        token_root: Principal,
        user_canister: Principal,
    ) -> Result<Self, String> {
        let bets_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/bets/{owner}/{token_root}/{user_canister}"))
            .expect("url to be valid");

        let player_count_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/player_count/{owner}/{token_root}"))
            .expect("url to be valid");

        let bets: UserBetsResponse = reqwest::get(bets_url)
            .await
            .map_err(|err| format!("Coulnd't load bets: {err}"))?
            .json()
            .await
            .map_err(|err| format!("Couldn't parse bets out of repsonse: {err}"))?;

        let player_count: u64 = reqwest::get(player_count_url)
            .await
            .map_err(|err| format!("Coulnd't load player count: {err}"))?
            .text()
            .await
            .map_err(|err| format!("Couldn't read response for player count: {err}"))?
            .parse()
            .map_err(|err| format!("Couldn't parse player count from response: {err}"))?;

        // Maybe we should also load winning pot as part of game running data
        Ok(Self::new(bets.pumps, bets.dumps, player_count, None))
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct PlayerData {
    pub(super) games_count: u64,
    pub(super) wallet_balance: u128,
}

impl PlayerData {
    pub(super) fn new(games_count: u64, wallet_balance: u128) -> Self {
        Self {
            games_count,
            wallet_balance,
        }
    }

    pub(super) async fn load(user_principal: Principal) -> Result<Self, String> {
        let balance_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/balance/{user_principal}"))
            .expect("Url to be valid");
        let games_count_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/game_count/{user_principal}"))
            .expect("Url to be valid");

        let games_count: u64 = reqwest::get(games_count_url)
            .await
            .map_err(|_| "Failed to load games count")?
            .text()
            .await
            .map_err(|_| "failed to read response body".to_string())?
            .parse()
            .map_err(|_| "Couldn't parse nat number".to_string())?;

        let wallet_balance: Nat = reqwest::get(balance_url)
            .await
            .map_err(|_| "failed to load balance".to_string())?
            .text()
            .await
            .map_err(|_| "failed to read response body".to_string())?
            .parse()
            .map_err(|_| "Couldn't parse nat number".to_string())?;

        let wallet_balance = convert_e8s_to_gdolr(wallet_balance);

        Ok(Self::new(games_count, wallet_balance))
    }
}

#[derive(Debug, Params, PartialEq, Clone)]
pub(super) struct CardQuery {
    pub(super) root: Option<Principal>,
    pub(super) state: Option<String>,
    pub(super) amount: Option<u128>,
}

impl CardQuery {
    pub(super) fn is_valid(&self) -> bool {
        let Self {
            root,
            state,
            amount,
        } = self;
        matches!(
            (root, state.as_ref().map(|s| s.as_str()), amount),
            (Some(_), Some("win" | "loss"), Some(_))
        )
    }

    pub(super) fn details(&self) -> Option<(Principal, GameResult)> {
        let Self {
            root,
            state,
            amount,
        } = self;
        match (root, state.as_ref().map(|s| s.as_str()), amount) {
            (Some(root), Some("win"), Some(amount)) => {
                Some((*root, GameResult::Win { amount: *amount }))
            }
            (Some(root), Some("loss"), Some(amount)) => {
                Some((*root, GameResult::Loss { amount: *amount }))
            }
            _ => None,
        }
    }
}
