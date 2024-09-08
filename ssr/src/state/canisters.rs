use std::sync::Arc;

use candid::Principal;
use ic_agent::{identity::DelegatedIdentity, AgentError, Identity};
use leptos::*;
use serde::{Deserialize, Serialize};
use yral_auth_client::{types::{metadata::UserMetadata, DelegatedIdentityWire}, AuthClient};

use crate::{
    canister::{
        individual_user_template::{IndividualUserTemplate, Result9, UserCanisterDetails},
        platform_orchestrator::PlatformOrchestrator,
        post_cache::PostCache,
        user_index::UserIndex,
        PLATFORM_ORCHESTRATOR_ID, POST_CACHE_ID,
    },
    consts::METADATA_API_BASE,
    utils::{ic::AgentWrapper, profile::ProfileDetails, MockPartialEq},
};

use super::auth::get_default_metadata_client;

#[cfg(feature = "backend-admin")]
use super::auth::get_default_auth_client;

#[derive(Clone, Serialize, Deserialize)]
pub struct CanistersAuthWire {
    id: DelegatedIdentityWire,
    user_canister: Principal,
    expiry: u64,
    profile_details: ProfileDetails,
}

impl CanistersAuthWire {
    pub fn canisters(self) -> Result<Canisters<true>, k256::elliptic_curve::Error> {
        let unauth = unauth_canisters();

        let id: DelegatedIdentity = self.id.try_into()?;
        let arc_id = Arc::new(id);

        let mut agent = unauth.agent.clone();
        agent.set_arc_id(arc_id.clone());

        Ok(Canisters {
            agent,
            id: Some(arc_id),
            metadata_client: unauth.metadata_client.clone(),
            user_canister: self.user_canister,
            expiry: self.expiry,
            profile_details: Some(self.profile_details),
        })
    }
}

#[derive(Clone)]
pub struct Canisters<const AUTH: bool> {
    agent: AgentWrapper,
    id: Option<Arc<DelegatedIdentity>>,
    metadata_client: AuthClient,
    user_canister: Principal,
    expiry: u64,
    profile_details: Option<ProfileDetails>,
}

impl Default for Canisters<false> {
    fn default() -> Self {



        Self {
            agent: AgentWrapper::build(|b| b),
            id: None,
            metadata_client: get_default_metadata_client(),
            user_canister: Principal::anonymous(),
            expiry: 0,
            profile_details: None,
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
            agent: AgentWrapper::build(|b| b.with_arc_identity(id.clone())),
            metadata_client: get_default_metadata_client(),
            id: Some(id),
            user_canister: Principal::anonymous(),
            expiry,
            profile_details: None,
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

    pub async fn authenticated_user(&self) -> Result<IndividualUserTemplate<'_>, AgentError> {
        self.individual_user(self.user_canister).await
    }

    pub fn profile_details(&self) -> ProfileDetails {
        self.profile_details
            .clone()
            .expect("Authenticated canisters must have profile details")
    }

    pub fn user_principal(&self) -> Principal {
        self.identity()
            .sender()
            .expect("expect principal to be present")
    }
}

impl<const A: bool> Canisters<A> {
    pub async fn post_cache(&self) -> Result<PostCache<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(PostCache(POST_CACHE_ID, agent))
    }

    pub async fn individual_user(
        &self,
        user_canister: Principal,
    ) -> Result<IndividualUserTemplate<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(IndividualUserTemplate(user_canister, agent))
    }

    pub async fn user_index_with(
        &self,
        subnet_principal: Principal,
    ) -> Result<UserIndex<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(UserIndex(subnet_principal, agent))
    }

    pub async fn orchestrator(&self) -> Result<PlatformOrchestrator<'_>, AgentError> {
        let agent = self.agent.get_agent().await?;
        Ok(PlatformOrchestrator(PLATFORM_ORCHESTRATOR_ID, agent))
    }

    pub async fn get_individual_canister_by_user_principal(
        &self,
        user_principal: Principal,
    ) -> Result<Option<Principal>, ServerFnError> {
        let meta = self
            .metadata_client
            .get_user_metadata(user_principal)
            .await?;
        Ok(meta.map(|m| m.user_canister_id))
    }

    async fn subnet_indexes(&self) -> Result<Vec<Principal>, AgentError> {
        #[cfg(any(feature = "local-bin", feature = "local-lib"))]
        {
            use crate::canister::USER_INDEX_ID;
            Ok(vec![USER_INDEX_ID])
        }
        #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
        {
            use std::collections::HashSet;
            // TODO: this is temporary
            let blacklisted =
                HashSet::from([Principal::from_text("rimrc-piaaa-aaaao-aaljq-cai").unwrap()]);
            let orchestrator = self.orchestrator().await?;
            Ok(orchestrator
                .get_all_available_subnet_orchestrators()
                .await?
                .into_iter()
                .filter(|subnet| !blacklisted.contains(subnet))
                .collect())
        }
    }
}

pub fn unauth_canisters() -> Canisters<false> {
    expect_context()
}

async fn create_individual_canister(
    canisters: &Canisters<true>,
) -> Result<Principal, ServerFnError> {
    let subnet_idxs = canisters.subnet_indexes().await?;

    let mut by = [0u8; 16];
    let principal = canisters.identity().sender().unwrap();
    let principal_by = principal.as_slice();
    let cnt = by.len().min(principal_by.len());
    by[..cnt].copy_from_slice(&principal_by[..cnt]);

    let discrim = u128::from_be_bytes(by);
    let subnet_idx = subnet_idxs[(discrim % subnet_idxs.len() as u128) as usize];
    let idx = canisters.user_index_with(subnet_idx).await?;
    let user_canister = idx
        .get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer()
        .await?;

    canisters
        .metadata_client
        .set_user_metadata(
            canisters.identity(),
            UserMetadata {
                user_canister_id: user_canister,
                user_name: "".into(),
            },
        )
        .await?;

    Ok(user_canister)
}

pub async fn do_canister_auth(
    auth: Option<DelegatedIdentityWire>,
    referrer: Option<Principal>,
) -> Result<Option<CanistersAuthWire>, ServerFnError> {

    let Some(auth) = auth else {
        return Ok(None)
    };

    let id = auth.clone().try_into()?;
    let mut canisters = Canisters::<true>::authenticated(id);

    canisters.user_canister = if let Some(user_canister) = canisters
        .get_individual_canister_by_user_principal(canisters.identity().sender().unwrap())
        .await?
    {
        user_canister
    } else {
        create_individual_canister(&canisters).await?
    };

    let user = canisters.authenticated_user().await?;

    if let Some(referrer_principal_id) = referrer {
        let referrer_canister = canisters
            .get_individual_canister_by_user_principal(referrer_principal_id)
            .await?;
        if let Some(referrer_canister_id) = referrer_canister {
            user.update_referrer_details(UserCanisterDetails {
                user_canister_id: referrer_canister_id,
                profile_owner: referrer_principal_id,
            })
            .await?;
        }
    }

    match user
        .update_last_access_time()
        .await
        .map_err(|e| e.to_string())
    {
        Ok(Result9::Ok(_)) => (),
        Err(e) | Ok(Result9::Err(e)) => log::warn!("Failed to update last access time: {}", e),
    }
    let profile_details = user.get_profile_details().await?.into();

    let cans_wire = CanistersAuthWire {
        id: auth,
        user_canister: canisters.user_canister,
        expiry: canisters.expiry,
        profile_details,
    };

    Ok(Some(cans_wire))
}

pub type AuthCansResource = Resource<
    MockPartialEq<Option<DelegatedIdentityWire>>,
    Result<Option<CanistersAuthWire>, ServerFnError>,
>;

/// The Authenticated Canisters helper resource
/// prefer using helpers from [crate::component::canisters_prov]
/// instead
pub fn authenticated_canisters() -> AuthCansResource {
    expect_context()
}

/// The store for Authenticated canisters
/// Do not use this for anything other than analytics
pub fn auth_canisters_store() -> RwSignal<Option<Canisters<true>>> {
    expect_context()
}
