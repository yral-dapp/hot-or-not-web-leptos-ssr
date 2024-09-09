pub mod create;
pub mod info;
mod popups;
mod sns_form;
pub mod swapper;
pub mod transfer;

use candid::Principal;
use leptos::Params;
use leptos_router::Params;

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: Principal,
}
