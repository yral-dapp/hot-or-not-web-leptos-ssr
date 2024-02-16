mod error;
mod video_iter;
mod video_loader;

use std::pin::pin;

use candid::Principal;
use futures::StreamExt;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::{
    use_debounce_fn, use_intersection_observer_with_options, UseIntersectionObserverOptions,
};

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
pub fn ScrollingView<NV: Fn() -> NVR + Clone + 'static, NVR>(
    next_videos: NV,
    recovering_state: RwSignal<bool>,
) -> impl IntoView {
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
    let scroll_root = create_node_ref::<html::Div>();

    view! {
        <div
            _ref=scroll_root
            class="snap-mandatory snap-y overflow-y-scroll h-dvh w-dvw bg-black"
            style:scroll-snap-points-y="repeat(100vh)"
        >
            <For
                each=video_enum
                key=|(_, details)| (details.canister_id, details.post_id)
                children=move |(queue_idx, details)| {
                    let container_ref = create_node_ref::<html::Div>();
                    let next_videos = next_videos.clone();
                    use_intersection_observer_with_options(
                        container_ref,
                        move |entry, _| {
                            let Some(visible) = entry
                                .first()
                                .filter(|entry| entry.is_intersecting()) else {
                                return;
                            };
                            let rect = visible.bounding_client_rect();
                            if rect.y() == rect.height() || queue_idx == current_idx.get_untracked()
                            {
                                return;
                            }
                            if video_queue.with_untracked(|q| q.len()).saturating_sub(queue_idx)
                                <= 10
                            {
                                next_videos();
                            }
                            log::info!("setting idx {}", queue_idx);
                            current_idx.set(queue_idx);
                        },
                        UseIntersectionObserverOptions::default()
                            .thresholds(vec![1.0])
                            .root(Some(scroll_root)),
                    );
                    create_effect(move |_| {
                        let Some(container) = container_ref() else {
                            return;
                        };
                        if current_idx.get_untracked() == queue_idx
                            && recovering_state.get_untracked()
                        {
                            container.scroll_into_view();
                            recovering_state.set(false);
                        }
                    });
                    view! {
                        // TODO: confirm this in different screens and browsers
                        // this prevents an initial back and forth between the first and second video

                        // fetch new videos

                        <div _ref=container_ref class="snap-always snap-end w-full h-full">
                            <BgView uid=details.uid>
                                <VideoView idx=queue_idx muted/>
                            </BgView>
                        </div>
                    }
                }
            />

            <Show when=muted>
                <div
                    class="fixed top-1/2 left-1/2 z-20 cursor-pointer"
                    on:click=move |_| muted.set(false)
                >
                    <Icon
                        class="text-white/80 animate-ping text-4xl"
                        icon=icondata::BiVolumeMuteSolid
                    />
                </div>
            </Show>
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

    let recovering_state = create_rw_signal(false);
    if let Some(initial_post) = initial_post {
        fetch_cursor.update_untracked(|f| {
            // we've already fetched the first posts
            if f.start > 1 {
                recovering_state.set(true);
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

    let fetch_video_action = create_action(move |()| async move {
        loop {
            let canisters = unauth_canisters();
            let fetch_stream = VideoFetchStream::new(&canisters, fetch_cursor.get_untracked());
            let chunks = try_or_redirect!(fetch_stream.fetch_post_uids_chunked(3).await);
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
            } else {
                break;
            }
        }

        fetch_cursor.update(|c| c.advance());
    });
    if !recovering_state.get_untracked() {
        fetch_video_action.dispatch(());
    }
    let next_videos = use_debounce_fn(
        move || {
            if !fetch_video_action.pending().get_untracked() {
                log::debug!("trigger rerender");
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

    view! { <ScrollingView next_videos recovering_state/> }
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
