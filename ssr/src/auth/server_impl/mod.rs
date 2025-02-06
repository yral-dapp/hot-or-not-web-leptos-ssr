#[cfg(feature = "oauth-ssr")]
pub mod google;
pub mod store;

use axum::response::IntoResponse;
use axum_extra::extract::{
    cookie::{Cookie, Key, SameSite},
    SignedCookieJar,
};
use candid::Principal;
use http::header;
use ic_agent::{identity::Secp256k1Identity, Identity};
use k256::elliptic_curve::JwkEcKey;
use leptos::{expect_context, ServerFnError};
use leptos_axum::{extract_with_state, ResponseOptions};
use rand_chacha::rand_core::OsRng;
use yral_canisters_common::utils::time::current_epoch;

use crate::consts::auth::{REFRESH_MAX_AGE, REFRESH_TOKEN_COOKIE};

use self::store::{KVStore, KVStoreImpl};
use yral_types::delegated_identity::DelegatedIdentityWire;

use super::{delegate_identity, RefreshToken};

fn set_cookies(resp: &ResponseOptions, jar: impl IntoResponse) {
    let resp_jar = jar.into_response();
    for cookie in resp_jar
        .headers()
        .get_all(header::SET_COOKIE)
        .into_iter()
        .cloned()
    {
        resp.append_header(header::SET_COOKIE, cookie);
    }
}

pub fn extract_principal_from_cookie(
    jar: &SignedCookieJar,
) -> Result<Option<Principal>, ServerFnError> {
    let Some(cookie) = jar.get(REFRESH_TOKEN_COOKIE) else {
        return Ok(None);
    };
    let token: RefreshToken = serde_json::from_str(cookie.value())?;
    if current_epoch().as_millis() > token.expiry_epoch_ms {
        return Ok(None);
    }
    Ok(Some(token.principal))
}

async fn fetch_identity_from_kv(
    kv: &KVStoreImpl,
    principal: Principal,
) -> Result<Option<k256::SecretKey>, ServerFnError> {
    let Some(identity_jwk) = kv.read(principal.to_text()).await? else {
        return Ok(None);
    };

    Ok(Some(k256::SecretKey::from_jwk_str(&identity_jwk)?))
}

pub async fn try_extract_identity(
    jar: &SignedCookieJar,
    kv: &KVStoreImpl,
) -> Result<Option<k256::SecretKey>, ServerFnError> {
    let Some(principal) = extract_principal_from_cookie(jar)? else {
        return Ok(None);
    };
    fetch_identity_from_kv(kv, principal).await
}

async fn generate_and_save_identity(kv: &KVStoreImpl) -> Result<Secp256k1Identity, ServerFnError> {
    let base_identity_key = k256::SecretKey::random(&mut OsRng);
    let base_identity = Secp256k1Identity::from_private_key(base_identity_key.clone());
    let principal = base_identity.sender().unwrap();

    let base_jwk = base_identity_key.to_jwk_string();
    kv.write(principal.to_text(), base_jwk.to_string()).await?;
    Ok(base_identity)
}

async fn save_identity(kv: &KVStoreImpl, id: JwkEcKey) -> Result<Secp256k1Identity, ServerFnError> {
    let base_identity = identity_from_jwk(&id)?;
    let principal = base_identity.sender().unwrap();

    let base_jwk = id.to_string();
    kv.write(principal.to_text(), base_jwk).await?;
    Ok(base_identity)
}

fn identity_from_jwk(id: &JwkEcKey) -> Result<Secp256k1Identity, ServerFnError> {
    let base_identity_key = k256::SecretKey::from_jwk(id)?;
    let base_identity: Secp256k1Identity =
        Secp256k1Identity::from_private_key(base_identity_key.clone());
    Ok(base_identity)
}

pub fn update_user_identity(
    response_opts: &ResponseOptions,
    mut jar: SignedCookieJar,
    identity: &impl Identity,
) -> Result<(), ServerFnError> {
    let refresh_max_age = REFRESH_MAX_AGE;
    let refresh_token = RefreshToken {
        principal: identity.sender().unwrap(),
        expiry_epoch_ms: (current_epoch() + refresh_max_age).as_millis(),
    };
    let refresh_token_enc = serde_json::to_string(&refresh_token)?;

    let refresh_cookie = Cookie::build((REFRESH_TOKEN_COOKIE, refresh_token_enc))
        .http_only(true)
        .secure(true)
        .path("/")
        .same_site(SameSite::None)
        .partitioned(true)
        .max_age(refresh_max_age.try_into().unwrap());

    jar = jar.add(refresh_cookie);
    set_cookies(response_opts, jar);
    Ok(())
}

pub fn update_user_identity_and_delegate(
    response_opts: &ResponseOptions,
    jar: SignedCookieJar,
    identity: impl Identity,
) -> Result<DelegatedIdentityWire, ServerFnError> {
    update_user_identity(response_opts, jar, &identity)?;
    Ok(delegate_identity(&identity))
}

pub async fn extract_identity_impl() -> Result<Option<DelegatedIdentityWire>, ServerFnError> {
    let key: Key = expect_context();
    let jar: SignedCookieJar = extract_with_state(&key).await?;
    let kv: KVStoreImpl = expect_context();

    let base_identity = if let Some(identity) = try_extract_identity(&jar, &kv).await? {
        Secp256k1Identity::from_private_key(identity)
    } else {
        return Ok(None);
    };

    Ok(Some(delegate_identity(&base_identity)))
}

pub async fn logout_identity_impl() -> Result<DelegatedIdentityWire, ServerFnError> {
    let key: Key = expect_context();
    let kv: KVStoreImpl = expect_context();
    let jar: SignedCookieJar = extract_with_state(&key).await?;
    let base_identity = generate_and_save_identity(&kv).await?;

    let resp: ResponseOptions = expect_context();
    let delegated = update_user_identity_and_delegate(&resp, jar, base_identity)?;
    Ok(delegated)
}

pub async fn generate_anonymous_identity_if_required_impl(
) -> Result<Option<JwkEcKey>, ServerFnError> {
    let key: Key = expect_context();
    let jar: SignedCookieJar = extract_with_state(&key).await?;
    if extract_principal_from_cookie(&jar)?.is_some() {
        return Ok(None);
    }

    let key = k256::SecretKey::random(&mut OsRng);
    Ok(Some(key.to_jwk()))
}

pub async fn set_anonymous_identity_cookie_impl(
    anonymous_identity: JwkEcKey,
) -> Result<(), ServerFnError> {
    let key: Key = expect_context();
    let jar: SignedCookieJar = extract_with_state(&key).await?;

    let kv: KVStoreImpl = expect_context();
    let base_identity = save_identity(&kv, anonymous_identity).await?;

    let resp: ResponseOptions = expect_context();
    update_user_identity(&resp, jar, &base_identity)?;

    Ok(())
}
