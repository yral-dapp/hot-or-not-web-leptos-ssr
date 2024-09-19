use codee::string::FromToStringCodec;
use leptos::*;
use leptos_router::Redirect;
use leptos_use::storage::use_local_storage;

use crate::{
    auth::logout_identity,
    component::loading::Loading,
    consts::ACCOUNT_CONNECTED_STORE,
    state::{auth::auth_state, canisters::auth_canisters_store},
    try_or_redirect_opt,
    utils::event_streaming::events::{LogoutClicked, LogoutConfirmation},
};

#[component]
pub fn Logout() -> impl IntoView {
    let canister_store = auth_canisters_store();

    LogoutClicked.send_event(canister_store);
    let auth = auth_state();

    let auth_res = create_blocking_resource(
        || (),
        move |_| async move {
            let id = try_or_redirect_opt!(logout_identity().await);

            LogoutConfirmation.send_event(canister_store);

            let (_, write_account_connected, _) =
                use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
            write_account_connected(false);
            Some(id)
        },
    );

    view! {
        <Loading text="Logging out...".to_string()>
            <Suspense>
                {move || {
                    auth_res
                        .get()
                        .flatten()
                        .map(|id| {
                            auth.set(Some(id));
                            view! { <Redirect path="/menu"/> }
                        })
                }}

            </Suspense>
        </Loading>
    }
}
