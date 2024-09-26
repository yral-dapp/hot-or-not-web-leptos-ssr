#[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
pub use yral_canisters_client::ic::*;
#[cfg(any(feature = "local-bin", feature = "local-lib"))]
pub use yral_canisters_client::local::*;
