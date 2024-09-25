#[cfg(not(feature = "local-bin"))]
pub use yral_canisters_client::ic::*;
#[cfg(feature = "local-bin")]
pub use yral_canisters_client::local::*;
