pub mod create;
pub mod create_token_faq;
pub mod info;
pub mod non_yral_tokens;
mod popups;
mod sns_form;
pub mod transfer;
pub mod types;

use leptos::Params;
use leptos_router::Params;

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: String,
}

#[derive(Params, PartialEq, Clone)]
struct TokenInfoParams {
    token_root: String,
}
