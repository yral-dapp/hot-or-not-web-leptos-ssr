use std::{collections::HashSet, sync::Arc};

use candid::Principal;
use ic_agent::{identity::DelegatedIdentity, AgentError, Identity};
use leptos::*;

use crate::{
    canister::{
        individual_user_template::IndividualUserTemplate,
        platform_orchestrator::{self, PlatformOrchestrator},
        post_cache::{self, PostCache},
        user_index::UserIndex,
        AGENT_URL,
    },
    consts::LEGACY_USER_INDEX,
    utils::MockPartialEq,
};

use super::auth::{types::DelegationIdentity, AuthClient, AuthError};
use thiserror::Error;

#[derive(Clone)]
pub struct Canisters<const AUTH: bool> {
    agent: ic_agent::Agent,
    auth_client: AuthClient,
    id: Option<Arc<DelegatedIdentity>>,
    user_canister: Principal,
    expiry: u64,
}

impl Default for Canisters<false> {
    fn default() -> Self {
        Self {
            agent: ic_agent::Agent::builder()
                .with_url(AGENT_URL)
                .build()
                .unwrap(),
            id: None,
            auth_client: AuthClient::default(),
            user_canister: Principal::anonymous(),
            expiry: 0,
        }
    }
}

impl Canisters<true> {
    pub fn authenticated(id: DelegatedIdentity) -> Canisters<true> {
        let expiry = id
            .delegation_chain()
            .iter()
            .fold(u64::MAX, |prev_expiry, del| {
                del.delegation.expiration.min(prev_expiry)
            });
        let id = Arc::new(id);

        Canisters {
            agent: ic_agent::Agent::builder()
                .with_url(AGENT_URL)
                .with_arc_identity(id.clone())
                .build()
                .unwrap(),
            id: Some(id),
            auth_client: AuthClient::default(),
            user_canister: Principal::anonymous(),
            expiry,
        }
    }

    pub fn expiry_ns(&self) -> u64 {
        self.expiry
    }

    pub fn identity(&self) -> &DelegatedIdentity {
        self.id
            .as_ref()
            .expect("Authenticated canisters must have an identity")
    }

    pub fn user_canister(&self) -> Principal {
        self.user_canister
    }

    pub fn authenticated_user(&self) -> IndividualUserTemplate<'_> {
        IndividualUserTemplate(self.user_canister, &self.agent)
    }
}

#[derive(Error, Debug, Clone)]
pub enum CanistersError {
    #[error("Auth service error: {0}")]
    Auth(#[from] AuthError),
    #[error("IC-Agent error: {0}")]
    Agent(String),
}

impl From<AgentError> for CanistersError {
    fn from(e: AgentError) -> Self {
        CanistersError::Agent(e.to_string())
    }
}

impl<const A: bool> Canisters<A> {
    pub fn post_cache(&self) -> PostCache<'_> {
        PostCache(post_cache::CANISTER_ID, &self.agent)
    }

    pub fn individual_user(&self, user_canister: Principal) -> IndividualUserTemplate<'_> {
        IndividualUserTemplate(user_canister, &self.agent)
    }

    pub fn user_index_with(&self, subnet_principal: Principal) -> UserIndex<'_> {
        UserIndex(subnet_principal, &self.agent)
    }

    pub fn orchestrator(&self) -> PlatformOrchestrator<'_> {
        PlatformOrchestrator(platform_orchestrator::CANISTER_ID, &self.agent)
    }

    pub async fn get_individual_canister_by_user_principal(
        &self,
        user_canister: Principal,
    ) -> Result<Option<Principal>, CanistersError> {
        let can = self
            .auth_client
            .get_individual_canister_by_user_principal(user_canister)
            .await?;
        if let Some(can) = can {
            return Ok(Some(can));
        }
        // Fallback to legacy fetch
        let user_idx = self.user_index_with(*LEGACY_USER_INDEX);
        let can = user_idx
            .get_user_canister_id_from_user_principal_id(user_canister)
            .await?;
        Ok(can)
    }
}

pub fn unauth_canisters() -> Canisters<false> {
    expect_context()
}

pub type AuthCanistersResource = Resource<
    MockPartialEq<Option<DelegationIdentity>>,
    Result<Option<Canisters<true>>, CanistersError>,
>;

async fn create_individual_canister(
    canisters: &Canisters<true>,
    delegation_id: DelegationIdentity,
) -> Result<Principal, CanistersError> {
    // TODO: this is temporary
    let blacklisted = HashSet::from([Principal::from_text("rimrc-piaaa-aaaao-aaljq-cai").unwrap()]);
    let orchestrator = canisters.orchestrator();
    let subnet_idxs: Vec<_> = orchestrator
        .get_all_available_subnet_orchestrators()
        .await?
        .into_iter()
        .filter(|subnet| !blacklisted.contains(subnet))
        .collect();

    let mut by = [0u8; 16];
    let principal = canisters.identity().sender().unwrap();
    let principal_by = principal.as_slice();
    let cnt = by.len().min(principal_by.len());
    by[..cnt].copy_from_slice(&principal_by[..cnt]);

    let discrim = u128::from_be_bytes(by);
    let subnet_idx = subnet_idxs[(discrim % subnet_idxs.len() as u128) as usize];
    let idx = canisters.user_index_with(subnet_idx);
    let user_canister = idx
        .get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer()
        .await?;

    canisters
        .auth_client
        .update_user_metadata(delegation_id, user_canister, "".into())
        .await?;
    Ok(user_canister)
}

pub async fn do_canister_auth(
    auth: Option<DelegationIdentity>,
) -> Result<Option<Canisters<true>>, CanistersError> {
    let Some(delegation_identity) = auth else {
        return Ok(None);
    };

    let auth: DelegatedIdentity = delegation_identity.clone().try_into()?;
    let mut canisters = Canisters::<true>::authenticated(auth);

    canisters.user_canister = if let Some(user_canister) = canisters
        .auth_client
        .get_individual_canister_by_user_principal(canisters.identity().sender().unwrap())
        .await?
    {
        user_canister
    } else {
        create_individual_canister(&canisters, delegation_identity).await?
    };

    Ok(Some(canisters))
}

pub fn authenticated_canisters() -> AuthCanistersResource {
    expect_context()
}
