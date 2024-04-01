use leptos::*;
use leptos_router::use_navigate;
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

use crate::{
    auth::logout_identity, consts::ACCOUNT_CONNECTED_STORE, state::auth::auth_state,
    try_or_redirect,
};

#[component]
pub fn Logout() -> impl IntoView {
    let auth_res = create_local_resource(
        || (),
        move |_| async move {
            let _ = try_or_redirect!(logout_identity().await);

            let auth = auth_state().identity;
            auth.set(None);

            let (_, write_account_connected, _) =
                use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
            write_account_connected(false);

            let navigate = use_navigate();
            navigate("/menu", Default::default());
        },
    );

    view! { <Suspense>{move || auth_res.get().map(|_| ())}</Suspense> }
}
