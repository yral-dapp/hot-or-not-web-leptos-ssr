use std::sync::LazyLock;

use reqwest::Url;

pub static METADATA_API_BASE: LazyLock<Url> =
    LazyLock::new(|| Url::parse("http://localhost:8001").unwrap());

pub const AGENT_URL: &str = "http://localhost:4943";

pub const YRAL_BACKEND_CONTAINER_TAG: &str = "04b53277579d9370c13312a2833ca0b855cdad72";
pub const YRAL_METADATA_CONTAINER_TAG: &str = "a4879e2e711c17beeb12ed6987ba315c110be9e5";
