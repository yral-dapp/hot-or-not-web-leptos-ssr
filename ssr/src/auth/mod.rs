#[cfg(feature = "ssr")]
pub mod server_impl;

use candid::Principal;
use ic_agent::{
    identity::{DelegatedIdentity, Delegation, Secp256k1Identity, SignedDelegation},
    Identity,
};
use k256::elliptic_curve::JwkEcKey;
use leptos::{server, ServerFnError};
use rand_chacha::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use web_time::Duration;

use crate::{consts::auth::DELEGATION_MAX_AGE, utils::time::current_epoch};

/// Delegated identity that can be serialized over the wire
#[derive(Serialize, Deserialize, Clone)]
pub struct DelegatedIdentityWire {
    /// raw bytes of delegated identity's public key
    from_key: Vec<u8>,
    /// JWK(JSON Web Key) encoded Secp256k1 secret key
    /// identity allowed to sign on behalf of `from_key`
    to_secret: JwkEcKey,
    /// Proof of delegation
    /// connecting from_key to `to_secret`
    delegation_chain: Vec<SignedDelegation>,
}

impl DelegatedIdentityWire {
    fn delegate_with_max_age(from: &impl Identity, max_age: Duration) -> Self {
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

        Self {
            from_key: sig.public_key.unwrap(),
            to_secret: to_secret.to_jwk(),
            delegation_chain,
        }
    }

    pub fn delegate(from: &impl Identity) -> Self {
        Self::delegate_with_max_age(from, DELEGATION_MAX_AGE)
    }

    pub fn delegate_short_lived_identity(from: &impl Identity) -> Self {
        let max_age = Duration::from_secs(24 * 60 * 60); // 1 day
        Self::delegate_with_max_age(from, max_age)
    }
}

impl std::fmt::Debug for DelegatedIdentityWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DelegatedIdentityWire").finish()
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct RefreshToken {
    principal: Principal,
    expiry_epoch_ms: u128,
}

impl TryFrom<DelegatedIdentityWire> for DelegatedIdentity {
    type Error = k256::elliptic_curve::Error;

    fn try_from(identity: DelegatedIdentityWire) -> Result<Self, Self::Error> {
        let to_secret = k256::SecretKey::from_jwk(&identity.to_secret)?;
        let to_identity = Secp256k1Identity::from_private_key(to_secret);
        Ok(Self::new(
            identity.from_key,
            Box::new(to_identity),
            identity.delegation_chain,
        ))
    }
}

/// Generate an anonymous identity if refresh token is not set
#[server]
pub async fn generate_anonymous_identity_if_required() -> Result<Option<JwkEcKey>, ServerFnError> {
    server_impl::generate_anonymous_identity_if_required_impl().await
}

/// this server function is purely a side effect and only sets the refresh token cookie
#[server]
pub async fn set_anonymous_identity_cookie(
    anonymous_identity: JwkEcKey,
) -> Result<(), ServerFnError> {
    server_impl::set_anonymous_identity_cookie_impl(anonymous_identity).await
}

/// Extract the identity from refresh token,
/// returns None if refresh token doesn't exist
#[server]
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
    }

    impl CoreClients {
        pub fn get_oauth_client(&self, host: &str) -> openidconnect::core::CoreClient {
            if host == "hotornot.wtf" {
                self.hotornot_google_oauth.clone()
            } else {
                self.google_oauth.clone()
            }
        }
    }
}
