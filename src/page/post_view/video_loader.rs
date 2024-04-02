use crate::{
    canister::utils::{bg_url, mp4_url},
    component::feed_popup::FeedPopUp,
    state::{
        auth::account_connected_reader, canisters::AuthProfileCanisterResource,
        local_storage::use_referrer_store,
    },
    utils::event_streaming::send_event_warehouse,
};
use leptos::{ev::beforeunload, html::Video, *};
use leptos_use::use_event_listener;

use super::{overlay::VideoDetailsOverlay, PostViewCtx};
use crate::utils::event_streaming::send_event;
use serde_json::json;
use wasm_bindgen::JsCast;

#[component]
pub fn BgView(idx: usize, children: Children) -> impl IntoView {
    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();
    let post = move || video_queue.with(|q| q.get(idx).cloned());
    let uid = move || post().as_ref().map(|q| q.uid.clone()).unwrap_or_default();

    let (is_connected, _) = account_connected_reader();
    let (show_login_popup, set_show_login_popup) = create_signal(true);

    let (show_refer_login_popup, set_show_refer_login_popup) = create_signal(true);
    let (referrer_store, _, _) = use_referrer_store();

    create_effect(move |_| {
        if current_idx.get() == 5 {
            set_show_login_popup.update(|n| *n = false);
        }
        Some(())
    });

    view! {
        <div class="bg-transparent w-full h-full relative overflow-hidden">
            <div
                class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                style:background-color="rgb(0, 0, 0)"
                style:background-image=move || format!("url({})", bg_url(uid()))
            ></div>
            <Show when=move || { idx == 4 && !is_connected.get() && show_login_popup.get() }>
                <FeedPopUp
                    on_click=move |_| set_show_login_popup.set(false)
                    header_text="Your Rewards are
                    Waiting!"
                    body_text="SignUp/Login to save your progress and claim your rewards."
                    login_text="Login"
                />
            </Show>
            <Show when=move || {
                referrer_store.get().is_some() && idx == 0 && !is_connected.get()
                    && show_refer_login_popup.get()
            }>
                <FeedPopUp
                    on_click=move |_| set_show_refer_login_popup.set(false)
                    header_text="Claim Your Referral
                    Rewards Now!"
                    body_text="SignUp from this link to get 500 COYNs as referral rewards."
                    login_text="Sign Up"
                />
            </Show>
            {move || post().map(|post| view! { <VideoDetailsOverlay post/> })}
            {children()}
        </div>
    }
}

#[component]
pub fn VideoView(idx: usize, muted: RwSignal<bool>) -> impl IntoView {
    let container_ref = create_node_ref::<Video>();
    let PostViewCtx {
        video_queue,
        current_idx,
        ..
    } = expect_context();

    let profile_and_canister_details: AuthProfileCanisterResource = expect_context();
    let (is_connected, _) = account_connected_reader();

    let vid_details = create_memo(move |_| with!(|video_queue| video_queue.get(idx).cloned()));

    let publisher_user_id = move || vid_details().as_ref().map(|q| q.poster_principal);
    let user_id = move || {
        profile_and_canister_details()
            .flatten()
            .map(|(q, _)| q.principal)
    };
    let display_name = move || {
        profile_and_canister_details()
            .flatten()
            .map(|(q, _)| q.display_name)
    };
    let canister_id = move || profile_and_canister_details().flatten().map(|(_, q)| q);
    let video_id = move || vid_details().as_ref().map(|q| q.uid.clone());
    let hastag_count = move || vid_details().as_ref().map(|q| q.hastags.len());
    let is_nsfw = move || vid_details().as_ref().map(|q| q.is_nsfw);
    let is_hotornot = move || {
        vid_details()
            .as_ref()
            .map(|q| q.hot_or_not_feed_ranking_score.is_some())
    };
    let view_count = move || vid_details().as_ref().map(|q| q.views);
    let like_count = move || vid_details().as_ref().map(|q| q.likes);

    let uid =
        create_memo(move |_| with!(|video_queue| video_queue.get(idx).map(|q| q.uid.clone())));
    let view_bg_url = move || uid().map(bg_url);
    let view_video_url = move || uid().map(mp4_url);

    // Handles autoplay
    create_effect(move |_| {
        let Some(vid) = container_ref() else {
            return;
        };
        if idx != current_idx() {
            _ = vid.pause();
            return;
        }
        vid.set_autoplay(true);
        _ = vid.play();
    });

    // Handles mute/unmute
    create_effect(move |_| {
        let vid = container_ref()?;
        vid.set_muted(muted());
        Some(())
    });

    create_effect(move |_| {
        let vid = container_ref()?;
        // the attributes in DOM don't seem to be working
        vid.set_muted(muted.get_untracked());
        vid.set_loop(true);
        Some(())
    });

    #[cfg(feature = "hydrate")]
    {
        // video_viewed - analytics
        let (video_watched, set_video_watched) = create_signal(false);

        let _ = use_event_listener(container_ref, ev::timeupdate, move |evt| {
            if video_watched.get() {
                return;
            }
            let target = evt.target().unwrap();
            let video = target.unchecked_into::<web_sys::HtmlVideoElement>();
            // let duration = video.duration() as f64;
            let current_time = video.current_time();

            if current_time >= 3.0 {
                // Video is halfway done, take action here

                send_event(
                    "video_viewed",
                    &json!({
                        "publisher_user_id":publisher_user_id(),
                        "user_id":user_id(),
                        "is_loggedIn": is_connected(),
                        "display_name": display_name(),
                        "canister_id": canister_id(),
                        "video_id": video_id(),
                        "video_category": "NA",
                        "creator_category": "NA",
                        "hashtag_count": hastag_count(),
                        "is_NSFW": is_nsfw(),
                        "is_hotorNot": is_hotornot(),
                        "feed_type": "NA",
                        "view_count": view_count(),
                        "like_count": like_count(),
                        "share_count": 0,
                    }),
                );
                set_video_watched.set(true);
            }
        });

        // video duration watched - warehousing

        let _ = use_event_listener(container_ref, ev::pause, move |evt| {
            let target = evt.target().unwrap();
            let video = target.unchecked_into::<web_sys::HtmlVideoElement>();
            let duration = video.duration() as f64;
            let current_time = video.current_time();
            if current_time < 1.0 {
                return;
            }

            let percentage_watched = (current_time / duration) * 100.0;

            send_event_warehouse(
                "video_duration_watched",
                &json!({
                    "publisher_user_id":publisher_user_id(),
                    "user_id":user_id(),
                    "is_loggedIn": is_connected(),
                    "display_name": display_name(),
                    "canister_id": canister_id(),
                    "video_id": video_id(),
                    "video_category": "NA",
                    "creator_category": "NA",
                    "hashtag_count": hastag_count(),
                    "is_NSFW": is_nsfw(),
                    "is_hotorNot": is_hotornot(),
                    "feed_type": "NA",
                    "view_count": view_count(),
                    "like_count": like_count(),
                    "share_count": 0,
                    "percentage_watched": percentage_watched,
                    "absolute_watched": current_time,
                    "video_duration": duration,
                }),
            );
        });
    }

    view! {
        <Suspense>
            <label class="w-full h-full absolute top-0 left-0 grid grid-cols-1 justify-items-center items-center cursor-pointer z-[3]">
                <input
                    on:change=move |_| muted.update(|m| *m = !*m)
                    type="checkbox"
                    value=""
                    class="sr-only"
                />
                <video
                    _ref=container_ref
                    class="object-contain h-dvh max-h-dvh cursor-pointer"
                    poster=view_bg_url
                    src=view_video_url
                    loop
                    muted
                    playsinline
                    disablepictureinpicture
                    disableremoteplayback
                    preload="auto"
                ></video>
            </label>
        </Suspense>
    }
}
