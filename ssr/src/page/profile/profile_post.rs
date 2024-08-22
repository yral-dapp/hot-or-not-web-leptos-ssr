use candid::Principal;
use leptos::*;
use leptos_router::*;
use leptos_use::use_debounce_fn;

use crate::{
    component::{
        back_btn::BackButton, scrolling_post_view::ScrollingPostView, spinner::FullScreenSpinner,
    },
    page::profile::ProfilePostsContext,
    state::canisters::{auth_canisters_store, unauth_canisters},
    try_or_redirect,
    utils::{
        posts::{get_post_uid, FetchCursor},
        route::failure_redirect,
    },
};

use super::overlay::YourProfileOverlay;
use super::profile_iter::ProfileVideoStream;

use crate::utils::posts::PostDetails;

#[derive(Params, PartialEq)]
struct ProfileVideoParams {
    canister_id: String,
    post_id: u64,
}

#[component]
pub fn ProfilePostWithUpdates(initial_post: PostDetails) -> impl IntoView {
    let ProfilePostsContext {
        video_queue,
        start_index,
        current_index,
        queue_end,
    } = expect_context();
    let recovering_state = create_rw_signal(true);
    let fetch_cursor = create_rw_signal(FetchCursor {
        start: start_index.get_untracked() as u64,
        limit: 10,
    });
    let auth_canister = auth_canisters_store();
    let overlay = match auth_canister.get_untracked() {
        Some(canisters) if canisters.user_canister() == initial_post.canister_id => {
            || view! { <YourProfileOverlay/> }.into_view()
        }
        _ => || view! {}.into_view(),
    };

    if start_index.get_untracked() == 0 {
        video_queue.update_untracked(|vq| {
            vq.push(initial_post.clone());
        });
        queue_end.update(|end| {
            *end = true;
        })
    }

    let next_videos = create_action(move |_| async move {
        let user_canister = initial_post.canister_id;
        let cursor = fetch_cursor.get_untracked();

        let posts_res = if let Some(canisters) = auth_canister.get_untracked() {
            let profile_posts = ProfileVideoStream::new(cursor, &canisters, user_canister);
            profile_posts.fetch_next_profile_posts().await
        } else {
            let unauth_canister = unauth_canisters();
            let profile_posts = ProfileVideoStream::new(cursor, &unauth_canister, user_canister);
            profile_posts.fetch_next_profile_posts().await
        };

        let posts = try_or_redirect!(posts_res);

        posts.into_iter().for_each(|p| {
            video_queue.try_update(|q| {
                q.push(p);
            });
        });
        fetch_cursor.try_update(|c| {
            c.advance();
        });
    });

    let fetch_next_videos = use_debounce_fn(
        move || {
            if !next_videos.pending().get_untracked() && !queue_end.get_untracked() {
                log::debug!("trigger rerender");
                next_videos.dispatch(video_queue)
            }
        },
        500.0,
    );

    let current_post_base = create_memo(move |_| {
        video_queue.with(|q| {
            let details = q.get(current_index());
            details.map(|d| (d.canister_id, d.post_id))
        })
    });

    create_effect(move |_| {
        let Some((canister_id, post_id)) = current_post_base.get() else {
            return;
        };

        if recovering_state.get_untracked() {
            return;
        }

        use_navigate()(
            &format!("profile/{canister_id}/{post_id}"),
            NavigateOptions {
                replace: true,
                ..Default::default()
            },
        );
    });

    view! {
        <ScrollingPostView
            video_queue
            current_idx=current_index
            queue_end
            recovering_state
            fetch_next_videos
            overlay
            threshold_trigger_fetch=10
        />
    }
}

#[component]
pub fn ProfilePost() -> impl IntoView {
    let params = use_params::<ProfileVideoParams>();

    let canister_and_post = move || {
        params.with_untracked(|p| {
            let p = p.as_ref().ok()?;
            let canister_id = Principal::from_text(&p.canister_id).ok()?;

            Some((canister_id, p.post_id))
        })
    };

    let ProfilePostsContext {
        video_queue,
        current_index,
        ..
    } = expect_context();

    let intial_post = create_resource(canister_and_post, move |params| {
        let canisters = unauth_canisters();
        async move {
            let Some((canister_id, post_id)) = params else {
                failure_redirect("Invalid profile post");
                return None;
            };

            let retrieved_post = video_queue.with_untracked(|vq| {
                let post_idx = vq
                    .iter()
                    .position(|post| post.canister_id == canister_id && post.post_id == post_id);
                current_index.update(|idx| *idx = post_idx.unwrap_or(0));
                post_idx.and_then(|p_idx| vq.get(p_idx)).cloned()
            });

            if let Some(post) = retrieved_post {
                return Some(post);
            };

            match get_post_uid(&canisters, canister_id, post_id).await {
                Ok(res) => res,
                Err(e) => {
                    failure_redirect(e);
                    None
                }
            }
        }
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                intial_post
                    .get()
                    .flatten()
                    .map(|pd| {
                        Some(
                            view! {
                                <div class="absolute left-4 top-4 bg-transparent z-10 text-white">
                                    <BackButton fallback="/".to_string()/>
                                </div>
                                <ProfilePostWithUpdates initial_post=pd/>
                            },
                        )
                    })
            }}

        </Suspense>
    }
}
