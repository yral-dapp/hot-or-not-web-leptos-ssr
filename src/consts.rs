use once_cell::sync::Lazy;
use reqwest::Url;

pub const CF_STREAM_BASE: &str = "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com";
pub const FALLBACK_PROPIC_BASE: &str = "https://api.dicebear.com/7.x/big-smile/svg";
pub const CF_WATERMARK_UID: &str = "28c721e45583a215d7b2ec1ae16e2679";
pub static CF_BASE_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://api.cloudflare.com/client/v4/").unwrap());
pub static AUTH_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://hot-or-not-auth.fly.dev/").unwrap());
