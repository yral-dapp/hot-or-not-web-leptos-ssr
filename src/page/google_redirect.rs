use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{auth::DelegatedIdentityWire, utils::route::go_to_root};

pub type GoogleAuthMessage = Result<DelegatedIdentityWire, String>;

#[server]
async fn google_auth_redirector() -> Result<(), ServerFnError> {
    use crate::auth::server_impl::google::google_auth_url_impl;
    let url = google_auth_url_impl().await?;
    leptos_axum::redirect(&url);
    Ok(())
}

#[server]
async fn perform_google_auth(oauth: OAuthQuery) -> Result<DelegatedIdentityWire, ServerFnError> {
    use crate::auth::server_impl::google::perform_google_auth_impl;
    perform_google_auth_impl(oauth.state, oauth.code).await
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

async fn handle_oauth_query(query: Result<OAuthQuery, ParamsError>) -> GoogleAuthMessage {
    let Ok(oauth_query) = query else {
        go_to_root();
        return Err("Invalid query".to_string());
    };
    let delegated = perform_google_auth(oauth_query)
        .await
        .map_err(|e| e.to_string())?;
    Ok(delegated)
}

#[component]
pub fn GoogleRedirectHandler() -> impl IntoView {
    let query = use_query::<OAuthQuery>();
    let identity_resource = create_blocking_resource(query, |query_res| async move {
        handle_oauth_query(query_res).await
    });

    view! {
        <Suspense>
            {move || {
                identity_resource().map(|identity_res| view! { <IdentitySender identity_res/> })
            }}

        </Suspense>
        <div class="h-dvh w-dvw bg-black flex flex-col justify-center items-center gap-10">
            <img class="h-56 w-56 object-contain animate-pulse" src="/img/logo.webp"/>
            <span class="text-2xl text-white/60">Good things come to those who wait...</span>
        </div>
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
