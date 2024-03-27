#[cfg(feature = "ssr")]
pub mod server_impl;

use candid::Principal;
use ic_agent::identity::{DelegatedIdentity, Secp256k1Identity, SignedDelegation};
use k256::elliptic_curve::JwkEcKey;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

/// Delegated identity that can be serialized over the wire
#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserMetadata {
    pub user_canister_id: Principal,
    pub user_name: String,
}

#[server]
pub async fn extract_or_generate_identity() -> Result<DelegatedIdentityWire, ServerFnError> {
    server_impl::extract_or_generate_identity_impl().await
}

#[server]
pub async fn set_user_metadata(
    principal: Principal,
    metadata: UserMetadata,
) -> Result<(), ServerFnError> {
    server_impl::set_user_metadata_impl(principal, metadata).await
}

#[server]
pub async fn get_user_metadata(
    principal: Principal,
) -> Result<Option<UserMetadata>, ServerFnError> {
    server_impl::get_user_metadata_impl(principal).await
}
