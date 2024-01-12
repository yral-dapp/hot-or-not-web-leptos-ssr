//! Auto generated bindings for canisters
#[allow(clippy::all)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/did/mod.rs"));
}

pub mod utils;
pub use generated::*;

pub const AGENT_URL: &str = "https://ic0.app";
