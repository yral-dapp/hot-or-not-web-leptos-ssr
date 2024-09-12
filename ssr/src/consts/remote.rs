use once_cell::sync::Lazy;
use reqwest::Url;

pub static METADATA_API_BASE: Lazy<Url> =
    Lazy::new(|| Url::parse("https://yral-metadata.fly.dev").unwrap());

pub const AGENT_URL: &str = "https://ic0.app";
pub const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
