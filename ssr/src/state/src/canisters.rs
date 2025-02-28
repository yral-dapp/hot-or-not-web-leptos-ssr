use candid::Principal;
use leptos::prelude::*;
use yral_canisters_common::{Canisters, CanistersAuthWire};

use utils::{send_wrap, MockPartialEq};
use yral_types::delegated_identity::DelegatedIdentityWire;

pub fn unauth_canisters() -> Canisters<false> {
    expect_context()
}

pub async fn do_canister_auth(
    auth: DelegatedIdentityWire,
    referrer: Option<Principal>,
) -> Result<CanistersAuthWire, ServerFnError> {
    let auth_fut = Canisters::authenticate_with_network(auth, referrer);
    let canisters = send_wrap(auth_fut).await?;
    Ok(canisters.into())
}
pub type AuthCansResource = Resource<Result<CanistersAuthWire, ServerFnError>>;

/// The Authenticated Canisters helper resource
/// prefer using helpers from [crate::component::canisters_prov]
/// instead
pub fn authenticated_canisters() -> AuthCansResource {
    expect_context()
}