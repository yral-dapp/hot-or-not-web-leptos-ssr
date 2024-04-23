#[cfg(feature = "ssr")]
pub mod server_impl;

use candid::Principal;
use ic_agent::identity::{DelegatedIdentity, Secp256k1Identity, SignedDelegation};
use k256::elliptic_curve::JwkEcKey;
use leptos::{server, server_fn::codec::Cbor, ServerFnError};
use serde::{Deserialize, Serialize};

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

impl std::fmt::Debug for DelegatedIdentityWire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DelegatedIdentityWire").finish()
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum RefreshTokenKind {
    Upgraded,
    Temporary,
}

impl Default for RefreshTokenKind {
    fn default() -> Self {
        Self::Upgraded
    }
}

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub struct RefreshToken {
    principal: Principal,
    expiry_epoch_ms: u128,
    #[serde(default)]
    kind: RefreshTokenKind,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct TempRefreshToken {
    inner: RefreshToken,
    digest: Vec<u8>,
}

impl std::fmt::Debug for TempRefreshToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TempRefreshToken").finish()
    }
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

#[server]
pub async fn extract_or_generate_identity(
) -> Result<(DelegatedIdentityWire, TempRefreshToken), ServerFnError> {
    server_impl::extract_or_generate_identity_impl().await
}

#[server(input = Cbor)]
pub async fn upgrade_temp_refresh_token(token: TempRefreshToken) -> Result<(), ServerFnError> {
    server_impl::upgrade_temp_refresh_token_impl(token).await
}

#[server]
pub async fn logout_identity() -> Result<DelegatedIdentityWire, ServerFnError> {
    server_impl::logout_identity_impl().await
}
