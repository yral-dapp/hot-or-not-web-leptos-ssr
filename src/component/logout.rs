use leptos::*;
use leptos_router::use_navigate;
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};
use serde_json::json;

use crate::{
    auth::logout_identity,
    component::loading::Loading,
    consts::ACCOUNT_CONNECTED_STORE,
    state::{auth::auth_state, canisters::AuthProfileCanisterResource},
    try_or_redirect,
};

#[component]
pub fn Logout() -> impl IntoView {
    let profile_and_canister_details: AuthProfileCanisterResource = expect_context();

    #[cfg(all(feature = "hydrate", feature = "ga4"))]
    {
        use crate::utils::event_streaming::send_event;

        let user_id = move || {
            profile_and_canister_details()
                .flatten()
                .map(|(q, _)| q.principal)
        };
        let display_name = move || {
            profile_and_canister_details()
                .flatten()
                .map(|(q, _)| q.display_name)
        };
        let canister_id = move || profile_and_canister_details().flatten().map(|(_, q)| q);

        send_event(
            "logout_clicked",
            &json!({
                "user_id_viewer": user_id(),
                "display_name": display_name(),
                "canister_id": canister_id(),
            }),
        );
    }

    let auth_res = create_local_resource(
        || (),
        move |_| async move {
            let _ = try_or_redirect!(logout_identity().await);

            #[cfg(all(feature = "hydrate", feature = "ga4"))]
            {
                use crate::utils::event_streaming::send_event;

                let user_id = move || {
                    profile_and_canister_details()
                        .flatten()
                        .map(|(q, _)| q.principal)
                };
                let display_name = move || {
                    profile_and_canister_details()
                        .flatten()
                        .map(|(q, _)| q.display_name)
                };
                let canister_id = move || profile_and_canister_details().flatten().map(|(_, q)| q);

                send_event(
                    "logout_confirmation",
                    &json!({
                        "user_id_viewer": user_id(),
                        "display_name": display_name(),
                        "canister_id": canister_id(),
                    }),
                );
            }

            let auth = auth_state().identity;
            auth.set(None);

            let (_, write_account_connected, _) =
                use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
            write_account_connected(false);

            let navigate = use_navigate();
            navigate("/menu", Default::default());
        },
    );

    view! {
        <Loading text="Logging out...".to_string()>
            <Suspense>{move || auth_res.get().map(|_| ())}</Suspense>
        </Loading>
    }
}
