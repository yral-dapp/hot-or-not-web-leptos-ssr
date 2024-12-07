#[cfg(any(feature = "local-bin", feature = "local-lib"))]
mod local;
use std::sync::LazyLock;

use candid::Principal;
#[cfg(any(feature = "local-bin", feature = "local-lib"))]
pub use local::*;

#[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
mod remote;
#[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
pub use remote::*;

use reqwest::Url;

pub const CF_STREAM_BASE: &str = "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com";
pub const FALLBACK_PROPIC_BASE: &str = "https://api.dicebear.com/7.x/big-smile/svg";
// an example URL is "https://imagedelivery.net/abXI9nS4DYYtyR1yFFtziA/gob.5/public";
pub const GOBGOB_PROPIC_URL: &str = "https://imagedelivery.net/abXI9nS4DYYtyR1yFFtziA/gob.";
pub const GOBGOB_TOTAL_COUNT: u32 = 18557;
pub const CF_WATERMARK_UID: &str = "b5588fa1516ca33a08ebfef06c8edb33";
pub const ACCOUNT_CONNECTED_STORE: &str = "account-connected-1";
pub static CF_BASE_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://api.cloudflare.com/client/v4/").unwrap());
pub const NOTIFICATIONS_ENABLED_STORE: &str = "yral-notifications-enabled";
pub const NSFW_TOGGLE_STORE: &str = "nsfw-enabled";
pub const REFERRER_STORE: &str = "referrer";
pub const USER_CANISTER_ID_STORE: &str = "user-canister-id";
pub const USER_PRINCIPAL_STORE: &str = "user-principal";
pub const USER_ONBOARDING_STORE: &str = "user-onboarding";

pub static OFF_CHAIN_AGENT_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://icp-off-chain-agent.fly.dev/").unwrap());
pub static OFF_CHAIN_AGENT_GRPC_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://icp-off-chain-agent.fly.dev:443").unwrap()); // pr-91-yral-dapp-off-chain-agent
                                                                                      // G-6W5Q2MRX0E to test locally | G-PLNNETMSLM
pub const GTAG_MEASUREMENT_ID: &str = "G-PLNNETMSLM";
pub static DOWNLOAD_UPLOAD_SERVICE: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://download-upload-service.fly.dev").unwrap());
pub const ML_FEED_GRPC_URL: &str = "https://yral-ml-feed-server.fly.dev:443"; // "http://localhost:50051";//

pub static FALLBACK_USER_INDEX: LazyLock<Principal> =
    LazyLock::new(|| Principal::from_text("rimrc-piaaa-aaaao-aaljq-cai").unwrap());

pub const ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

pub const ICPUMP_LISTING_PAGE_SIZE: usize = 12;

pub const CDAO_SWAP_PRE_READY_TIME_SECS: u64 = 150;

pub const CDAO_SWAP_TIME_SECS: u64 = CDAO_SWAP_PRE_READY_TIME_SECS + 150;

pub const ICPUMP_SEARCH_GRPC_URL: &str = "https://prod-yral-icpumpsearch.fly.dev:443";
pub const NSFW_SERVER_URL: &str = "https://prod-yral-nsfw-classification.fly.dev:443";

pub mod social {
    pub const TELEGRAM: &str = "https://t.me/+c-LTX0Cp-ENmMzI1";
    pub const DISCORD: &str = "https://discord.gg/GZ9QemnZuj";
    pub const TWITTER: &str = "https://twitter.com/Yral_app";
    pub const IC_WEBSITE: &str = "https://vyatz-hqaaa-aaaam-qauea-cai.ic0.app";
}

pub mod auth {
    use web_time::Duration;

    /// Delegation Expiry, 7 days
    pub const DELEGATION_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24 * 7);
    /// Refresh expiry, 30 days
    pub const REFRESH_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24 * 30);
    pub const REFRESH_TOKEN_COOKIE: &str = "user-identity";
}

#[cfg(feature = "oauth-ssr")]
pub mod google {
    pub const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
    pub const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
    pub const GOOGLE_ISSUER_URL: &str = "https://accounts.google.com";
}
