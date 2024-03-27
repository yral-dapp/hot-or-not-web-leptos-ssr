use std::env;

use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
};
use axum::{routing::get, Router};
use axum_extra::extract::cookie::Key;
use hot_or_not_web_leptos_ssr::state::canisters::Canisters;
use hot_or_not_web_leptos_ssr::{app::App, state::server::AppState};
use hot_or_not_web_leptos_ssr::{
    auth::server_impl::store::KVStoreImpl, fallback::file_and_error_handler,
};
use leptos::{get_configuration, logging::log, provide_context};
use leptos_axum::handle_server_fns_with_context;
use leptos_axum::{generate_route_list, LeptosRoutes};

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log!("{:?}", path);

    handle_server_fns_with_context(
        move || {
            provide_context(app_state.canisters.clone());
            #[cfg(feature = "backend-admin")]
            provide_context(app_state.admin_canisters.clone());
            #[cfg(feature = "cloudflare")]
            provide_context(app_state.cloudflare.clone());
            provide_context(app_state.kv.clone());
            provide_context(app_state.cookie_key.clone());
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.canisters.clone());
            #[cfg(feature = "backend-admin")]
            provide_context(app_state.admin_canisters.clone());
            #[cfg(feature = "cloudflare")]
            provide_context(app_state.cloudflare.clone());
            provide_context(app_state.kv.clone());
            provide_context(app_state.cookie_key.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

#[cfg(feature = "cloudflare")]
fn init_cf() -> gob_cloudflare::CloudflareAuth {
    use gob_cloudflare::{CloudflareAuth, Credentials};
    let creds = Credentials {
        token: env::var("CF_TOKEN").expect("`CF_TOKEN` is required!"),
        account_id: env::var("CF_ACCOUNT_ID").expect("`CF_ACCOUNT_ID` is required!"),
    };
    CloudflareAuth::new(creds)
}

#[cfg(feature = "backend-admin")]
fn init_admin_canisters() -> hot_or_not_web_leptos_ssr::state::admin_canisters::AdminCanisters {
    use hot_or_not_web_leptos_ssr::state::admin_canisters::AdminCanisters;
    use ic_agent::identity::BasicIdentity;

    let admin_id_pem =
        env::var("BACKEND_ADMIN_IDENTITY").expect("`BACKEND_ADMIN_IDENTITY` is required!");
    let admin_id_pem_by = admin_id_pem.as_bytes();
    let admin_id =
        BasicIdentity::from_pem(admin_id_pem_by).expect("Invalid `BACKEND_ADMIN_IDENTITY`");
    AdminCanisters::new(admin_id)
}

fn init_kv() -> KVStoreImpl {
    #[cfg(feature = "cloudflare")]
    {
        unimplemented!("Cloudflare KV is not implemented")
    }

    #[cfg(not(feature = "cloudflare"))]
    {
        use hot_or_not_web_leptos_ssr::auth::server_impl::store::redb_kv::ReDBKV;
        KVStoreImpl::ReDB(ReDBKV::new().expect("Failed to initialize ReDB"))
    }
}

fn init_cookie_key() -> Key {
    let cookie_key_str = env::var("COOKIE_KEY").expect("`COOKIE_KEY` is required!");
    let cookie_key_raw =
        hex::decode(cookie_key_str).expect("Invalid `COOKIE_KEY` (must be length 128 hex)");
    Key::from(&cookie_key_raw)
}

#[cfg(feature = "oauth-provider")]
fn init_google_oauth() -> oauth2::basic::BasicClient {
    unimplemented!("Google OAuth is not implemented")
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
    dotenv::dotenv().expect("couldn't load .env file");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        canisters: Canisters::default(),
        routes: routes.clone(),
        #[cfg(feature = "backend-admin")]
        admin_canisters: init_admin_canisters(),
        #[cfg(feature = "cloudflare")]
        cloudflare: init_cf(),
        kv: init_kv(),
        cookie_key: init_cookie_key(),
        #[cfg(feature = "oauth-provider")]
        google_oauth: init_google_oauth(),
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
