use candid::Principal;
use ic_agent::identity::{DelegatedIdentity, Secp256k1Identity};
use k256::SecretKey;

use serde::{Deserialize, Serialize};

use super::AuthError;

#[derive(Debug, Serialize, Clone)]
struct PrincipalId {
    _arr: String,
    #[serde(rename = "_isPrincipal")]
    _is_principal: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelegationIdentity {
    pub _inner: Vec<Vec<u8>>,
    pub _delegation: DelegationChain,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DelegationChain {
    pub delegations: Vec<SignedDelegation>,
    #[serde(rename = "publicKey")]
    pub public_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
pub struct SessionResponse {
    #[allow(dead_code)]
    user_identity: String,
    pub delegation_identity: DelegationIdentity,
}

#[derive(Serialize)]
pub struct UserDetails {
    pub delegation_identity: DelegationIdentity,
    pub user_canister_id: String,
    pub user_name: String,
}
