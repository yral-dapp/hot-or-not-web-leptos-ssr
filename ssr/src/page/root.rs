use candid::Principal;
use leptos::*;
use leptos_router::*;

use crate::{component::spinner::FullScreenSpinner, utils::host::show_cdao_page};

#[server]
async fn get_top_post_id() -> Result<Option<(Principal, u64)>, ServerFnError> {
    use crate::state::canisters::unauth_canisters;
    use yral_canisters_client::post_cache;

    let canisters = unauth_canisters();
    let post_cache = canisters.post_cache().await;

    let top_items = match post_cache
        .get_top_posts_aggregated_from_canisters_on_this_network_for_home_feed_cursor(
            0,
            1,
            None,
            None,
            Some(post_cache::NsfwFilter::ExcludeNsfw),
        )
        .await?
    {
        post_cache::Result_::Ok(items) => items,
        post_cache::Result_::Err(_) => {
            return Err(ServerFnError::ServerError(
                "failed to fetch top post".to_string(),
            ));
        }
    };
    let Some(top_item) = top_items.first() else {
        return Ok(None);
    };

    Ok(Some((top_item.publisher_canister_id, top_item.post_id)))
}

#[server]
async fn get_top_post_id_mlcache() -> Result<Option<(Principal, u64)>, ServerFnError> {
    use crate::auth::server_impl::extract_principal_from_cookie;
    use crate::state::canisters::unauth_canisters;
    use axum_extra::extract::{cookie::Key, SignedCookieJar};
    use leptos_axum::extract_with_state;

    let key: Key = expect_context();
    let jar: SignedCookieJar = extract_with_state(&key).await?;
    let principal = extract_principal_from_cookie(&jar)?;
    if principal.is_none() {
        return get_top_post_id_mlfeed().await;
    }

    let canisters = unauth_canisters();
    let user_canister_id = canisters
        .get_individual_canister_by_user_principal(principal.unwrap())
        .await?;
    if user_canister_id.is_none() {
        return get_top_post_id_mlfeed().await;
    }

    let user_canister = canisters.individual_user(user_canister_id.unwrap()).await;

    let top_items = user_canister
        .get_ml_feed_cache_paginated(0, 1)
        .await
        .unwrap();
    if top_items.is_empty() {
        return get_top_post_id_mlfeed().await;
    }

    let Some(top_item) = top_items.first() else {
        return Ok(None);
    };

    Ok(Some((top_item.canister_id, top_item.post_id)))
}

#[server]
async fn get_top_post_id_mlfeed() -> Result<Option<(Principal, u64)>, ServerFnError> {
    use crate::utils::ml_feed::ml_feed_grpc::get_coldstart_feed;

    let top_posts_fut = get_coldstart_feed();

    let top_items = match top_posts_fut.await {
        Ok(top_posts) => top_posts,
        Err(e) => {
            log::error!("failed to fetch top post ml feed: {:?}", e);
            return Err(ServerFnError::ServerError(
                "failed to fetch top post ml feed".to_string(),
            ));
        }
    };
    let Some(top_item) = top_items.first() else {
        return Ok(None);
    };

    Ok(Some((top_item.0, top_item.1)))
}

#[component]
pub fn CreatorDaoRootPage() -> impl IntoView {
    view! {
        {move || {
            let redirect_url = "/board".to_string();
            view! { <Redirect path=redirect_url /> }
        }}
    }
}

#[component]
pub fn YralRootPage() -> impl IntoView {
    let target_post;
    #[cfg(any(feature = "local-bin", feature = "local-lib"))]
    {
        target_post = create_resource(|| (), |_| get_top_post_id());
    }
    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    {
        target_post = create_resource(|| (), |_| get_top_post_id_mlcache());
    }

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                target_post
                    .get()
                    .map(|u| {
                        let url = match u {
                            Ok(Some((canister, post_id))) => {
                                format!("/hot-or-not/{canister}/{post_id}")
                            }
                            Ok(None) => "/error?err=No Posts Found".to_string(),
                            Err(e) => format!("/error?err={e}"),
                        };
                        view! { <Redirect path=url /> }
                    })
            }}

        </Suspense>
    }
}

#[component]
pub fn RootPage() -> impl IntoView {
    if show_cdao_page() {
        view! { <CreatorDaoRootPage /> }
    } else {
        view! { <YralRootPage /> }
    }
}
