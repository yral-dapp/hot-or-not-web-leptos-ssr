use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use yral_types::delegated_identity::DelegatedIdentityWire;

use crate::consts::ACCOUNT_CONNECTED_STORE;

pub type AuthState = RwSignal<Option<DelegatedIdentityWire>>;

pub fn auth_state() -> AuthState {
    expect_context()
}

/// Prevents hydration bugs if the value in store is used to conditionally show views
/// this is because the server will always get a `false` value and do rendering based on that
pub fn account_connected_reader() -> (ReadSignal<bool>, Effect<LocalStorage>) {
    let (read_account_connected, _, _) =
        use_local_storage::<bool, FromToStringCodec>(ACCOUNT_CONNECTED_STORE);
    let (is_connected, set_is_connected) = signal(false);

    (
        is_connected,
        Effect::new(move |_| {
            set_is_connected(read_account_connected());
        }),
    )
}
