pub mod create;
pub mod create_token_faq;
pub mod info;
pub mod popups;
mod sns_form;
pub mod transfer;
pub mod types;

use candid::Principal;
use leptos::Params;
use leptos_router::Params;

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: Principal,
}
