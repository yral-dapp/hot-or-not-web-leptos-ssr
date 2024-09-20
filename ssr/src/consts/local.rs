use once_cell::sync::Lazy;
use reqwest::Url;

pub static METADATA_API_BASE: Lazy<Url> =
    Lazy::new(|| Url::parse("http://localhost:8001").unwrap());

pub const AGENT_URL: &str = "http://localhost:4943";

pub const YRAL_BACKEND_CONTAINER_TAG: &str = "692e419da7e96c9a7d0e20fe89287460df795cea";
pub const YRAL_METADATA_CONTAINER_TAG: &str = "a4879e2e711c17beeb12ed6987ba315c110be9e5";
