use crate::pumpdump::PumpNDump;
use candid::Principal;
use component::spinner::FullScreenSpinner;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::Redirect;
use utils::{
    host::{show_cdao_page, show_pnd_page},
    ml_feed::{
        get_coldstart_feed_paginated, get_coldstart_nsfw_feed_paginated,
        get_posts_ml_feed_cache_paginated,
    },
};
use yral_canisters_common::utils::time::current_epoch;

#[server]
async fn get_top_post_id() -> Result<Option<(Principal, u64)>, ServerFnError> {
    use state::canisters::unauth_canisters;
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
    use auth::server_impl::extract_principal_from_cookie;
    use axum_extra::extract::{cookie::Key, SignedCookieJar};
    use leptos_axum::extract_with_state;
    use state::canisters::unauth_canisters;

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

    let posts = get_posts_ml_feed_cache_paginated(user_canister_id.unwrap(), 0, 1).await;
    if let Ok(posts) = posts {
        if !posts.is_empty() {
            return Ok(Some((posts[0].0, posts[0].1)));
        }
    }
    get_top_post_id_mlfeed().await
}

#[server]
async fn get_top_post_id_mlfeed() -> Result<Option<(Principal, u64)>, ServerFnError> {
    use rand::{rngs::SmallRng, Rng, SeedableRng};
    let top_posts_fut = get_coldstart_feed_paginated(0, 50);

    let top_items = match top_posts_fut.await {
        Ok(top_posts) => top_posts,
        Err(e) => {
            log::error!("failed to fetch top post ml feed: {:?}", e);
            return Err(ServerFnError::ServerError(
                "failed to fetch top post ml feed".to_string(),
            ));
        }
    };
    let mut rand_gen = SmallRng::seed_from_u64(current_epoch().as_nanos() as u64);
    let rand_num = rand_gen.random_range(0..top_items.len());
    let top_item = top_items[rand_num];

    Ok(Some((top_item.0, top_item.1)))
}

#[server]
async fn get_top_post_id_mlfeed_nsfw() -> Result<Option<(Principal, u64)>, ServerFnError> {
    use rand::{rngs::SmallRng, Rng, SeedableRng};
    let top_posts_fut = get_coldstart_nsfw_feed_paginated(0, 50);

    let top_items = match top_posts_fut.await {
        Ok(top_posts) => top_posts,
        Err(e) => {
            log::error!("failed to fetch top post ml feed nsfw: {:?}", e);
            return Err(ServerFnError::ServerError(
                "failed to fetch top post ml feed nsfw".to_string(),
            ));
        }
    };
    let mut rand_gen = SmallRng::seed_from_u64(current_epoch().as_nanos() as u64);
    let rand_num = rand_gen.random_range(0..top_items.len());
    let top_item = top_items[rand_num];

    Ok(Some((top_item.0, top_item.1)))
}

#[component]
pub fn CreatorDaoRootPage() -> impl IntoView {
    view! {
        <Redirect path="/board".to_string() />
    }
}

#[component]
pub fn YralRootPage() -> impl IntoView {
    let target_post;
    #[cfg(any(feature = "local-bin", feature = "local-lib"))]
    {
        target_post = Resource::new(|| (), |_| get_top_post_id());
    }
    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    {
        use utils::host::show_nsfw_content;

        if show_nsfw_content() {
            target_post = Resource::new(|| (), |_| get_top_post_id_mlfeed_nsfw());
        } else {
            target_post = Resource::new(|| (), |_| get_top_post_id_mlfeed());
        }
    }

    view! {
        <Title text="YRAL - Home" />
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
    if show_pnd_page() {
        view! { <PumpNDump /> }.into_any()
    } else if show_cdao_page() {
        view! { <CreatorDaoRootPage /> }.into_any()
    } else {
        view! { <YralRootPage /> }.into_any()
    }
}
