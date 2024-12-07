#![allow(clippy::empty_docs)]
pub mod app;
pub mod auth;
pub mod canister_ids;
pub mod component;
pub mod consts;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod init;
pub mod page;
pub mod state;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;
    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
