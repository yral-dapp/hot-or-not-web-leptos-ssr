use candid::Principal;
use leptos::{either::EitherOf3, prelude::*};
use leptos_router::{components::Redirect, hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

use super::{overlay::VideoDetailsOverlay, video_loader::VideoView};
use crate::{
    component::{
        back_btn::go_back_or_fallback, scrolling_post_view::MuteIconOverlay,
        spinner::FullScreenSpinner,
    },
    state::{
        audio_state::AudioState,
        canisters::{auth_canisters_store, unauth_canisters},
    },
    utils::{bg_url, send_wrap},
};
use yral_canisters_common::utils::posts::PostDetails;

#[derive(Params, PartialEq, Clone, Copy)]
struct PostParams {
    canister_id: Principal,
    post_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PostFetchError {
    Invalid,
    Unavailable,
    GetUid(String),
}

#[component]
fn SinglePostViewInner(post: PostDetails) -> impl IntoView {
    let AudioState {
        muted,
        show_mute_icon,
        ..
    } = expect_context();
    let bg_url = bg_url(&post.uid);

    view! {
        <div class="w-dvw h-dvh">
            <div class="bg-transparent w-full h-full relative overflow-hidden">
                <div
                    class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                    style:background-color="rgb(0, 0, 0)"
                    style:background-image=format!("url({bg_url})")
                ></div>
                <VideoDetailsOverlay post=post.clone() />
                <VideoView post=Some(post) muted autoplay_at_render=true />
            </div>
            <MuteIconOverlay show_mute_icon />
        </div>
    }
}

#[component]
fn UnavailablePost() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-2 justify-center h-dvh w-dvw bg-black">
            <span class="text-white text-lg md:text-xl lg:text-2xl">Post is unavailable</span>
            <button
                on:click=|_| go_back_or_fallback("/")
                class="px-4 py-2 bg-primary-600 text-center text-white rounded-full"
            >
                Go back
            </button>
        </div>
    }
}

#[component]
pub fn SinglePost() -> impl IntoView {
    let params = use_params::<PostParams>();
    let auth_cans = auth_canisters_store();
    let fetch_post = Resource::new(params, move |params| {
        send_wrap(async move {
            let params = params.map_err(|_| PostFetchError::Invalid)?;
            let post_uid = if let Some(canisters) = auth_cans.get_untracked() {
                canisters
                    .get_post_details(params.canister_id, params.post_id)
                    .await
            } else {
                let canisters = unauth_canisters();
                canisters
                    .get_post_details(params.canister_id, params.post_id)
                    .await
            };
            post_uid
                .map_err(|e| PostFetchError::GetUid(e.to_string()))
                .and_then(|post| post.ok_or(PostFetchError::Unavailable))
        })
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                fetch_post.get()
                    .map(|post| match post {
                        Ok(post) => EitherOf3::A(view! { <SinglePostViewInner post /> }),
                        Err(PostFetchError::Invalid) => EitherOf3::B(view! { <Redirect path="/" /> }),
                        Err(PostFetchError::Unavailable) => EitherOf3::C(view! { <UnavailablePost /> }),
                        Err(PostFetchError::GetUid(e)) => {
                            EitherOf3::B(view! { <Redirect path=format!("/error?err={e}") /> })
                        }
                    })
            }}

        </Suspense>
    }
}
