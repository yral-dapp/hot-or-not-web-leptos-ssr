use leptos::*;
use leptos_use::{storage::use_local_storage, utils::FromToStringCodec};

use crate::{auth::DelegatedIdentityWire, consts::ACCOUNT_CONNECTED_STORE};

pub type AuthState = RwSignal<Option<DelegatedIdentityWire>>;

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
