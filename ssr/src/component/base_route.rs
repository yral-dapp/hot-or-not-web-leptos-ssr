use candid::Principal;
use ic_agent::identity::Secp256k1Identity;
use k256::elliptic_curve::JwkEcKey;
use leptos::prelude::*;
use leptos_router::components::Outlet;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;
use leptos_use::use_cookie;

use crate::auth::delegate_identity;
use crate::consts::{ACCOUNT_CONNECTED_STORE, USER_CANISTER_ID_STORE, USER_PRINCIPAL_STORE};
use crate::utils::send_wrap;
use crate::{
    auth::{
        extract_identity, generate_anonymous_identity_if_required, set_anonymous_identity_cookie,
    },
    component::spinner::FullScreenSpinner,
    state::{
        auth::AuthState,
        canisters::{do_canister_auth, AuthCansResource},
        local_storage::use_referrer_store,
    },
    try_or_redirect,
    utils::MockPartialEq,
};
use codee::string::{FromToStringCodec, JsonSerdeCodec};
use leptos_use::storage::use_local_storage;
use yral_canisters_common::Canisters;

#[derive(Params, PartialEq, Clone)]
struct Referrer {
    user_refer: String,
}

#[component]
fn CtxProvider(temp_identity: Option<JwkEcKey>, children: ChildrenFn) -> impl IntoView {
    let auth = AuthState::default();
    provide_context(auth);

    let canisters_store = RwSignal::new(None::<Canisters<true>>);
    provide_context(canisters_store);

    let new_identity_issued = temp_identity.is_some();
    let temp_identity_c = temp_identity.clone();
    LocalResource::new(move || {
        let temp_identity = temp_identity_c.clone();
        async move {
            let Some(id) = temp_identity else {
                return;
            };
            if let Err(e) = set_anonymous_identity_cookie(id).await {
                log::error!("Failed to set anonymous identity as cookie?! err {e}");
            }
        }
    });

    let referrer_query = use_query::<Referrer>();
    let referrer_principal = Signal::derive(move || {
        referrer_query()
            .ok()
            .and_then(|r| Principal::from_text(r.user_refer).ok())
    });
    let (referrer_store, set_referrer_store, _) = use_referrer_store();
    Effect::new(move || {
        if referrer_store.get_untracked().is_some() {
            return;
        }
        set_referrer_store(referrer_principal.get_untracked())
    });

    // We need to perform this cleanup in case the user's cookie expired
    let (_, set_logged_in, _) =
        use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
    let (_, set_user_canister_id, _) =
        use_local_storage::<Option<Principal>, JsonSerdeCodec>(USER_CANISTER_ID_STORE);
    let (_, set_user_principal) = use_cookie::<Principal, FromToStringCodec>(USER_PRINCIPAL_STORE);
    Effect::new(move || {
        if new_identity_issued {
            set_logged_in(false);
            set_user_canister_id(None);
            set_user_principal(None);
        }
    });

    let canisters_res: AuthCansResource = Resource::new(
        move || MockPartialEq(auth()),
        move |auth_id| {
            let temp_identity = temp_identity.clone();
            async move {
                let ref_principal = referrer_principal.get_untracked();

                if let Some(id_wire) = auth_id.0 {
                    return do_canister_auth(id_wire, ref_principal).await;
                }

                let Some(jwk_key) = temp_identity else {
                    let id_wire = extract_identity().await?.expect("No refresh cookie set?!");
                    return do_canister_auth(id_wire, ref_principal).await;
                };

                let key = k256::SecretKey::from_jwk(&jwk_key)?;
                let id = Secp256k1Identity::from_private_key(key);
                let id_wire = delegate_identity(&id);

                do_canister_auth(id_wire, ref_principal).await
            }
        },
    );
    provide_context(canisters_res);

    view! {
        {children()}
        <Suspense>
            {move || {
                canisters_res.get()
                    .map(|res| {
                        let cans_wire = try_or_redirect!(res);
                        let maybe_cans = Canisters::from_wire(cans_wire, expect_context());
                        let cans = try_or_redirect!(maybe_cans);
                        let user_canister = cans.user_canister();
                        let user_principal = cans.user_principal();
                        Effect::new(move || {
                            set_user_canister_id(Some(user_canister));
                            set_user_principal(Some(user_principal));
                        });
                        canisters_store.set(Some(cans));
                    })
            }}

        </Suspense>
    }
}

#[component]
pub fn BaseRoute() -> impl IntoView {
    let temp_identity_res = Resource::new_blocking(
        || (),
        |_| async move {
            generate_anonymous_identity_if_required()
                .await
                .expect("Failed to generate anonymous identity?!")
        },
    );

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                temp_identity_res.get()
                    .map(|temp_identity| {
                        view! {
                            <CtxProvider temp_identity>
                                <Outlet />
                            </CtxProvider>
                        }
                    })
            }}

        </Suspense>
    }
}
