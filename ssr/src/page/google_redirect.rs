use leptos::*;
use leptos_router::*;
use openidconnect::CsrfToken;
use serde::{Deserialize, Serialize};
use server_fn::codec::{GetUrl, Json};

use crate::{
    component::loading::Loading,
    utils::{host::get_host, route::go_to_root},
};
use yral_types::delegated_identity::DelegatedIdentityWire;

pub type GoogleAuthMessage = Result<DelegatedIdentityWire, String>;

#[server]
async fn google_auth_redirector() -> Result<(), ServerFnError> {
    use crate::auth::core_clients::CoreClients;
    use crate::auth::server_impl::google::google_auth_url_impl;
    use http::header::HeaderMap;
    use leptos_axum::extract;

    let headers: HeaderMap = extract().await?;
    let host = headers.get("Host").unwrap().to_str().unwrap();

    let oauth_clients: CoreClients = expect_context();
    let oauth2 = oauth_clients.get_oauth_client(host);

    let url = google_auth_url_impl(oauth2, None).await?;
    leptos_axum::redirect(&url);
    Ok(())
}

async fn get_google_auth_url(host: String) -> Result<String, ServerFnError> {
    let client_redirect_uri = format!("https://{}/auth/google_redirect", host);
    let url = format!(
        "https://yral.com/api/google_auth_url?client_redirect_uri={}",
        client_redirect_uri
    );

    let client = reqwest::Client::new();

    let mut request = client.get(url);

    request = {
        #[cfg(target_arch = "wasm32")]
        {
            request.fetch_credentials_include()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            request
        }
    };

    let redirect_url: String = request.send().await?.json().await?;

    Ok(redirect_url)
}

#[server]
async fn preview_google_auth_redirector(redirect_url: String) -> Result<(), ServerFnError> {
    leptos_axum::redirect(&redirect_url);
    Ok(())
}

#[cfg(feature = "ssr")]
fn is_valid_redirect_uri_inner(client_redirect_uri: &str) -> Option<()> {
    use crate::utils::host::is_host_a_preview_link;

    let parsed_uri = http::Uri::try_from(client_redirect_uri).ok()?;

    if parsed_uri.scheme_str() == Some("yralmobile://") {
        return Some(());
    }

    let host = parsed_uri.host()?;
    if host == "yral.com" {
        return Some(());
    }

    is_host_a_preview_link(host).then_some(())
}

#[cfg(feature = "ssr")]
fn is_valid_redirect_uri(client_redirect_uri: &str) -> bool {
    is_valid_redirect_uri_inner(client_redirect_uri).is_some()
}

#[server(endpoint = "google_auth_url", input = GetUrl, output = Json)]
async fn google_auth_url(client_redirect_uri: String) -> Result<String, ServerFnError> {
    use crate::auth::core_clients::CoreClients;
    use crate::auth::server_impl::google::google_auth_url_impl;
    use http::header::HeaderMap;
    use leptos_axum::extract;

    let headers: HeaderMap = extract().await?;

    if !is_valid_redirect_uri(&client_redirect_uri) {
        return Err(ServerFnError::new("Invalid client redirect uri"));
    }

    let host = headers.get("Host").unwrap().to_str().unwrap();
    let oauth_clients: CoreClients = expect_context();
    let oauth2 = oauth_clients.get_oauth_client(host);
    let url = google_auth_url_impl(oauth2, Some(client_redirect_uri)).await?;

    Ok(url)
}

#[server(endpoint = "perform_google_auth", input = Json, output = Json)]
async fn perform_google_auth(oauth: OAuthQuery) -> Result<DelegatedIdentityWire, ServerFnError> {
    use crate::auth::core_clients::CoreClients;
    use crate::auth::server_impl::google::perform_google_auth_impl;
    use http::header::HeaderMap;
    use leptos_axum::extract;

    let headers: HeaderMap = extract().await?;
    let host = headers.get("Host").unwrap().to_str().unwrap();

    let oauth_clients: CoreClients = expect_context();
    let oauth2 = oauth_clients.get_oauth_client(host);

    perform_google_auth_impl(oauth.state, oauth.code, oauth2).await
}

#[derive(Params, Debug, PartialEq, Clone, Serialize, Deserialize)]
struct OAuthQuery {
    pub code: String,
    pub state: String,
}

#[component]
pub fn IdentitySender(identity_res: GoogleAuthMessage) -> impl IntoView {
    create_effect(move |_| {
        let _id = &identity_res;
        #[cfg(feature = "hydrate")]
        {
            use web_sys::Window;

            let win = window();
            let origin = win.origin();
            let opener = win.opener().unwrap();
            if opener.is_null() {
                go_to_root();
            }
            let opener = Window::from(opener);
            let msg = serde_json::to_string(&_id).unwrap();
            _ = opener.post_message(&msg.into(), &origin);
        }
    });

    view! {
        <div class="h-dvh w-dvw bg-black flex flex-col justify-center items-center gap-10">
            <img class="h-56 w-56 object-contain animate-pulse" src="/img/logo.webp"/>
            <span class="text-2xl text-white/60">Good things come to those who wait...</span>
        </div>
    }
}

async fn handle_oauth_query(oauth_query: OAuthQuery) -> GoogleAuthMessage {
    let delegated = perform_google_auth(oauth_query)
        .await
        .map_err(|e| e.to_string())?;
    Ok(delegated)
}

#[derive(Serialize, Deserialize)]
struct GoogleAuthRequestBody {
    oauth: OAuthQuery,
}

async fn preview_handle_oauth_query(
    oauth_query: OAuthQuery,
) -> Result<DelegatedIdentityWire, ServerFnError> {
    let client: reqwest::Client = reqwest::Client::new();

    let yral_url = "https://yral.com/api/perform_google_auth".to_string();

    let oauth_request_body = GoogleAuthRequestBody { oauth: oauth_query };

    let mut request = client.post(yral_url).json(&oauth_request_body);

    request = {
        #[cfg(target_arch = "wasm32")]
        {
            request.fetch_credentials_include()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            request
        }
    };

    let response = request.send().await?;

    if response.status().is_success() {
        let identity_wire: DelegatedIdentityWire = response.json().await?;
        Ok(identity_wire)
    } else {
        let err: ServerFnError = response.json().await?;
        Err(err)
    }
}

#[server]
async fn handle_oauth_query_for_external_client(
    client_redirect_uri: String,
    oauth_query: OAuthQuery,
) -> Result<(), ServerFnError> {
    leptos_axum::redirect(&format!(
        "{}?code={}&state={}",
        client_redirect_uri, oauth_query.code, oauth_query.state
    ));
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
enum RedirectHandlerReturnType {
    Identity(GoogleAuthMessage),
    ExternalClient(Result<(), String>),
}

#[derive(Serialize, Deserialize)]
struct OAuthState {
    pub csrf_token: CsrfToken,
    pub client_redirect_uri: Option<String>,
}

#[component]
pub fn PreviewGoogleRedirectHandler() -> impl IntoView {
    let query = use_query::<OAuthQuery>();
    let identity_resource = create_local_resource(query, |query_res| async move {
        if let Err(e) = query_res {
            return Err(format!("Invalid Params {}", e));
        }

        let oauth_query = query_res.unwrap();

        preview_handle_oauth_query(oauth_query)
            .await
            .map_err(|e| e.to_string())
    });

    view! {
        <Loading text="Logging out...".to_string()>
            <Suspense>
                {move || {
                    identity_resource().map(|identity_res| view! { <IdentitySender identity_res/> })
                }}

            </Suspense>
        </Loading>
    }
}

#[component]
pub fn GoogleRedirectHandler() -> impl IntoView {
    let query = use_query::<OAuthQuery>();
    let identity_resource = create_blocking_resource(query, |query_res| async move {
        let Ok(oauth_query) = query_res else {
            return RedirectHandlerReturnType::Identity(Err("Invalid query".to_string()));
        };

        let Ok(oauth_state) = serde_json::from_str::<OAuthState>(&oauth_query.state) else {
            return RedirectHandlerReturnType::Identity(Err("Invalid OAuth State".to_string()));
        };

        if oauth_state.client_redirect_uri.is_some() {
            let res = handle_oauth_query_for_external_client(
                oauth_state.client_redirect_uri.unwrap(),
                oauth_query,
            )
            .await
            .map_err(|e| e.to_string());
            RedirectHandlerReturnType::ExternalClient(res)
        } else {
            let res = handle_oauth_query(oauth_query).await;
            RedirectHandlerReturnType::Identity(res)
        }
    });

    view! {
        <Loading text="Logging out...".to_string()>
            <Suspense>
                {move || {
                    identity_resource()
                        .map(|identity_res: RedirectHandlerReturnType| match identity_res {
                            RedirectHandlerReturnType::Identity(identity_res) => {
                                view! { <IdentitySender identity_res/> }.into_view()
                            }
                            RedirectHandlerReturnType::ExternalClient(_) => view! {}.into_view(),
                        })
                }}

            </Suspense>
        </Loading>
    }
}

#[component]
pub fn PreviewGoogleRedirector() -> impl IntoView {
    let host = get_host();
    let google_redirect = create_local_resource(
        || {},
        move |_| {
            let host = host.clone();
            async move {
                let url = get_google_auth_url(host).await?;
                Ok::<String, ServerFnError>(url)
            }
        },
    );

    create_local_resource(google_redirect, |url_res| async {
        let url_res = url_res.transpose()?;
        if let Some(redirect_url) = url_res {
            preview_google_auth_redirector(redirect_url).await?;
            return Ok(());
        }
        Ok::<(), ServerFnError>(())
    });


    let do_close = create_rw_signal(false);
    create_effect(move |_| {
        if !do_close() {
            return;
        }
        let window = window();
        _ = window.close();
    });

    view! {
        <Suspense>
            {move || {
                if let Some(Err(err)) = google_redirect() {
                    log::info!("Error Redirecting {}", err)
                }
                None::<()>
            }}

        </Suspense>
    }
}

#[component]
pub fn GoogleRedirector() -> impl IntoView {
    let google_redirect = create_blocking_resource(|| (), |_| google_auth_redirector());
    let do_close = create_rw_signal(false);
    create_effect(move |_| {
        if !do_close() {
            return;
        }
        let window = window();
        _ = window.close();
    });

    view! {
        <Suspense>
            {move || {
                if let Some(Err(_)) = google_redirect() {
                    do_close.set(true)
                }
                None::<()>
            }}

        </Suspense>
    }
}
