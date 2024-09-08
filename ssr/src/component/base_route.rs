use candid::Principal;
use ic_agent::identity::Secp256k1Identity;
use k256::elliptic_curve::JwkEcKey;
use leptos::*;
use leptos_router::*;

use crate::consts::USER_CANISTER_ID_STORE;
use crate::state::auth::{auth_client, auth_state};
use yral_auth_client::types::DelegatedIdentityWire;
use crate::{
    auth::{
        extract_identity, generate_anonymous_identity_if_required, set_anonymous_identity_cookie,
    },
    component::spinner::FullScreenSpinner,
    component::auth_providers::AuthProvider,
    state::{
        auth::AuthState,
        canisters::{do_canister_auth, AuthCansResource, Canisters},
        local_storage::use_referrer_store,
    },
    try_or_redirect,
    utils::MockPartialEq,
};
use codee::string::JsonSerdeCodec;
use leptos_use::storage::use_local_storage;

#[derive(Params, PartialEq, Clone)]
struct Referrer {
    user_refer: String,
}

#[component]
fn CtxProvider(children: ChildrenFn) -> impl IntoView {
    let canisters_store = create_rw_signal(None::<Canisters<true>>);
    provide_context(canisters_store);
    let referrer_query = use_query::<Referrer>();
    let referrer_principal = Signal::derive(move || {
        referrer_query()
            .ok()
            .and_then(|r| Principal::from_text(r.user_refer).ok())
    });
    let (referrer_store, set_referrer_store, _) = use_referrer_store();
    create_effect(move |_| {
        if referrer_store.get_untracked().is_some() {
            return;
        }
        set_referrer_store(referrer_principal.get_untracked())
    });

    let auth = auth_state();

    let canisters_res: AuthCansResource = create_resource(
        move || MockPartialEq(auth.identity.get()),
        move |auth_id| {
            async move {
                let ref_principal = referrer_principal.get_untracked();
                let res = do_canister_auth(auth_id.0, ref_principal).await;
                res
            }
        },
    );
    provide_context(canisters_res);

    view! {
        {children}
        <Suspense>
            {move || {
                canisters_res()
                    .map(|res| {
                        let cans_wire = try_or_redirect!(res);
                        let Some(cans_wire) = cans_wire else {
                            return;
                        };
                        let cans = try_or_redirect!(cans_wire.canisters());
                        let (_, set_user_canister_id, _) = use_local_storage::<
                            Option<Principal>,
                            JsonSerdeCodec,
                        >(USER_CANISTER_ID_STORE);
                        set_user_canister_id(Some(cans.user_canister()));
                        canisters_store.set(Some(cans));
                    })
            }}

        </Suspense>
    }
}

#[component]
pub fn BaseRoute() -> impl IntoView {

    view! {
        <CtxProvider>
            <Outlet/>
        </CtxProvider>
        <AuthProvider/>
    }
}
