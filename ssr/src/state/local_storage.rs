use candid::Principal;
use codee::string::JsonSerdeCodec;
use leptos::prelude::{Signal, WriteSignal};
use leptos_use::storage::use_local_storage;

use crate::consts::REFERRER_STORE;

pub fn use_referrer_store() -> (
    Signal<Option<Principal>>,
    WriteSignal<Option<Principal>>,
    impl Fn() + Clone,
) {
    use_local_storage::<Option<Principal>, JsonSerdeCodec>(REFERRER_STORE)
}
