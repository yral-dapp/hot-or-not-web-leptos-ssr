use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_use::storage::use_local_storage;
use auth::logout_identity;
use component::loading::Loading;
use consts::ACCOUNT_CONNECTED_STORE;
use state::{auth::auth_state};
use utils::{event_streaming::events::{LogoutClicked, LogoutConfirmation, auth_canisters_store}, try_or_redirect_opt};

#[component]
pub fn Logout() -> impl IntoView {
    let canister_store = auth_canisters_store();

    LogoutClicked.send_event(canister_store);
    let auth = auth_state();

    let auth_res = Resource::new_blocking(
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
                            view! { <Redirect path="/menu" /> }
                        })
                }}

            </Suspense>
        </Loading>
    }
}
