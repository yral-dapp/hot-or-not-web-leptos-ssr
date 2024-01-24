use std::num::ParseIntError;

use candid::Principal;
use ic_agent::identity::{DelegatedIdentity, Secp256k1Identity};
use k256::SecretKey;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consts::AUTH_URL;

#[derive(Debug, Serialize)]
struct PrincipalId {
    _arr: String,
    #[serde(rename = "_isPrincipal")]
    _is_principal: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelegationIdentity {
    pub _inner: Vec<Vec<u8>>,
    pub _delegation: DelegationChain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DelegationChain {
    pub delegations: Vec<SignedDelegation>,
    #[serde(rename = "publicKey")]
    pub public_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedDelegation {
    pub delegation: Delegation,
    pub signature: Vec<u8>,
}

impl TryFrom<SignedDelegation> for ic_agent::identity::SignedDelegation {
    type Error = AuthError;

    fn try_from(value: SignedDelegation) -> Result<Self, AuthError> {
        Ok(ic_agent::identity::SignedDelegation {
            delegation: ic_agent::identity::Delegation {
                pubkey: value.delegation.pubkey,
                expiration: u64::from_str_radix(&value.delegation.expiration, 16)?,
                targets: value.delegation.targets.and_then(|v| {
                    v.into_iter()
                        .map(|s| Principal::from_text(s).ok())
                        .collect::<Option<_>>()
                }),
            },
            signature: value.signature,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delegation {
    pub pubkey: Vec<u8>,
    pub expiration: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
}

impl TryFrom<DelegationIdentity> for DelegatedIdentity {
    type Error = AuthError;

    fn try_from(value: DelegationIdentity) -> Result<Self, AuthError> {
        let sec_key = SecretKey::from_slice(&value._inner[1])?;
        let del_key = Secp256k1Identity::from_private_key(sec_key);
        Ok(DelegatedIdentity::new(
            value._delegation.public_key,
            Box::new(del_key),
            value
                ._delegation
                .delegations
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Deserialize)]
struct SessionResponse {
    #[allow(dead_code)]
    user_identity: String,
    delegation_identity: DelegationIdentity,
}

#[derive(Error, Debug, Clone)]
pub enum AuthError {
    #[error("Invalid Secret Key")]
    InvalidSecretKey(#[from] k256::elliptic_curve::Error),
    #[error("Invalid expiry")]
    InvalidExpiry(#[from] ParseIntError),
    #[error("reqwest error: {0}")]
    Reqwest(String),
}

impl From<reqwest::Error> for AuthError {
    fn from(e: reqwest::Error) -> Self {
        AuthError::Reqwest(e.to_string())
    }
}

#[derive(Default, Clone)]
pub struct AuthClient {
    client: reqwest::Client,
}

impl AuthClient {
    pub async fn generate_session(&self) -> Result<DelegatedIdentity, AuthError> {
        let resp: SessionResponse = self
            .client
            .post(AUTH_URL.join("api/generate_session").unwrap())
            .send()
            .await?
            .json()
            .await?;
        resp.delegation_identity.try_into()
    }
}
