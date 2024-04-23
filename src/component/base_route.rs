use candid::Principal;
use leptos::*;
use leptos_router::*;

use crate::{
    auth::{extract_or_generate_identity, upgrade_temp_refresh_token, TempRefreshToken},
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
pub fn BaseRoute() -> impl IntoView {
    let auth = AuthState::default();
    let canisters_store = create_rw_signal(None::<Canisters<true>>);
    provide_context(canisters_store);

    let temp_refresh_token = create_rw_signal(None::<TempRefreshToken>);
    let canisters_res = create_resource(
        move || MockPartialEq(auth()),
        move |auth_id| async move {
            let (id, temp_tok) = if let Some(id) = auth_id.0 {
                (id, None)
            } else {
                let (id, temp_tok) = extract_or_generate_identity().await?;
                (id, Some(temp_tok))
            };
            let cans = do_canister_auth(id).await?;
            Ok((cans, temp_tok))
        },
    );
    let _refresh_token_upgrade =
        create_local_resource(temp_refresh_token, |temp_token| async move {
            let Some(temp_token) = temp_token else {
                return;
            };
            if let Err(e) = upgrade_temp_refresh_token(temp_token).await {
                log::warn!("failed to upgrade temp refresh token: {e}... ignoring");
            }
        });

    view! {
        <CtxProvider auth canisters_res>
            <Outlet/>
        </CtxProvider>
        <Suspense>
            {move || {
                canisters_res()
                    .map(|res| {
                        let (cans, temp_tok) = try_or_redirect!(res);
                        let cans = try_or_redirect!(cans.try_into());
                        canisters_store.set(Some(cans));
                        temp_refresh_token.set(temp_tok);
                    })
            }}

        </Suspense>
    }
}
