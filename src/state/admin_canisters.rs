use candid::Principal;
use ic_agent::Identity;
use leptos::expect_context;

use crate::canister::{
    individual_user_template::IndividualUserTemplate, user_index::UserIndex, AGENT_URL,
};

#[derive(Clone)]
pub struct AdminCanisters {
    agent: ic_agent::Agent,
}

impl AdminCanisters {
    pub fn new(key: impl Identity + 'static) -> Self {
        Self {
            agent: ic_agent::Agent::builder()
                .with_url(AGENT_URL)
                .with_identity(key)
                .build()
                .unwrap(),
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
