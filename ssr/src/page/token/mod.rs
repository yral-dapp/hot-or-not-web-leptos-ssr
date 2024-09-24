pub mod create;
pub mod create_token_faq;
pub mod info;
mod popups;
mod sns_form;
pub mod token_transaction;
pub mod transfer;
pub(super) mod txn;
pub mod types;

use candid::Principal;
use leptos::Params;
use leptos_router::Params;

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: Principal,
}

#[derive(Params, PartialEq, Clone)]
struct TokenInfoParams {
    token_root: Principal,
    user_principal: Principal,
}
