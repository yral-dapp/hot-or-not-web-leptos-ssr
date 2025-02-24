pub mod create;
pub mod create_token_faq;
pub mod info;
mod popups;
mod sns_form;
pub mod transfer;
pub mod types;

use leptos_router::params::Params;
use leptos::prelude::*;
use yral_canisters_common::utils::token::RootType;

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: RootType,
}

#[derive(Params, PartialEq, Clone)]
struct TokenInfoParams {
    token_root: RootType,
}
