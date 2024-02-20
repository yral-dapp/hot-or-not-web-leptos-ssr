pub mod types;

use std::num::ParseIntError;

use ic_agent::{export::Principal, identity::DelegatedIdentity};

use leptos::{create_effect, create_signal, expect_context, Effect, ReadSignal, RwSignal};
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use thiserror::Error;

use crate::consts::{ACCOUNT_CONNECTED_STORE, AUTH_URL};
use types::{DelegationIdentity, SessionResponse, UserDetails};

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

    pub async fn update_user_metadata(
        &self,
        id: DelegationIdentity,
        user_canister: Principal,
        username: String,
    ) -> Result<(), AuthError> {
        let details = UserDetails {
            delegation_identity: id,
            user_canister_id: user_canister.to_text(),
            user_name: username,
        };
        let res = self
            .client
            .post(AUTH_URL.join("rest_api/update_user_metadata").unwrap())
            .json(&details)
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            Err(AuthError::Reqwest(res.text().await?))
        }
    }

    pub async fn get_individual_canister_by_user_principal(
        &self,
        user_principal: Principal,
    ) -> Result<Option<Principal>, AuthError> {
        let res = self
            .client
            .post(AUTH_URL.join("rest_api/get_user_canister").unwrap())
            .json(&user_principal.to_text())
            .send()
            .await?
            .text()
            .await?;
        Ok(Principal::from_text(res).ok())
    }
}

pub fn auth_client() -> AuthClient {
    expect_context()
}

#[derive(Default, Clone)]
pub struct AuthState {
    pub identity: RwSignal<Option<DelegationIdentity>>,
}

pub fn auth_state() -> AuthState {
    expect_context()
}

/// Prevents hydration bugs if the value in store is used to conditionally show views
/// this is because the server will always get a `false` value and do rendering based on that
pub fn account_connected_reader() -> (ReadSignal<bool>, Effect<()>) {
    let (read_account_connected, _, _) =
        use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
    let (is_connected, set_is_connected) = create_signal(false);

    (
        is_connected,
        create_effect(move |_| {
            set_is_connected(read_account_connected());
        }),
    )
}
