pub mod auth;
pub mod canisters;
#[cfg(feature = "cloudflare")]
pub mod cf;
pub mod history;

#[cfg(feature = "ssr")]
pub mod server {
    use super::canisters::Canisters;
    use axum::extract::FromRef;
    use leptos::LeptosOptions;
    use leptos_router::RouteListing;

    #[derive(FromRef, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub canisters: Canisters<false>,
        #[cfg(feature = "cloudflare")]
        pub cloudflare: super::cf::CfApi<true>,
        pub routes: Vec<RouteListing>,
    }
}
