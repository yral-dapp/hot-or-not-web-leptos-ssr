use leptos::{create_signal, ev, expect_context, html::Video, Memo, NodeRef, SignalGet, SignalSet};
use leptos_use::use_event_listener;
use serde_json::json;
use wasm_bindgen::JsCast;

use crate::{
    page::post_view::video_iter::PostDetails,
    state::{auth::account_connected_reader, canisters::AuthProfileCanisterResource},
};

pub enum AnalyticsEvent {
    VideoWatched(VideoWatched),
}

#[derive(Default)]
pub struct VideoWatched;

impl VideoWatched {
    pub fn send_event(
        &self,
        vid_details: Memo<Option<PostDetails>>,
        container_ref: NodeRef<Video>,
    ) {
        use crate::utils::event_streaming::{send_event, send_event_warehouse};

        let profile_and_canister_details: AuthProfileCanisterResource = expect_context();
        let (is_connected, _) = account_connected_reader();

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

        // video_viewed - analytics
        let (video_watched, set_video_watched) = create_signal(false);
        let (full_video_watched, set_full_video_watched) = create_signal(false);

        let _ = use_event_listener(container_ref, ev::timeupdate, move |evt| {
            let target = evt.target().unwrap();
            let video = target.unchecked_into::<web_sys::HtmlVideoElement>();
            let duration = video.duration();
            let current_time = video.current_time();

            if current_time < 0.95 * duration {
                set_full_video_watched.set(false);
            }

            // send bigquery event when video is watched > 95%
            if current_time >= 0.95 * duration && !full_video_watched.get() {
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
                        "percentage_watched": 100.0,
                        "absolute_watched": duration,
                        "video_duration": duration,
                    }),
                );

                set_full_video_watched.set(true);
            }

            if video_watched.get() {
                return;
            }

            if current_time >= 3.0 {
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
            let duration = video.duration();
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
}
