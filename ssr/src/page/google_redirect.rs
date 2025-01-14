use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::{GetUrl, Json};

use crate::{component::loading::Loading, utils::route::go_to_root};
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

#[server(endpoint = "google_auth_url", input = GetUrl, output = Json)]
async fn google_auth_url(client_redirect_uri: String) -> Result<String, ServerFnError> {
    use crate::auth::core_clients::CoreClients;
    use crate::auth::server_impl::google::google_auth_url_impl;
    use http::header::HeaderMap;
    use leptos_axum::extract;

    let headers: HeaderMap = extract().await?;

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
    pub client_redirect_uri: Option<String>,
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

async fn handle_oauth_query_for_external_client(
    client_redirect_uri: String,
    auth_code: String,
) -> Result<(), String> {
    use_navigate()(
        &format!("{}?authCode={}", client_redirect_uri, auth_code),
        NavigateOptions {
            resolve: false,
            ..Default::default()
        }
    );
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
enum RedirectHandlerReturnType {
    Identity(GoogleAuthMessage),
    ExternalClient(Result<(), String>),
}

#[component]
pub fn GoogleRedirectHandler() -> impl IntoView {
    let query = use_query::<OAuthQuery>();
    let identity_resource = create_blocking_resource(query, |query_res| async move {
        let Ok(oauth_query) = query_res else {
            return RedirectHandlerReturnType::Identity(Err("Invalid query".to_string()));
        };

        if oauth_query.client_redirect_uri.is_some() {
            let res = handle_oauth_query_for_external_client(
                oauth_query.client_redirect_uri.unwrap(),
                oauth_query.code,
            )
            .await;
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
                    identity_resource().map(|identity_res: RedirectHandlerReturnType| match identity_res {
                        RedirectHandlerReturnType::Identity(identity_res) => view! {<IdentitySender identity_res/> }.into_view(),
                        RedirectHandlerReturnType::ExternalClient(_) => view! {}.into_view()
                    })
                }}

            </Suspense>
        </Loading>
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
                    do_close.set(true);
                }
                None::<()>
            }}

        </Suspense>
    }
}
