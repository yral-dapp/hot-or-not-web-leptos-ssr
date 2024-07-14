use std::sync::Arc;

use candid::Principal;
use ic_agent::{agent::AgentBuilder, identity::DelegatedIdentity, AgentError, Identity};
use leptos::*;
use yral_metadata_client::MetadataClient;
use yral_metadata_types::UserMetadata;

use crate::{
    auth::DelegatedIdentityWire,
    canister::{
        individual_user_template::{IndividualUserTemplate, Result9, UserCanisterDetails},
        platform_orchestrator::PlatformOrchestrator,
        post_cache::PostCache,
        user_index::UserIndex,
        PLATFORM_ORCHESTRATOR_ID, POST_CACHE_ID,
    },
    consts::{AGENT_URL, METADATA_API_BASE},
    utils::{profile::ProfileDetails, MockPartialEq},
};

pub fn build_agent(builder_func: impl FnOnce(AgentBuilder) -> AgentBuilder) -> ic_agent::Agent {
    let mut builder = ic_agent::Agent::builder().with_url(AGENT_URL);

    builder = builder_func(builder);
    #[cfg(any(feature = "local-bin", feature = "local-lib"))]
    {
        builder = builder.with_verify_query_signatures(false);
        let agent = builder.build().unwrap();
        // TODO: this is specific to the local environment
        agent.set_root_key(Vec::from([
            48, 129, 130, 48, 29, 6, 13, 43, 6, 1, 4, 1, 130, 220, 124, 5, 3, 1, 2, 1, 6, 12, 43,
            6, 1, 4, 1, 130, 220, 124, 5, 3, 2, 1, 3, 97, 0, 177, 245, 178, 217, 104, 189, 171,
            227, 105, 94, 61, 178, 63, 151, 4, 117, 247, 115, 131, 226, 98, 62, 205, 43, 78, 42, 7,
            213, 13, 11, 186, 34, 7, 14, 23, 23, 196, 62, 83, 237, 220, 71, 19, 60, 44, 9, 152, 36,
            25, 96, 126, 153, 75, 232, 77, 136, 196, 241, 4, 243, 202, 21, 52, 235, 136, 5, 178,
            138, 210, 174, 215, 93, 253, 250, 164, 233, 106, 176, 111, 133, 142, 165, 125, 25, 82,
            136, 150, 165, 108, 198, 152, 49, 68, 168, 10, 40,
        ]));
        agent
    }
    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    {
        builder.build().unwrap()
    }
}

#[derive(Clone)]
pub struct Canisters<const AUTH: bool> {
    agent: ic_agent::Agent,
    id: Option<Arc<DelegatedIdentity>>,
    metadata_client: MetadataClient,
    user_canister: Principal,
    expiry: u64,
    profile_details: Option<ProfileDetails>,
}

impl Default for Canisters<false> {
    fn default() -> Self {
        Self {
            agent: build_agent(|b| b),
            id: None,
            metadata_client: MetadataClient::with_base_url(METADATA_API_BASE.clone()),
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
            agent: build_agent(|b| b.with_arc_identity(id.clone())),
            metadata_client: MetadataClient::with_base_url(METADATA_API_BASE.clone()),
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

    pub fn authenticated_user(&self) -> IndividualUserTemplate<'_> {
        IndividualUserTemplate(self.user_canister, &self.agent)
    }

    pub fn profile_details(&self) -> ProfileDetails {
        self.profile_details
            .clone()
            .expect("Authenticated canisters must have profile details")
    }
}

impl<const A: bool> Canisters<A> {
    pub fn post_cache(&self) -> PostCache<'_> {
        PostCache(POST_CACHE_ID, &self.agent)
    }

    pub fn individual_user(&self, user_canister: Principal) -> IndividualUserTemplate<'_> {
        IndividualUserTemplate(user_canister, &self.agent)
    }

    pub fn user_index_with(&self, subnet_principal: Principal) -> UserIndex<'_> {
        UserIndex(subnet_principal, &self.agent)
    }

    pub fn orchestrator(&self) -> PlatformOrchestrator<'_> {
        PlatformOrchestrator(PLATFORM_ORCHESTRATOR_ID, &self.agent)
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
            let orchestrator = self.orchestrator();
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
    auth: DelegatedIdentityWire,
    referrer: Option<Principal>,
) -> Result<Canisters<true>, ServerFnError> {
    let id: DelegatedIdentity = auth.clone().try_into()?;
    let mut canisters = Canisters::<true>::authenticated(id);

    canisters.user_canister = if let Some(user_canister) = canisters
        .get_individual_canister_by_user_principal(canisters.identity().sender().unwrap())
        .await?
    {
        user_canister
    } else {
        create_individual_canister(&canisters).await?
    };

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
    }

    match user
        .update_last_access_time()
        .await
        .map_err(|e| e.to_string())
    {
        Ok(Result9::Ok(_)) => (),
        Err(e) | Ok(Result9::Err(e)) => log::warn!("Failed to update last access time: {}", e),
    }
    canisters.profile_details = Some(user.get_profile_details().await?.into());

    Ok(canisters)
}

pub type AuthCansResource =
    Resource<MockPartialEq<Option<DelegatedIdentityWire>>, Result<Canisters<true>, ServerFnError>>;

pub fn authenticated_canisters() -> AuthCansResource {
    expect_context()
}

pub fn auth_canisters_store() -> RwSignal<Option<Canisters<true>>> {
    expect_context()
}
