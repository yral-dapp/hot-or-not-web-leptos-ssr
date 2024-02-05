mod error;
mod video_iter;
mod video_loader;

use std::pin::pin;

use candid::Principal;
use futures::StreamExt;
use leptos::*;
use leptos_router::*;

use crate::{
    component::spinner::FullScreenSpinner,
    state::canisters::unauth_canisters,
    try_or_redirect,
    utils::route::{failure_redirect, go_to_root},
};
use video_iter::{get_post_uid, VideoFetchStream};
use video_loader::{BgView, VideoView};

use self::video_iter::{FetchCursor, PostDetails};

#[derive(Params, PartialEq)]
struct PostParams {
    canister_id: String,
    post_id: u64,
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
}

const PLAYER_CNT: usize = 15;

// Infinite Scrolling View
// Basically a virtual list with 5 items visible at a time
#[component]
pub fn ScrollingView() -> impl IntoView {
    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();

    // let video_ref = create_node_ref::<Video>();
    // // Cache wasp views to avoid re-initialization
    // let _video_view = view! { <HlsVideo video_ref allow_show/> };
    let current_start = move || {
        let cur_idx = current_idx();
        cur_idx.max(PLAYER_CNT / 2) - (PLAYER_CNT / 2)
    };

    let video_enum = create_memo(move |_| {
        with!(|video_queue| {
            let start = current_start();
            video_queue[start..]
                .iter()
                .take(PLAYER_CNT)
                .enumerate()
                .map(|(idx, item)| (idx + start, item.clone()))
                .collect::<Vec<_>>()
        })
    });
    let muted = create_rw_signal(true);

    view! {
        <div
            class="snap-mandatory snap-y overflow-y-scroll h-screen bg-black"
            style:scroll-snap-points-y="repeat(100vh)"
        >
            <For
                each=video_enum
                key=|u| u.1.uid.clone()
                children=move |(queue_idx, details)| {
                    view! {
                        <div class="snap-always snap-end h-full">
                            <BgView uid=details.uid.clone()>
                                <VideoView idx=queue_idx muted/>
                            </BgView>
                        </div>
                    }
                }
            />

        </div>
    }
}

#[component]
pub fn PostViewWithUpdates(initial_post: Option<PostDetails>) -> impl IntoView {
    let PostViewCtx {
        fetch_cursor,
        video_queue,
        current_idx,
    } = expect_context();

    let fetch_fuse = create_rw_signal(true);

    if let Some(initial_post) = initial_post {
        fetch_cursor.update_untracked(|f| {
            // we've already fetched the first posts
            if f.start > 1 {
                // unblow fuse so first fetch is skipped
                fetch_fuse.set(false);
                return;
            }
            f.start = 1;
            f.limit = 1;
        });
        video_queue.update_untracked(|v| {
            if v.len() > 1 {
                return;
            }
            *v = vec![initial_post];
        })
    }

    let _ = create_resource(fetch_cursor, move |cursor| async move {
        // if fuse is not yet blown
        // skip fetching
        if !fetch_fuse.get_untracked() {
            // blow the fuse so next fetch is not skipped
            fetch_fuse.set(true);
            return;
        }

        let canisters = unauth_canisters();
        let fetch_stream = VideoFetchStream::new(&canisters, cursor);
        let chunks = try_or_redirect!(fetch_stream.fetch_post_uids_chunked(2).await);
        let mut chunks = pin!(chunks);
        let mut cnt = 0;
        while let Some(chunk) = chunks.next().await {
            cnt += chunk.len();
            video_queue.update(|q| {
                for uid in chunk {
                    let uid = try_or_redirect!(uid);
                    q.push(uid);
                }
            });
        }
        if cnt < 8 {
            fetch_cursor.update(|c| c.advance());
        }
    });

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

    view! { <ScrollingView/> }
}

#[component]
pub fn PostView() -> impl IntoView {
    let params = use_params::<PostParams>();
    let canister_and_post = move || {
        params.with_untracked(|p| {
            let p = p.as_ref().ok()?;
            let canister_id = Principal::from_text(&p.canister_id).ok()?;

            Some((canister_id, p.post_id))
        })
    };

    let fetch_first_video_uid = create_resource(
        || (),
        move |_| async move {
            let PostViewCtx {
                video_queue,
                current_idx,
                ..
            } = expect_context();
            let Some((canister, post_id)) = canister_and_post() else {
                go_to_root();
                return None;
            };
            if let Some(post) =
                video_queue.with_untracked(|q| q.get(current_idx.get_untracked()).cloned())
            {
                if post.canister_id == canister && post.post_id == post_id {
                    return Some(post);
                }
            }

            let canisters = expect_context();
            match get_post_uid(&canisters, canister, post_id).await {
                Ok(Some(uid)) => Some(uid),
                Err(e) => {
                    failure_redirect(e);
                    None
                }
                Ok(None) => None,
            }
        },
    );

    view! {
        <Suspense fallback=FullScreenSpinner>

            {move || {
                fetch_first_video_uid
                    .get()
                    .map(|post| view! { <PostViewWithUpdates initial_post=post/> })
            }}

        </Suspense>
    }
}
