use candid::Principal;
use ic_agent::Identity;
use leptos::{html::Iframe, *};
use leptos_use::{
    storage::use_local_storage, use_event_listener, use_window, utils::FromToStringCodec,
};
use reqwest::Url;

use crate::{
    consts::{self, ACCOUNT_CONNECTED_STORE, AUTH_URL},
    state::{
        auth::{
            auth_state,
            types::{DelegationIdentity, SessionResponse},
        },
        canisters::{do_canister_auth, Canisters},
        local_storage::use_referrer_store,
    },
};

use self::set_referrer_impl::set_referrer;

#[server]
async fn issue_referral_rewards(referee_canister: Principal) -> Result<(), ServerFnError> {
    use self::server_fn_impl::issue_referral_rewards_impl;
    issue_referral_rewards_impl(referee_canister).await
}

#[server]
async fn mark_user_registered(user_principal: Principal) -> Result<(), ServerFnError> {
    use self::server_fn_impl::mark_user_registered_impl;
    use crate::state::canisters::unauth_canisters;

    // TODO: verify that user principal is registered
    let canisters = unauth_canisters();
    let user_canister = canisters
        .get_individual_canister_by_user_principal(user_principal)
        .await?
        .ok_or_else(|| ServerFnError::new("User not found"))?;
    mark_user_registered_impl(user_canister).await?;
    Ok(())
}

async fn handle_user_login(
    canisters: Canisters<true>,
    referrer: Option<Principal>,
) -> Result<(), ServerFnError> {
    let user_principal = canisters.identity().sender().unwrap();
    mark_user_registered(user_principal).await?;
    let Some(referrer) = referrer else {
        return Ok(());
    };
    let Some(referrer_canister) = canisters
        .get_individual_canister_by_user_principal(referrer)
        .await?
    else {
        return Ok(());
    };
    set_referrer(&canisters, referrer, referrer_canister).await?;

    issue_referral_rewards(canisters.user_canister()).await?;

    Ok(())
}

#[component]
pub fn ConnectLogin(
    #[prop(optional, default = "Login")] login_text: &'static str,
) -> impl IntoView {
    let (_, write_account_connected, _) =
        use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
    let logging_in = create_rw_signal(false);
    let auth = auth_state().identity;

    let iframe_ref = create_node_ref::<Iframe>();

    let (referrer_store, _, _) = use_referrer_store();

    let handle_user_login_action = create_action(move |identity: &DelegationIdentity| {
        let identity = identity.clone();
        async move {
            // This is some redundant work, but saves us 100+ lines of resource handling
            let canisters = do_canister_auth(Some(identity)).await?.unwrap();

            if let Err(e) = handle_user_login(canisters, referrer_store.get_untracked()).await {
                log::warn!("failed to handle user login, err {e}. skipping");
            }
            logging_in.set(false);
            write_account_connected.set(true);
            Ok::<_, ServerFnError>(())
        }
    });

    create_effect(move |_| {
        if auth.with(|a| a.is_none()) {
            return;
        }
        _ = use_event_listener(use_window(), ev::message, move |msg| {
            if Url::parse(&msg.origin())
                .map(|u| u.origin() != consts::AUTH_URL.origin())
                .unwrap_or_default()
            {
                return;
            }
            let data = msg.data().as_string().unwrap();
            let res: SessionResponse = serde_json::from_str(&data).unwrap();
            let identity = res.delegation_identity;
            auth.set(Some(identity.clone()));
            handle_user_login_action.dispatch(identity);
        });
    });

    _ = use_event_listener(iframe_ref, ev::load, move |_| {
        let iframe_node = iframe_ref().unwrap();
        let iframe_w = iframe_node.content_window().unwrap();
        _ = iframe_w.post_message(&("login".into()), "*");
    });

    view! {
        <button
            on:click=move |ev| {
                ev.prevent_default();
                ev.stop_propagation();
                logging_in.set(true);
            }

            class="font-bold rounded-full bg-primary-600 py-2 md:py-3 w-full text-center text-lg md:text-xl text-white"
            disabled=move || logging_in() || auth.with(|a| a.is_none())
        >
            {move || if logging_in() { "Connecting..." } else { login_text }}

        </button>
        <Show when=logging_in>
            <iframe
                _ref=iframe_ref
                class="hidden w-0 h-0"
                src=AUTH_URL.join("auth_init").unwrap().to_string()
            ></iframe>
        </Show>
    }
}

mod set_referrer_impl {
    use crate::state::canisters::Canisters;
    use candid::Principal;
    use leptos::ServerFnError;

    #[cfg(feature = "backend-admin")]
    pub async fn set_referrer(
        canisters: &Canisters<true>,
        referrer: Principal,
        referrer_canister: Principal,
    ) -> Result<(), ServerFnError> {
        use crate::canister::individual_user_template::{Result8, UserCanisterDetails};

        let user = canisters.authenticated_user();
        user.update_referrer_details(UserCanisterDetails {
            user_canister_id: referrer_canister,
            profile_owner: referrer,
        })
        .await
        .map_err(ServerFnError::from)
        .and_then(|res| match res {
            Result8::Ok(_) => Ok(()),
            Result8::Err(e) => Err(ServerFnError::new(format!("failed to set referrer {e}"))),
        })
    }

    #[cfg(not(feature = "backend-admin"))]
    pub async fn set_referrer(
        _canisters: &Canisters<true>,
        _referrer: Principal,
        _referrer_canister: Principal,
    ) -> Result<(), ServerFnError> {
        Ok(())
    }
}

#[cfg(feature = "ssr")]
mod server_fn_impl {
    #[cfg(feature = "backend-admin")]
    pub use backend_admin::*;
    #[cfg(not(feature = "backend-admin"))]
    pub use mock::*;

    #[cfg(feature = "backend-admin")]
    mod backend_admin {
        use candid::Principal;
        use leptos::ServerFnError;

        use crate::{
            canister::individual_user_template::KnownPrincipalType,
            state::canisters::unauth_canisters,
        };

        pub async fn issue_referral_rewards_impl(
            referee_canister: Principal,
        ) -> Result<(), ServerFnError> {
            let canisters = unauth_canisters();
            let user = canisters.individual_user(referee_canister);

            let user_details = user.get_profile_details().await?;
            let ref_details = user_details
                .referrer_details
                .ok_or_else(|| ServerFnError::new("Referral details for user not found"))?;
            let referrer = canisters.individual_user(ref_details.user_canister_id);

            let referrer_index_principal = referrer
                .get_well_known_principal_value(KnownPrincipalType::CanisterIdUserIndex)
                .await?
                .ok_or_else(|| ServerFnError::new("User index not present in referrer"))?;
            let user_index_principal = user
                .get_well_known_principal_value(KnownPrincipalType::CanisterIdUserIndex)
                .await?
                .ok_or_else(|| ServerFnError::new("User index not present in referee"))?;

            issue_referral_reward_for(
                user_index_principal,
                referee_canister,
                ref_details.profile_owner,
                user_details.principal_id,
            )
            .await?;
            issue_referral_reward_for(
                referrer_index_principal,
                ref_details.user_canister_id,
                ref_details.profile_owner,
                user_details.principal_id,
            )
            .await?;

            Ok(())
        }

        async fn issue_referral_reward_for(
            user_index: Principal,
            user_canister_id: Principal,
            referrer_principal_id: Principal,
            referee_principal_id: Principal,
        ) -> Result<(), ServerFnError> {
            use crate::{canister::user_index::Result_, state::admin_canisters::admin_canisters};

            let admin_cans = admin_canisters();
            let user_idx = admin_cans.user_index_with(user_index);
            let res = user_idx
                .issue_rewards_for_referral(
                    user_canister_id,
                    referrer_principal_id,
                    referee_principal_id,
                )
                .await?;
            if let Result_::Err(e) = res {
                return Err(ServerFnError::new(format!(
                    "failed to issue referral reward {e}"
                )));
            }
            Ok(())
        }

        pub async fn mark_user_registered_impl(
            user_canister: Principal,
        ) -> Result<(), ServerFnError> {
            use crate::{
                canister::individual_user_template::{Result6, Result8, SessionType},
                state::admin_canisters::admin_canisters,
            };

            let admin_cans = admin_canisters();
            let user = admin_cans.individual_user_for(user_canister);
            if matches!(
                user.get_session_type().await?,
                Result6::Ok(SessionType::RegisteredSession)
            ) {
                return Ok(());
            }
            user.update_session_type(SessionType::RegisteredSession)
                .await
                .map_err(ServerFnError::from)
                .and_then(|res| match res {
                    Result8::Ok(_) => Ok(()),
                    Result8::Err(e) => Err(ServerFnError::new(format!(
                        "failed to mark user as registered {e}"
                    ))),
                })?;
            Ok(())
        }
    }

    #[cfg(not(feature = "backend-admin"))]
    mod mock {
        use candid::Principal;
        use leptos::ServerFnError;

        pub async fn issue_referral_rewards_impl(
            _referee_canister: Principal,
        ) -> Result<(), ServerFnError> {
            Ok(())
        }

        pub async fn mark_user_registered_impl(
            _user_canister: Principal,
        ) -> Result<(), ServerFnError> {
            Ok(())
        }
    }
}
