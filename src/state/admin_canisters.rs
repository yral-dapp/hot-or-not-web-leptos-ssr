use candid::Principal;
use ic_agent::{AgentError, Identity};
use leptos::expect_context;

use crate::{
    canister::{individual_user_template::IndividualUserTemplate, user_index::UserIndex},
    utils::ic::AgentWrapper,
};

#[derive(Clone)]
pub struct AdminCanisters {
    agent: AgentWrapper,
}

impl AdminCanisters {
    pub fn new(key: impl Identity + 'static) -> Self {
        Self {
            agent: AgentWrapper::build(|b| b.with_identity(key)),
        }
    }

    pub async fn user_index_with(
        &self,
        idx_principal: Principal,
    ) -> Result<UserIndex<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(UserIndex(idx_principal, agent))
    }

    pub async fn individual_user_for(
        &self,
        user_canister: Principal,
    ) -> Result<IndividualUserTemplate<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(IndividualUserTemplate(user_canister, agent))
    }
}

pub fn admin_canisters() -> AdminCanisters {
    expect_context()
}
