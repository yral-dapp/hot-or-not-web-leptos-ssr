use crate::post_view::PostDetailsCacheCtx;
use crate::pumpdump::PumpNDump;
use candid::Principal;
use component::spinner::FullScreenSpinner;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::params::Params;
use leptos_router::{components::Redirect, hooks::use_query};
use utils::host::show_nsfw_content;
use utils::{
    host::{show_cdao_page, show_pnd_page},
    ml_feed::{get_ml_feed_coldstart_clean, get_ml_feed_coldstart_nsfw},
};
use yral_types::post::PostItem;

#[server]
async fn get_top_post_id_global_clean_feed() -> Result<Option<PostItem>, ServerFnError> {
    let posts = get_ml_feed_coldstart_clean(Principal::anonymous(), 1, vec![])
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    if !posts.is_empty() {
        return Ok(Some(posts[0].clone()));
    }

    Ok(None)
}

#[server]
async fn get_top_post_id_global_nsfw_feed() -> Result<Option<PostItem>, ServerFnError> {
    let posts = get_ml_feed_coldstart_nsfw(Principal::anonymous(), 1, vec![])
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
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

    let target_post = if nsfw_enabled || show_nsfw_content() {
        Resource::new(|| (), |_| get_top_post_id_global_nsfw_feed())
    } else {
        Resource::new(|| (), |_| get_top_post_id_global_clean_feed())
    };
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
