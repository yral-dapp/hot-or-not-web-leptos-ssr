use candid::Principal;
use leptos::prelude::*;
use yral_canisters_common::{utils::profile::ProfileDetails, Canisters};

#[derive(Clone, Debug)]
pub struct UserDetails {
    pub details: ProfileDetails,
    pub canister_id: Principal,
}

impl UserDetails {
    pub fn try_get() -> Option<Self> {
        let cans_store: RwSignal<Option<Canisters<true>>> = use_context()?;
        Self::try_get_from_canister_store(cans_store)
    }

    pub fn try_get_from_canister_store(
        canister_store: RwSignal<Option<Canisters<true>>>,
    ) -> Option<Self> {
        let canisters = canister_store.get_untracked()?;
        let details = canisters.profile_details();

        Some(Self {
            details,
            canister_id: canisters.user_canister(),
        })
    }
}

macro_rules! user_details_or_ret {
    () => {
        if let Some(user) = $crate::user::UserDetails::try_get() {
            user
        } else {
            return;
        }
    };
}

macro_rules! user_details_can_store_or_ret {
    ($e:expr) => {
        if let Some(user) = $crate::user::UserDetails::try_get_from_canister_store($e) {
            user
        } else {
            return;
        }
    };
}

pub(crate) use user_details_can_store_or_ret;
pub(crate) use user_details_or_ret;
