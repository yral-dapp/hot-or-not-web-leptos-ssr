use reqwest::Url;
use std::sync::LazyLock;

pub static METADATA_API_BASE: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://yral-metadata.fly.dev").unwrap());

pub const AGENT_URL: &str = "https://ic0.app";
