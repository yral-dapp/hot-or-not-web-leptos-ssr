use candid::Principal;
use leptos::ServerFnError;

use crate::{
    canister::individual_user_template::{self, PlaceBetArg , BettingStatus, BetOnCurrentlyViewingPostError, BetDirection, SystemTime, Result_, Result1 },
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

impl fmt::Debug for BettingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BettingStatus::BettingOpen {
                number_of_participants,
                ongoing_room,
                ongoing_slot,
                has_this_user_participated_in_this_post,
                started_at,
            } => f.debug_struct("BettingOpen")
                .field("number_of_participants", number_of_participants)
                .field("ongoing_room", ongoing_room)
                .field("ongoing_slot", ongoing_slot)
                .field("has_this_user_participated_in_this_post", has_this_user_participated_in_this_post)
                .field("started_at", started_at)
                .finish(),
            BettingStatus::BettingClosed => f.write_str("BettingClosed"),
        }
    }
}

impl fmt::Debug for SystemTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SystemTime")
            .field("nanos_since_epoch", &self.nanos_since_epoch)
            .field("secs_since_epoch", &self.secs_since_epoch)
            .finish()
    }
}

impl fmt::Debug for BetOnCurrentlyViewingPostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BetOnCurrentlyViewingPostError::UserPrincipalNotSet => write!(f, "UserPrincipalNotSet"),
            BetOnCurrentlyViewingPostError::InsufficientBalance => write!(f, "InsufficientBalance"),
            BetOnCurrentlyViewingPostError::UserAlreadyParticipatedInThisPost => write!(f, "UserAlreadyParticipatedInThisPost"),
            BetOnCurrentlyViewingPostError::BettingClosed => write!(f, "BettingClosed"),
            BetOnCurrentlyViewingPostError::Unauthorized => write!(f, "Unauthorized"),
            BetOnCurrentlyViewingPostError::PostCreatorCanisterCallFailed => write!(f, "PostCreatorCanisterCallFailed"),
            BetOnCurrentlyViewingPostError::UserNotLoggedIn => write!(f, "UserNotLoggedIn"),
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

    let betting_status = match res {
        Result1::Ok(p) => p,
        Result1::Err(e) => {  
            // todo send event that betting failed
            return Err(ServerFnError::new(format!("bet on bet_on_currently_viewing_post error = {:?}", e)))},
    };

    Ok(betting_status)
}