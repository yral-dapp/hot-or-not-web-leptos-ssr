use candid::Principal;
use leptos::*;
use leptos_router::*;

use crate::{
    auth::extract_or_generate_identity,
    state::{
        auth::AuthState,
        canisters::{do_canister_auth, AuthCansResource, Canisters},
        local_storage::use_referrer_store,
    },
    try_or_redirect,
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
    provide_context(auth);

    children()
}

#[component]
pub fn BaseRoute() -> impl IntoView {
    let auth = AuthState::default();
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

    let canisters_res = create_local_resource(
        move || MockPartialEq(auth()),
        move |auth_id| async move {
            let id = if let Some(id) = auth_id.0 {
                id
            } else {
                extract_or_generate_identity().await?
            };
            let cans = do_canister_auth(id, referrer_principal.get_untracked()).await?;
            Ok(cans)
        },
    );

    // for loading Canisters<true> for events

    view! {
        <CtxProvider auth canisters_res>
            <Outlet/>
        </CtxProvider>
        <Suspense>
            {move || {
                canisters_res()
                    .map(|res| {
                        let cans = try_or_redirect!(res);
                        canisters_store.set(Some(cans));
                    })
            }}

        </Suspense>
    }
}
