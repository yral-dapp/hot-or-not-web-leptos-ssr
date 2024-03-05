use candid::Principal;
use leptos::*;
use leptos_router::*;

use super::auth_provider::AuthProvider;
use crate::{
    state::{auth::AuthState, canisters::do_canister_auth},
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
    let referrer_untracked = referrer_query
        .get_untracked()
        .ok()
        .and_then(|r| Principal::from_text(r.user_refer).ok());
    let auth_state = expect_context::<AuthState>();

    provide_context(Resource::local(
        move || MockPartialEq(auth_state.identity.get()),
        move |auth| do_canister_auth(auth.0, referrer_untracked),
    ));

    view! {
        <Outlet/>
        <AuthProvider/>
    }
}
