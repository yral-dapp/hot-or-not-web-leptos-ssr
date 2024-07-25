use candid::Principal;
use leptos::{Signal, WriteSignal};
use leptos_use::{storage::use_local_storage, utils::JsonCodec};

use crate::consts::REFERRER_STORE;

pub fn use_referrer_store() -> (
    Signal<Option<Principal>>,
    WriteSignal<Option<Principal>>,
    impl Fn() + Clone,
) {
    use_local_storage::<Option<Principal>, JsonCodec>(REFERRER_STORE)
}
