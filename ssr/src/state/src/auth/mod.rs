use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::storage::use_local_storage;
use yral_types::delegated_identity::DelegatedIdentityWire;

use consts::ACCOUNT_CONNECTED_STORE;

pub type AuthState = RwSignal<Option<DelegatedIdentityWire>>;

pub fn auth_state() -> AuthState {
    expect_context()
}

