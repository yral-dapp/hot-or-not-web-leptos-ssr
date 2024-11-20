#[cfg(feature = "ssr")]
pub mod server_impl;

use candid::Principal;
use ic_agent::{
    identity::{Delegation, Secp256k1Identity, SignedDelegation},
    Identity,
};
use k256::elliptic_curve::JwkEcKey;
use leptos::{server, server_fn::codec::Json, ServerFnError};
use rand_chacha::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use web_time::Duration;
use yral_canisters_common::utils::time::current_epoch;

use crate::consts::auth::DELEGATION_MAX_AGE;
use yral_types::delegated_identity::DelegatedIdentityWire;

fn delegate_identity_with_max_age(
    from: &impl Identity,
    max_age: Duration,
) -> DelegatedIdentityWire {
    let to_secret = k256::SecretKey::random(&mut OsRng);
    let to_identity = Secp256k1Identity::from_private_key(to_secret.clone());
    let expiry = current_epoch() + max_age;
    let expiry_ns = expiry.as_nanos() as u64;
    let delegation = Delegation {
        pubkey: to_identity.public_key().unwrap(),
        expiration: expiry_ns,
        targets: None,
    };
    let sig = from.sign_delegation(&delegation).unwrap();
    let signed_delegation = SignedDelegation {
        delegation,
        signature: sig.signature.unwrap(),
    };

    let mut delegation_chain = from.delegation_chain();
    delegation_chain.push(signed_delegation);

    DelegatedIdentityWire {
        from_key: sig.public_key.unwrap(),
        to_secret: to_secret.to_jwk(),
        delegation_chain,
    }
}

pub fn delegate_identity(from: &impl Identity) -> DelegatedIdentityWire {
    delegate_identity_with_max_age(from, DELEGATION_MAX_AGE)
}

pub fn delegate_short_lived_identity(from: &impl Identity) -> DelegatedIdentityWire {
    let max_age = Duration::from_secs(24 * 60 * 60); // 1 day
    delegate_identity_with_max_age(from, max_age)
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct RefreshToken {
    principal: Principal,
    expiry_epoch_ms: u128,
}

/// Generate an anonymous identity if refresh token is not set
#[server]
pub async fn generate_anonymous_identity_if_required() -> Result<Option<JwkEcKey>, ServerFnError> {
    server_impl::generate_anonymous_identity_if_required_impl().await
}

/// this server function is purely a side effect and only sets the refresh token cookie
#[server(endpoint = "set_anonymous_identity_cookie", input = Json, output = Json)]
pub async fn set_anonymous_identity_cookie(
    anonymous_identity: JwkEcKey,
) -> Result<(), ServerFnError> {
    server_impl::set_anonymous_identity_cookie_impl(anonymous_identity).await
}

/// Extract the identity from refresh token,
/// returns None if refresh token doesn't exist
#[server(endpoint = "extract_identity", input = Json, output = Json)]
pub async fn extract_identity() -> Result<Option<DelegatedIdentityWire>, ServerFnError> {
    server_impl::extract_identity_impl().await
}

#[server]
pub async fn logout_identity() -> Result<DelegatedIdentityWire, ServerFnError> {
    server_impl::logout_identity_impl().await
}

#[cfg(feature = "oauth-ssr")]
pub mod core_clients {
    #[derive(Clone)]
    pub struct CoreClients {
        pub google_oauth: openidconnect::core::CoreClient,
        pub hotornot_google_oauth: openidconnect::core::CoreClient,
        pub icpump_google_oauth: openidconnect::core::CoreClient,
    }

    impl CoreClients {
        pub fn get_oauth_client(&self, host: &str) -> openidconnect::core::CoreClient {
            if host == "hotornot.wtf" {
                self.hotornot_google_oauth.clone()
            } else if host == "icpump.fun" {
                self.icpump_google_oauth.clone()
            } else {
                self.google_oauth.clone()
            }
        }
    }
}
