mod bet;
pub mod error;
pub mod overlay;
pub mod single_post;
pub mod video_iter;
pub mod video_loader;
use priority_queue::DoublePriorityQueue;
use std::cmp::Reverse;

use crate::{
    component::{scrolling_post_view::ScrollingPostView, spinner::FullScreenSpinner},
    consts::NSFW_TOGGLE_STORE,
    state::canisters::{authenticated_canisters, unauth_canisters, Canisters},
    try_or_redirect,
    utils::{
        posts::{get_post_uid, FetchCursor, PostDetails},
        route::failure_redirect,
    },
};
use candid::Principal;
use codee::string::FromToStringCodec;
use futures::StreamExt;
use leptos::*;
use leptos_router::*;
use leptos_use::{storage::use_local_storage, use_debounce_fn};

use video_iter::{FeedResultType, VideoFetchStream};

#[derive(Params, PartialEq, Clone, Copy)]
struct PostParams {
    canister_id: Principal,
    post_id: u64,
}

#[derive(Clone, Default)]
pub struct BetEligiblePostCtx {
    // This is true if betting is enabled for the current post and no bet has been placed
    pub can_place_bet: RwSignal<bool>,
}

#[derive(Clone, Default)]
pub struct PostViewCtx {
    fetch_cursor: RwSignal<FetchCursor>,
    // TODO: this is a dead simple with no GC
    // We're using virtual lists for DOM, so this doesn't consume much memory
    // as uids only occupy 32 bytes each
    // but ideally this should be cleaned up
    video_queue: RwSignal<Vec<PostDetails>>,
    current_idx: RwSignal<usize>,
    queue_end: RwSignal<bool>,
    priority_q: RwSignal<DoublePriorityQueue<PostDetails, (usize, Reverse<usize>)>>, // we are using DoublePriorityQueue for GC in the future through pop_min
    batch_cnt: RwSignal<usize>,
}

#[component]
pub fn CommonPostViewWithUpdates(
    initial_post: Option<PostDetails>,
    fetch_video_action: Action<(), ()>,
    threshold_trigger_fetch: usize,
) -> impl IntoView {
    let PostViewCtx {
        fetch_cursor,
        video_queue,
        current_idx,
        queue_end,
        ..
    } = expect_context();

    let recovering_state = create_rw_signal(false);
    if let Some(initial_post) = initial_post.clone() {
        fetch_cursor.update_untracked(|f| {
            // we've already fetched the first posts
            if f.start > 1 || queue_end.get_untracked() {
                recovering_state.set(true);
                return;
            }
            f.start = 1;
            f.limit = 1;
        });
        video_queue.update_untracked(|v| {
            if v.len() > 1 {
                // Safe to do a GC here
                let rem = 0..(current_idx.get_untracked().saturating_sub(6));
                current_idx.update(|c| *c -= rem.len());
                v.drain(rem);
                return;
            }
            *v = vec![initial_post];
        })
    }

    create_effect(move |_| {
        if !recovering_state.get_untracked() {
            fetch_video_action.dispatch(());
        }
    });
    let next_videos = use_debounce_fn(
        move || {
            if !fetch_video_action.pending().get_untracked() && !queue_end.get_untracked() {
                fetch_video_action.dispatch(())
            }
        },
        500.0,
    );

    let current_post_base = create_memo(move |_| {
        with!(|video_queue| {
            let cur_idx = current_idx();
            let details = video_queue.get(cur_idx)?;
            Some((details.canister_id, details.post_id))
        })
    });

    create_effect(move |_| {
        let Some((canister_id, post_id)) = current_post_base() else {
            return;
        };
        use_navigate()(
            &format!("/hot-or-not/{canister_id}/{post_id}",),
            Default::default(),
        );
    });

    view! {
        <ScrollingPostView
            video_queue
            current_idx
            recovering_state
            fetch_next_videos=next_videos
            queue_end
            threshold_trigger_fetch
        />
    }
}

#[component]
pub fn PostViewWithUpdates(initial_post: Option<PostDetails>) -> impl IntoView {
    let PostViewCtx {
        fetch_cursor,
        video_queue,
        queue_end,
        ..
    } = expect_context();

    let (nsfw_enabled, _, _) = use_local_storage::<bool, FromToStringCodec>(NSFW_TOGGLE_STORE);
    let auth_canisters: RwSignal<Option<Canisters<true>>> = expect_context();

    let fetch_video_action = create_action(move |_| async move {
        loop {
            let Some(cursor) = fetch_cursor.try_get_untracked() else {
                return;
            };
            let Some(auth_canisters) = auth_canisters.try_get_untracked() else {
                return;
            };
            let Some(nsfw_enabled) = nsfw_enabled.try_get_untracked() else {
                return;
            };
            let unauth_canisters = unauth_canisters();

            let chunks = if let Some(canisters) = auth_canisters.as_ref() {
                let fetch_stream = VideoFetchStream::new(canisters, cursor);
                fetch_stream.fetch_post_uids_chunked(3, nsfw_enabled).await
            } else {
                let fetch_stream = VideoFetchStream::new(&unauth_canisters, cursor);
                fetch_stream.fetch_post_uids_chunked(3, nsfw_enabled).await
            };

            let res = try_or_redirect!(chunks);
            let mut chunks = res.posts_stream;
            let mut cnt = 0;
            while let Some(chunk) = chunks.next().await {
                cnt += chunk.len();
                video_queue.try_update(|q| {
                    for uid in chunk {
                        let uid = try_or_redirect!(uid);
                        q.push(uid);
                    }
                });
            }
            if res.end || cnt >= 8 {
                queue_end.try_set(res.end);
                break;
            }
            fetch_cursor.try_update(|c| c.advance());
        }

        fetch_cursor.try_update(|c| c.advance());
    });

    view! { <CommonPostViewWithUpdates initial_post fetch_video_action threshold_trigger_fetch=10 /> }
}

#[component]
pub fn PostViewWithUpdatesMLFeed(initial_post: Option<PostDetails>) -> impl IntoView {
    let PostViewCtx {
        fetch_cursor,
        video_queue,
        queue_end,
        priority_q,
        batch_cnt,
        ..
    } = expect_context();

    let (nsfw_enabled, _, _) = use_local_storage::<bool, FromToStringCodec>(NSFW_TOGGLE_STORE);

    let auth_cans = authenticated_canisters();

    let fetch_video_action = create_action(move |_| {
        let auth_cans = auth_cans.clone();
        async move {
            while priority_q.with_untracked(|q| q.len()) < 15 {
                let Some(cursor) = fetch_cursor.try_get_untracked() else {
                    return;
                };
                let Some(nsfw_enabled) = nsfw_enabled.try_get_untracked() else {
                    return;
                };
                let Some(batch_cnt_val) = batch_cnt.try_get_untracked() else {
                    return;
                };

                let canisters = auth_cans.wait_untracked().await;
                let cans_true = canisters.unwrap().canisters().unwrap();

                let mut fetch_stream = VideoFetchStream::new(&cans_true, cursor);
                let chunks = fetch_stream
                    .fetch_post_uids_hybrid(3, nsfw_enabled, video_queue.get_untracked())
                    .await;

                let res = try_or_redirect!(chunks);
                let mut chunks = res.posts_stream;
                let cnt = create_rw_signal(0);
                while let Some(chunk) = chunks.next().await {
                    update!(move |video_queue, priority_q, cnt| {
                        for uid in chunk {
                            let post_detail = try_or_redirect!(uid);

                            if video_queue.len() < 10 {
                                video_queue.push(post_detail);
                            } else {
                                priority_q.push(post_detail, (batch_cnt_val, Reverse(*cnt)));
                            }

                            *cnt += 1;
                        }
                    });
                }

                leptos::logging::log!("feed type: {:?}", res.res_type);
                if res.res_type != FeedResultType::MLFeed {
                    fetch_cursor.try_update(|c| {
                        c.set_limit(15);
                        c.advance_and_set_limit(15)
                    });
                }

                if res.end {
                    queue_end.try_set(res.end);
                }

                batch_cnt.update(|x| *x += 1);
            }

            update!(move |video_queue, priority_q| {
                let mut cnt = 0;
                while let Some((next, _)) = priority_q.pop_max() {
                    video_queue.push(next);
                    cnt += 1;
                    if cnt >= 10 {
                        break;
                    }
                }
            });
        }
    });

    view! {
        <CommonPostViewWithUpdates
            initial_post
            fetch_video_action
            threshold_trigger_fetch=15
        />
    }
}

#[component]
pub fn PostView() -> impl IntoView {
    let params = use_params::<PostParams>();
    let initial_canister_and_post = create_rw_signal(params.get_untracked().ok());

    create_isomorphic_effect(move |_| {
        if initial_canister_and_post.with_untracked(|p| p.is_some()) {
            return None;
        }
        let p = params.get().ok()?;
        initial_canister_and_post.set(Some(p));
        Some(())
    });

    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();
    let canisters = unauth_canisters();

    let fetch_first_video_uid = create_resource(initial_canister_and_post, move |params| {
        let canisters = canisters.clone();
        async move {
            let Some(params) = params else {
                return Err(());
            };
            let cached_post = video_queue
                .with_untracked(|q| q.get(current_idx.get_untracked()).cloned())
                .filter(|post| {
                    post.canister_id == params.canister_id && post.post_id == params.post_id
                });
            if let Some(post) = cached_post {
                return Ok(Some(post));
            }

            match get_post_uid(&canisters, params.canister_id, params.post_id).await {
                Ok(post) => Ok(post),
                Err(e) => {
                    failure_redirect(e);
                    Err(())
                }
            }
        }
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
            {{
                move || {
                    fetch_first_video_uid()
                        .and_then(|initial_post| {
                            let initial_post = initial_post.ok()?;
                            Some(view! { <PostViewWithUpdatesMLFeed initial_post /> })
                        })
                }
            }}

        </Suspense>
    }
}
