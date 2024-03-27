#[cfg(feature = "local-auth")]
mod local_storage;

use candid::Principal;
use ic_agent::Identity;
use leptos::*;
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

use crate::{
    auth::DelegatedIdentityWire,
    consts::ACCOUNT_CONNECTED_STORE,
    state::{
        auth::auth_state,
        canisters::{do_canister_auth, Canisters},
        local_storage::use_referrer_store,
    },
};

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
    use self::set_referrer_impl::set_referrer;

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

pub struct LoginProvCtx {
    pub processing: RwSignal<bool>,
    pub login_complete: SignalSetter<DelegatedIdentityWire>,
}

#[component]
pub fn LoginProviders(show_modal: RwSignal<bool>) -> impl IntoView {
    let auth = auth_state();
    let (_, write_account_connected, _) =
        use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);

    let (referrer_store, _, _) = use_referrer_store();
    let handle_user_login_action = create_action(move |identity: &DelegatedIdentityWire| {
        let identity = identity.clone();
        async move {
            // This is some redundant work, but saves us 100+ lines of resource handling
            let canisters = do_canister_auth(Some(identity)).await?.unwrap();

            if let Err(e) = handle_user_login(canisters, referrer_store.get_untracked()).await {
                log::warn!("failed to handle user login, err {e}. skipping");
            }
            Ok::<_, ServerFnError>(())
        }
    });

    let ctx = LoginProvCtx {
        processing: RwSignal::new(false),
        login_complete: SignalSetter::map(move |val: DelegatedIdentityWire| {
            write_account_connected(true);
            handle_user_login_action.dispatch(val.clone());
            auth.identity.set(Some(val));
            show_modal.set(false)
        }),
    };

    view! {
        <div class="flex w-full gap-4">

            {
                #[cfg(feature = "local-auth")]
                view! {
                    <local_storage::LocalStorageProvider ctx></local_storage::LocalStorageProvider>
                }
            }

        </div>
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
