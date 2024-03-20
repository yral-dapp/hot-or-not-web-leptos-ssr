use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
};
use axum::{routing::get, Router};
use hot_or_not_web_leptos_ssr::fallback::file_and_error_handler;
use hot_or_not_web_leptos_ssr::state::canisters::Canisters;
use hot_or_not_web_leptos_ssr::{app::App, state::server::AppState};
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
        },
        App,
    );
    handler(req).await.into_response()
}

#[cfg(feature = "cloudflare")]
fn init_cf() -> hot_or_not_web_leptos_ssr::state::cf::CfApi<true> {
    use hot_or_not_web_leptos_ssr::state::cf::{CfApi, CfCredentials};
    let Some(creds) = CfCredentials::from_env("CF_TOKEN", "CF_ACCOUNT_ID") else {
        panic!("Cloudlflare credentials are required: CF_TOKEN, CF_ACCOUNT_ID");
    };
    CfApi::<true>::new(creds)
}

#[cfg(feature = "backend-admin")]
fn init_admin_canisters() -> hot_or_not_web_leptos_ssr::state::admin_canisters::AdminCanisters {
    use hot_or_not_web_leptos_ssr::state::admin_canisters::AdminCanisters;
    use ic_agent::identity::BasicIdentity;
    use std::env;

    let admin_id_pem =
        env::var("BACKEND_ADMIN_IDENTITY").expect("`BACKEND_ADMIN_IDENTITY` is required!");
    let admin_id_pem_by = admin_id_pem.as_bytes();
    let admin_id =
        BasicIdentity::from_pem(admin_id_pem_by).expect("Invalid `BACKEND_ADMIN_IDENTITY`");
    AdminCanisters::new(admin_id)
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

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
