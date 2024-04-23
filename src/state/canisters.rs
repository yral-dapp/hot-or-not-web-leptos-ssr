use std::{collections::HashSet, sync::Arc};

use candid::Principal;
use ic_agent::{identity::DelegatedIdentity, AgentError, Identity};
use leptos::*;
use yral_metadata_client::MetadataClient;
use yral_metadata_types::UserMetadata;

use crate::{
    auth::DelegatedIdentityWire,
    canister::{
        individual_user_template::{IndividualUserTemplate, Result8, UserCanisterDetails},
        platform_orchestrator::{self, PlatformOrchestrator},
        post_cache::{self, PostCache},
        user_index::UserIndex,
        AGENT_URL,
    },
    consts::{FALLBACK_USER_INDEX, METADATA_API_BASE},
    utils::{profile::ProfileDetails, MockPartialEq},
};

use super::local_storage::use_referrer_store;

#[derive(Clone)]
pub struct Canisters<const AUTH: bool> {
    agent: ic_agent::Agent,
    id: Option<Arc<DelegatedIdentity>>,
    metadata_client: MetadataClient,
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
            metadata_client: MetadataClient::with_base_url(METADATA_API_BASE.clone()),
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
            metadata_client: MetadataClient::with_base_url(METADATA_API_BASE.clone()),
            id: Some(id),
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
        user_principal: Principal,
    ) -> Result<Option<Principal>, ServerFnError> {
        let meta = self
            .metadata_client
            .get_user_metadata(user_principal)
            .await?;
        if let Some(meta) = meta {
            return Ok(Some(meta.user_canister_id));
        }
        // Fallback to oldest user index
        let user_idx = self.user_index_with(*FALLBACK_USER_INDEX);
        let can = user_idx
            .get_user_canister_id_from_user_principal_id(user_principal)
            .await?;
        Ok(can)
    }

    async fn subnet_indexes(&self) -> Result<Vec<Principal>, AgentError> {
        // TODO: this is temporary
        let blacklisted =
            HashSet::from([Principal::from_text("rimrc-piaaa-aaaao-aaljq-cai").unwrap()]);
        let orchestrator = self.orchestrator();
        Ok(orchestrator
            .get_all_available_subnet_orchestrators()
            .await?
            .into_iter()
            .filter(|subnet| !blacklisted.contains(subnet))
            .collect())
    }
}

pub fn unauth_canisters() -> Canisters<false> {
    expect_context()
}

pub type AuthCanistersResource = Resource<
    MockPartialEq<Option<DelegatedIdentityWire>>,
    Result<Option<Canisters<true>>, ServerFnError>,
>;

pub type AuthProfileCanisterResource = Resource<
    MockPartialEq<Option<Result<Canisters<true>, ServerFnError>>>,
    Option<(ProfileDetails, Principal)>,
>;

async fn create_individual_canister(
    canisters: &Canisters<true>,
    _delegation_id: DelegatedIdentityWire,
) -> Result<Principal, ServerFnError> {
    let subnet_idxs = canisters.subnet_indexes().await?;

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
) -> Result<Option<Canisters<true>>, ServerFnError> {
    let Some(delegation_identity) = auth else {
        return Ok(None);
    };

    let auth: DelegatedIdentity = delegation_identity.clone().try_into()?;
    let mut canisters = Canisters::<true>::authenticated(auth);

    canisters.user_canister = if let Some(user_canister) = canisters
        .get_individual_canister_by_user_principal(canisters.identity().sender().unwrap())
        .await?
    {
        user_canister
    } else {
        create_individual_canister(&canisters, delegation_identity).await?
    };

    let (_, set_referrer_store, _) = use_referrer_store();
    let user = canisters.authenticated_user();

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

        set_referrer_store.set(Some(referrer_principal_id));
    }

    match user
        .update_last_access_time()
        .await
        .map_err(|e| e.to_string())
    {
        Ok(Result8::Ok(_)) => (),
        Err(e) | Ok(Result8::Err(e)) => log::warn!("Failed to update last access time: {}", e),
    }

    Ok(Some(canisters))
}

pub fn authenticated_canisters() -> AuthCanistersResource {
    expect_context()
}
