use leptos::prelude::*;
use yral_types::delegated_identity::DelegatedIdentityWire;

pub type AuthState = RwSignal<Option<DelegatedIdentityWire>>;

pub fn auth_state() -> AuthState {
    expect_context()
}
