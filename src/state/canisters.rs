use std::sync::Arc;

use candid::Principal;
use ic_agent::{identity::DelegatedIdentity, Identity};
use leptos::{expect_context, Resource};

use crate::{
    canister::{
        individual_user_template::IndividualUserTemplate,
        post_cache::{self, PostCache},
        user_index::{self, UserIndex},
        AGENT_URL,
    },
    state::auth::AuthClient,
};

use super::auth::AuthError;

#[derive(Clone)]
pub struct Canisters<const AUTH: bool> {
    agent: ic_agent::Agent,
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

    pub fn user_index(&self) -> UserIndex<'_> {
        UserIndex(user_index::CANISTER_ID, &self.agent)
    }
}

pub fn unauth_canisters() -> Canisters<false> {
    expect_context()
}

pub type AuthCanistersResource = Resource<(), Result<Canisters<true>, AuthError>>;

pub async fn do_canister_auth(client: AuthClient) -> Result<Canisters<true>, AuthError> {
    let auth = client.generate_session().await?;
    let mut canisters = Canisters::<true>::authenticated(auth);
    let idx = canisters.user_index();
    // TOOD: referrer
    // TODO: error handling
    let user_canister = idx
        .get_requester_principals_canister_id_create_if_not_exists_and_optionally_allow_referrer(
            None,
        )
        .await
        .unwrap();
    canisters.user_canister = user_canister;
    Ok(canisters)
}

pub fn authenticated_canisters() -> AuthCanistersResource {
    // TODO: handle identity expiry
    expect_context()
}
