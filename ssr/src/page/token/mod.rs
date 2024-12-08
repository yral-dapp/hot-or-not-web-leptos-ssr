pub mod create;
pub mod create_token_faq;
pub mod info;
mod popups;
mod sns_form;
pub mod transfer;
pub mod types;

use leptos::Params;
use leptos_router::Params;
use yral_canisters_common::utils::token::RootType;

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: RootType,
}

#[derive(Params, PartialEq, Clone)]
pub struct TokenInfoParams {
    pub token_root: RootType,
}
