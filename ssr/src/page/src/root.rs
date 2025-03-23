use crate::{post_view::{PostDetailsCacheCtx}, pumpdump::PumpNDump};
use candid::Principal;
use component::spinner::FullScreenSpinner;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::Redirect, hooks::use_query};
use leptos_router::params::Params;
use utils::{
    host::{show_cdao_page, show_pnd_page}, ml_feed::{get_ml_feed_coldstart_clean, get_ml_feed_coldstart_nsfw},
};
use yral_canisters_common::utils::time::current_epoch;
use yral_types::post::PostItem;
use leptos_use::storage::use_local_storage;

#[server]
async fn get_top_post_id() -> Result<Option<PostItem>, ServerFnError> {
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

    Ok(Some(PostItem {
        canister_id: top_item.publisher_canister_id,
        post_id: top_item.post_id,
        video_id: "".to_string(),
        nsfw_probability: 0.0,
    }))
}

#[server]
async fn get_top_post_id_global_clean_feed() -> Result<Option<PostItem>, ServerFnError> {
    let posts = get_ml_feed_coldstart_clean(Principal::anonymous(), 1, vec![]).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    if !posts.is_empty() {
        return Ok(Some(posts[0].clone()));
    }

    Ok(None)
}

#[server]
async fn get_top_post_id_global_nsfw_feed() -> Result<Option<PostItem>, ServerFnError> {
    let posts = get_ml_feed_coldstart_nsfw(Principal::anonymous(), 1, vec![]).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    if !posts.is_empty() {
        return Ok(Some(posts[0].clone()));
    }

    Ok(None)
}

#[component]
pub fn CreatorDaoRootPage() -> impl IntoView {
    view! {
        <Redirect path="/board".to_string() />
    }
}

#[derive(Clone, Params, PartialEq)]
struct NsfwParam {
    nsfw: bool,
}

#[component]
pub fn YralRootPage() -> impl IntoView {

    // check nsfw param
    let params = use_query::<NsfwParam>();
    let nsfw_enabled = params.get_untracked().map(|p| p.nsfw).unwrap_or(false);

    let target_post;
    #[cfg(any(feature = "local-bin", feature = "local-lib"))]
    {
        target_post = Resource::new(|| (), |_| get_top_post_id());
    }
    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    {
        use utils::host::show_nsfw_content;

        if nsfw_enabled || show_nsfw_content() {
            target_post = Resource::new(|| (), |_| get_top_post_id_global_nsfw_feed());
        } else {
            target_post = Resource::new(|| (), |_| get_top_post_id_global_clean_feed());
        }
    }
    let post_details_cache: PostDetailsCacheCtx = expect_context();

    view! {
        <Title text="YRAL - Home" />
        <Suspense fallback=FullScreenSpinner>
            {move || {
                target_post
                    .get()
                    .map(|u| {
                        let url = match u {
                            Ok(Some(post_item)) => {
                                let canister_id = post_item.canister_id;
                                let post_id = post_item.post_id;
                                post_details_cache.post_details.update(|post_details| {
                                    post_details.insert((canister_id, post_id), post_item.clone());
                                });

                                format!("/hot-or-not/{canister_id}/{post_id}")
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
