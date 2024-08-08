use candid::Principal;
use leptos::ServerFnError;

use crate::{
    canister::individual_user_template::{self, PlaceBetArg , BettingStatus, BetDirection, Result_ },
    state::canisters::Canisters
};
// use crate::canister::individual_user_template::{self, PlaceBetArg, types::arg::PlaceBetArg};


#[derive(Clone, Debug,  PartialEq)]
pub enum CoinStates {
    C50,
    C100,
    C200
}

use std::fmt;
// todo remove this debug
impl fmt::Display for CoinStates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            CoinStates::C50 => "50",
            CoinStates::C100 => "100",
            CoinStates::C200 => "200",
        };
        write!(f, "{}", value)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MyBetDirection {
    Hot,
    Not,
}

impl From<BetDirection> for MyBetDirection {
    fn from(direction: BetDirection) -> Self {
        match direction {
            BetDirection::Hot => MyBetDirection::Hot,
            BetDirection::Not => MyBetDirection::Not,
        }
    }
}

impl From<MyBetDirection> for BetDirection {
     fn from(direction: MyBetDirection) -> Self {
        match direction {
            MyBetDirection::Hot => BetDirection::Hot,
            MyBetDirection::Not => BetDirection::Not,
        }
    }
}

pub async fn bet_on_currently_viewing_post_fe(
    canisters: Canisters<true>,
    bet_amount: u64, 
    bet_direction: BetDirection, 
    post_id: u64, 
    post_canister_id: Principal) 
    -> Result<BettingStatus, ServerFnError> {
    let user = canisters.authenticated_user().await?;

    let place_bet_arg = PlaceBetArg {
        bet_amount,
        post_id,
        bet_direction,
        post_canister_id,
    };
 
    let res = user.bet_on_currently_viewing_post(place_bet_arg).await?;

    Ok(BettingStatus::BettingClosed)
}