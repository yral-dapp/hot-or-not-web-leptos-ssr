use once_cell::sync::Lazy;
use reqwest::Url;

pub static METADATA_API_BASE: Lazy<Url> =
    Lazy::new(|| Url::parse("http://localhost:8001").unwrap());

pub const AGENT_URL: &str = "http://localhost:4943";
