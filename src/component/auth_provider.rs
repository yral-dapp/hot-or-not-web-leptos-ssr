use crate::{
    state::auth::{auth_client, auth_state},
    try_or_redirect,
};
use leptos::*;
use yral_auth_client::types::{DelegatedIdentityWire, SignedRefreshTokenClaim};

#[server]
async fn generate_claim_for_migration() -> Result<Option<SignedRefreshTokenClaim>, ServerFnError> {
    use crate::utils::current_epoch;
    use axum_extra::extract::cookie::{Key, SignedCookieJar};
    use candid::Principal;
    use hmac::{Hmac, Mac};
    use leptos_axum::{extract_with_state, ResponseOptions};
    use serde::Deserialize;
    use web_time::Duration;
    use yral_auth_client::types::RefreshTokenClaim;

    #[derive(Deserialize)]
    struct RefreshToken {
        principal: Principal,
        expiry_epoch_ms: u128,
    }

    let key: Key = expect_context();
    let jar: SignedCookieJar = extract_with_state(&key).await?;
    let Some(cookie) = jar.get("user-identity") else {
        return Ok(None);
    };
    let Some(host) = cookie.domain() else {
        return Ok(None);
    };

    let token: RefreshToken = serde_json::from_str(cookie.value())?;
    if current_epoch().as_millis() > token.expiry_epoch_ms {
        return Ok(None);
    }

    let signing = key.signing();
    let mut mac = Hmac::<sha2::Sha256>::new_from_slice(signing)?;

    let claim = RefreshTokenClaim {
        principal: token.principal,
        expiry_epoch: current_epoch() + Duration::from_secs(300),
        referrer_host: url::Host::parse(host)?,
    };
    let raw = serde_json::to_vec(&claim)?;
    mac.update(&raw);
    let digest = mac.finalize().into_bytes();

    let resp: ResponseOptions = expect_context();
    resp.insert_header(
        "Set-Cookie".parse()?,
        "user-identity=; Max-Age=0; Path=/; Secure; HttpOnly; SameSite=None".parse()?,
    );

    Ok(Some(SignedRefreshTokenClaim {
        claim,
        digest: digest.to_vec(),
    }))
}

#[component]
pub fn AuthFrame(auth: RwSignal<Option<DelegatedIdentityWire>>) -> impl IntoView {
    let auth_res = create_local_resource(
        || (),
        move |_| async move {
            let auth_c = auth_client();
            let id = if let Some(claim) = try_or_redirect!(generate_claim_for_migration().await) {
                try_or_redirect!(auth_c.upgrade_refresh_token_claim(claim).await)
            } else {
                try_or_redirect!(auth_c.extract_or_generate_identity().await)
            };
            auth.set(Some(id));
        },
    );

    view! { <Suspense>{move || auth_res.get().map(|_| ())}</Suspense> }
}

#[component]
pub fn AuthProvider() -> impl IntoView {
    let auth = auth_state().identity;
    view! {
        <div class="hidden">
            <Show when=move || auth.with(|a| a.is_none())>
                <AuthFrame auth/>
            </Show>
        </div>
    }
}
