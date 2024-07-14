use candid::Principal;
use ic_agent::Identity;
use leptos::expect_context;

use super::canisters::build_agent;
use crate::canister::{individual_user_template::IndividualUserTemplate, user_index::UserIndex};

#[derive(Clone)]
pub struct AdminCanisters {
    agent: ic_agent::Agent,
}

impl AdminCanisters {
    pub fn new(key: impl Identity + 'static) -> Self {
        Self {
            agent: build_agent(|b| b.with_identity(key)),
        }
    }

    pub fn user_index_with(&self, idx_principal: Principal) -> UserIndex<'_> {
        UserIndex(idx_principal, &self.agent)
    }

    pub fn individual_user_for(&self, user_canister: Principal) -> IndividualUserTemplate<'_> {
        IndividualUserTemplate(user_canister, &self.agent)
    }
}

pub fn admin_canisters() -> AdminCanisters {
    expect_context()
}
