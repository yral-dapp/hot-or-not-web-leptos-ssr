use candid::Principal;
use leptos::*;
use leptos_router::*;

use super::auth_provider::AuthProvider;
use crate::{
    state::{
        auth::AuthState,
        canisters::{do_canister_auth, AuthCanistersResource},
        local_storage::use_referrer_store,
    },
    utils::MockPartialEq,
};

#[derive(Params, PartialEq, Clone)]
struct Referrer {
    user_refer: String,
}

/// Base route is technically rendered **adjacent** to all routes
/// do not use it for any parent -> child communication, such as passing global context
#[component]
pub fn BaseRoute() -> impl IntoView {
    let referrer_query = use_query::<Referrer>();
    let referrer_principal = move || {
        referrer_query()
            .ok()
            .and_then(|r| Principal::from_text(r.user_refer).ok())
    };
    let auth_state = expect_context::<AuthState>();

    let (referrer_store, set_referrer_store, _) = use_referrer_store();
    create_effect(move |_| {
        if referrer_store.get_untracked().is_some() {
            return;
        }
        let refp = referrer_principal();
        set_referrer_store.set(refp);
    });

    let auth_cans_res: AuthCanistersResource = Resource::local(
        move || MockPartialEq(auth_state.identity.get()),
        move |auth| do_canister_auth(auth.0),
    );

    provide_context(auth_cans_res);

    view! {
        <Outlet/>
        <AuthProvider/>
    }
}
