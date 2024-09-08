use once_cell::sync::Lazy;
use reqwest::Url;

pub static METADATA_API_BASE: Lazy<Url> =
    Lazy::new(|| Url::parse("https://yral-metadata.fly.dev").unwrap());

pub const AGENT_URL: &str = "https://ic0.app";

pub static AUTH_API_BASE: Lazy<Url> =
    Lazy::new(|| Url::parse("https://yral-auth.fly.dev/").unwrap());
