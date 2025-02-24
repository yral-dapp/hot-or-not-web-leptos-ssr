use candid::Principal;
use leptos::prelude::*;
use yral_canisters_common::{Canisters, CanistersAuthWire};

use crate::utils::MockPartialEq;
use yral_types::delegated_identity::DelegatedIdentityWire;

pub fn unauth_canisters() -> Canisters<false> {
    expect_context()
}

pub async fn do_canister_auth(
    auth: DelegatedIdentityWire,
    referrer: Option<Principal>,
) -> Result<CanistersAuthWire, ServerFnError> {
    let canisters = Canisters::authenticate_with_network(auth, referrer).await?;
    Ok(canisters.into())
}

pub type AuthCansResource = Resource<Result<CanistersAuthWire, ServerFnError>>;

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
