use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use codee::string::FromToStringCodec;
use leptos::*;
use leptos_use::storage::use_local_storage;
use serde::Serialize;
use yral_auth_client::{types::DelegatedIdentityWire, AuthClient};

use crate::consts::{ACCOUNT_CONNECTED_STORE, AUTH_API_BASE};

#[derive(Default, Clone)]
pub struct AuthState {
    pub identity: RwSignal<Option<DelegatedIdentityWire>>,
}

pub fn auth_client() -> AuthClient {
    expect_context()
}

pub fn auth_state() -> AuthState {
    expect_context()
}

pub fn get_default_metadata_client() -> AuthClient {
    AuthClient::with_base_url(AUTH_API_BASE.clone(), None)
}

#[cfg(feature = "backend-admin")]
pub fn get_default_auth_client() -> AuthClient {
    let private_key =
        env::var("BACKEND_ADMIN_IDENTITY").expect("BACKEND ADMIN IDENTITY SHOULD BE PRESENT");
    #[derive(Serialize)]
    struct JwtAuth {
        namespace: String,
        exp: u128,
    }

    let exp_time = Duration::saturating_add(
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        Duration::from_secs(60 * 60),
    );

    let encoding_key = jsonwebtoken::EncodingKey::from_ed_pem(private_key.as_bytes()).unwrap();
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA),
        &JwtAuth {
            namespace: "YRAL".into(),
            exp: exp_time.as_secs() as u128,
        },
        &encoding_key,
    )
    .unwrap();

    AuthClient::with_base_url(AUTH_API_BASE.clone(), Some(&token))
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
