use leptos::*;
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

use crate::{
    auth::{extract_or_generate_identity, DelegatedIdentityWire},
    consts::ACCOUNT_CONNECTED_STORE,
    try_or_redirect_opt,
};

pub fn auth_resource() -> Resource<(), Option<DelegatedIdentityWire>> {
    create_blocking_resource(
        || (),
        |_| async move {
            let id = try_or_redirect_opt!(extract_or_generate_identity().await);
            Some(id)
        },
    )
}

pub type AuthState = RwSignal<DelegatedIdentityWire>;

pub fn auth_state() -> AuthState {
    expect_context()
}

/// Prevents hydration bugs if the value in store is used to conditionally show views
/// this is because the server will always get a `false` value and do rendering based on that
pub fn account_connected_reader() -> (ReadSignal<bool>, Effect<()>) {
    let (read_account_connected, _, _) =
        use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
    let (is_connected, set_is_connected) = create_signal(false);

    (
        is_connected,
        create_effect(move |_| {
            set_is_connected(read_account_connected());
        }),
    )
}
