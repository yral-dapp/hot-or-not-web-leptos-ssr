use candid::{Nat, Principal};
use leptos::Params;
use leptos_router::Params;
use yral_pump_n_dump_common::rest::UserBetsResponse;

use crate::consts::PUMP_AND_DUMP_WORKER_URL;

/// Convert e8s to gdolr
/// Backend returns dolr in e8s, and 1dolr = 100gdolr
pub(super) fn convert_e8s_to_gdolr(num: Nat) -> u128 {
    (num * 100u64 / 10u64.pow(8))
        .0
        .try_into()
        .expect("gdolr, scoped at individual player, to be small enough to fit in a u128")
}

/// Estimates the player count based on the count returned by the server
///
/// Logarithimically inflates the count
fn estimate_player_count(num: u64) -> u64 {
    let x = num as f64;
    let res = x + 4.0 + 20.0 * ((x.sqrt() + 2.).log10());
    res.round() as u64
}

/// The data that is required when game is being played by the user
///
/// This data is kept out of GameState so that mutating pumps and dumps doesn't
/// cause the whole game card to rerender
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct GameRunningData {
    pub(super) pumps: u64,
    pub(super) dumps: u64,
    pub(super) winning_pot: Option<u64>,
    pub(super) player_count: u64,
}

/// The current state of the game
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub(super) enum GameState {
    Playing,
    ResultDeclared(GameResult),
}

impl GameState {
    /// Get the winnings if the game result is declared to be a win
    pub(super) fn winnings(&self) -> Option<u128> {
        match self {
            GameState::ResultDeclared(GameResult::Win { amount }) => Some(*amount),
            _ => None,
        }
    }

    /// Get the amount the user lost if the game result is declared to be loss
    pub(super) fn lossings(&self) -> Option<u128> {
        match self {
            GameState::ResultDeclared(GameResult::Loss { amount }) => Some(*amount),
            _ => None,
        }
    }

    /// Has the player lost
    pub(super) fn has_lost(&self) -> bool {
        matches!(self, GameState::ResultDeclared(GameResult::Loss { .. }))
    }

    /// Has the player won
    pub(super) fn has_won(&self) -> bool {
        matches!(self, GameState::ResultDeclared(GameResult::Win { .. }))
    }

    /// Is the game running
    pub(super) fn is_running(&self) -> bool {
        matches!(self, GameState::Playing)
    }
}

/// The result of the game, can never be draw
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub(super) enum GameResult {
    Win { amount: u128 },
    Loss { amount: u128 },
}

impl GameState {
    /// Load the current game state for the given token root and owner
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
            player_count: estimate_player_count(player_count),
            winning_pot,
        }
    }

    /// Load the game running data from the server
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

/// The player's overarching stats
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

    /// Load the user's stats from the server
    pub(super) async fn load(user_canister: Principal) -> Result<Self, String> {
        let balance_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/balance/{user_canister}"))
            .expect("Url to be valid");
        let games_count_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/game_count/{user_canister}"))
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

/// Query parameters for when user click on the card in profile section
#[derive(Debug, Params, PartialEq, Clone)]
pub(super) struct CardQuery {
    pub(super) root: Principal,
    pub(super) state: String,
    pub(super) amount: u128,
}

impl CardQuery {
    /// Check whether the query parameters are coherent
    pub(super) fn is_valid(&self) -> bool {
        let Self { state, .. } = self;
        // only win and loss states are allowed currently
        matches!(state.as_str(), "win" | "loss")
    }

    /// Parses out the details necessary for showing game card
    pub(super) fn details(&self) -> Option<(Principal, GameResult)> {
        let Self {
            root,
            state,
            amount,
        } = self;
        match state.as_str() {
            "win" => Some((*root, GameResult::Win { amount: *amount })),
            "loss" => Some((*root, GameResult::Loss { amount: *amount })),
            _ => None,
        }
    }
}
