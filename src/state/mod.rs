#[cfg(feature = "backend-admin")]
pub mod admin_canisters;
pub mod auth;
pub mod canisters;
pub mod history;
pub mod local_storage;

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
        #[cfg(feature = "backend-admin")]
        pub admin_canisters: super::admin_canisters::AdminCanisters,
        #[cfg(feature = "cloudflare")]
        pub cloudflare: gob_cloudflare::CloudflareAuth,
        pub routes: Vec<RouteListing>,
    }
}
