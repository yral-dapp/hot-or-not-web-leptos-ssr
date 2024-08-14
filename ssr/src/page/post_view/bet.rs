use candid::Principal;
use leptos::ServerFnError;

use crate::{
    canister::individual_user_template::{self, PlaceBetArg , BettingStatus, BetOnCurrentlyViewingPostError, BetDirection, SystemTime, PlacedBetDetail,BetOutcomeForBetMaker,  Result_, Result1 },
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


impl fmt::Debug for PlacedBetDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PlacedBetDetail")
            .field("outcome_received", &self.outcome_received)
            .field("slot_id", &self.slot_id)
            .field("post_id", &self.post_id)
            .field("room_id", &self.room_id)
            .field("canister_id", &self.canister_id)
            .field("bet_direction", &self.bet_direction)
            .field("amount_bet", &self.amount_bet)
            .field("bet_placed_at", &self.bet_placed_at)
            .finish()
    }
}

impl Clone for PlacedBetDetail {
    fn clone(&self) -> Self {
        Self {
            outcome_received: self.outcome_received.clone(),
            slot_id: self.slot_id.clone(),
            post_id: self.post_id.clone(),
            room_id: self.room_id.clone(),
            canister_id: self.canister_id.clone(),
            bet_direction: self.bet_direction.clone(),
            amount_bet: self.amount_bet.clone(),
            bet_placed_at: self.bet_placed_at.clone(),
        }
    }
}


impl Clone for SystemTime {
    fn clone(&self) -> Self {
        Self {
            nanos_since_epoch: self.nanos_since_epoch.clone(),
            secs_since_epoch: self.secs_since_epoch.clone(),
        }
    }
}



// use serde::{Serialize, Deserialize};
// use serde::ser::SerializeStruct;

// impl Serialize for PlacedBetDetail {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("PlacedBetDetail", 7)?;
//         state.serialize_field("outcome_received", &self.outcome_received)?;
//         state.serialize_field("slot_id", &self.slot_id)?;
//         state.serialize_field("post_id", &self.post_id)?;
//         state.serialize_field("room_id", &self.room_id)?;
//         state.serialize_field("canister_id", &self.canister_id)?;
//         state.serialize_field("bet_direction", &self.bet_direction)?;
//         state.serialize_field("amount_bet", &self.amount_bet)?;
//         state.serialize_field("bet_placed_at", &self.bet_placed_at)?;
//         state.end()
//     }
// }


impl fmt::Debug for BetDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BetDirection::Hot => write!(f, "Hot"),
            BetDirection::Not => write!(f, "Not"),
        }
    }
}

impl Clone for BetDirection {
    fn clone(&self) -> Self {
        match self {
            BetDirection::Hot => BetDirection::Hot,
            BetDirection::Not => BetDirection::Not,
        }
    }
}

impl PartialEq for BetDirection {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BetDirection::Hot, BetDirection::Hot) => true,
            (BetDirection::Not, BetDirection::Not) => true,
            _ => false,
        }
    }
}



impl fmt::Debug for BetOutcomeForBetMaker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BetOutcomeForBetMaker::Won(amount) => write!(f, "Won: {:?}", amount),
            BetOutcomeForBetMaker::Draw(amount) => write!(f, "Draw: {:?}", amount),
            BetOutcomeForBetMaker::Lost => write!(f, "Lost"),
            BetOutcomeForBetMaker::AwaitingResult => write!(f, "AwaitingResult"),
        }
    }
}


impl Clone for BetOutcomeForBetMaker {
    fn clone(&self) -> Self {
        match self {
            BetOutcomeForBetMaker::Won(amount) => BetOutcomeForBetMaker::Won(*amount),
            BetOutcomeForBetMaker::Draw(amount) => BetOutcomeForBetMaker::Draw(*amount),
            BetOutcomeForBetMaker::Lost => BetOutcomeForBetMaker::Lost,
            BetOutcomeForBetMaker::AwaitingResult => BetOutcomeForBetMaker::AwaitingResult,
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

    log::info!("bet_on_currently_viewing_post_fe - {} - user: {:?} ", post_id, user.0);

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