use once_cell::sync::Lazy;
use reqwest::Url;

pub static METADATA_API_BASE: Lazy<Url> =
    Lazy::new(|| Url::parse("http://localhost:8001").unwrap());

pub const AGENT_URL: &str = "http://localhost:4943";

pub const YRAL_BACKEND_CONTAINER_TAG: &str = "76bfd0fa78e4f862a4b30601f4ff3143aa974ee7";
pub const YRAL_METADATA_CONTAINER_TAG: &str = "a4879e2e711c17beeb12ed6987ba315c110be9e5";

pub const ICP_LEDGER_CANISTER_ID: &'static str = "ryjl3-tyaaa-aaaaa-aaaba-cai";