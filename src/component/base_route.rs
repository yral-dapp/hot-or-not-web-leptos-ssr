use candid::Principal;
use leptos::*;
use leptos_router::*;

use crate::{
    auth::DelegatedIdentityWire,
    component::spinner::FullScreenSpinner,
    state::{
        auth::{auth_resource, AuthState},
        canisters::{do_canister_auth, AuthCansResource},
        local_storage::use_referrer_store,
    },
    try_or_redirect_opt,
    utils::MockPartialEq,
};

#[derive(Params, PartialEq, Clone)]
struct Referrer {
    user_refer: String,
}

#[component]
fn CtxProvider(
    auth: AuthState,
    canisters_res: AuthCansResource,
    children: Children,
) -> impl IntoView {
    provide_context(canisters_res);

    let referrer_query = use_query::<Referrer>();
    let referrer_principal = move || {
        referrer_query()
            .ok()
            .and_then(|r| Principal::from_text(r.user_refer).ok())
    };

    let (referrer_store, set_referrer_store, _) = use_referrer_store();
    create_effect(move |_| {
        if referrer_store.get_untracked().is_some() {
            return;
        }
        let refp = referrer_principal();
        set_referrer_store.set(refp);
    });
    provide_context(auth);

    children()
}

#[component]
fn CanistersProvider(id: DelegatedIdentityWire) -> impl IntoView {
    let auth = use_context().unwrap_or_else(move || AuthState::new(id));
    let canisters_store = create_rw_signal(None);
    provide_context(canisters_store);

    let canisters_res = create_resource(
        move || MockPartialEq(auth()),
        move |id| async move {
            let cans = try_or_redirect_opt!(do_canister_auth(id.0).await);
            canisters_store.set(Some(cans.clone()));
            Some(cans)
        },
    );

    view! {
        <CtxProvider auth canisters_res>
            <Outlet/>
        </CtxProvider>
    }
}

/// Base route is technically rendered **adjacent** to all routes
/// do not use it for any parent -> child communication, such as passing global context
#[component]
pub fn BaseRoute() -> impl IntoView {
    let id_res = auth_resource();

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                id_res()
                    .flatten()
                    .map(move |id| {
                        view! { <CanistersProvider id/> }
                    })
            }}

        </Suspense>
    }
}
