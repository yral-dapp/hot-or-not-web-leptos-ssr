use candid::Principal;
use ic_agent::Identity;
use leptos::html::Input;
use leptos::{create_effect, MaybeSignal, ReadSignal, RwSignal, SignalGetUntracked};
use leptos::{create_signal, ev, expect_context, html::Video, NodeRef, SignalGet, SignalSet};
use leptos_use::use_event_listener;
use serde_json::json;
use sns_validation::pbs::sns_pb::SnsInitPayload;
use wasm_bindgen::JsCast;

use super::EventHistory;
use crate::component::auth_providers::ProviderKind;
use crate::state::auth::account_connected_reader;
use crate::state::canisters::{auth_canisters_store, Canisters};
use crate::state::history::HistoryCtx;
#[cfg(feature = "ga4")]
use crate::utils::event_streaming::{send_event, send_event_warehouse, send_user_id};
use crate::utils::posts::PostDetails;
use crate::utils::user::{user_details_can_store_or_ret, user_details_or_ret};

pub enum AnalyticsEvent {
    VideoWatched(VideoWatched),
    LikeVideo(LikeVideo),
    ShareVideo(ShareVideo),
    VideoUploadInitiated(VideoUploadInitiated),
    VideoUploadUploadButtonClicked(VideoUploadUploadButtonClicked),
    VideoUploadVideoSelected(VideoUploadVideoSelected),
    VideoUploadUnsuccessful(VideoUploadUnsuccessful),
    VideoUploadSuccessful(VideoUploadSuccessful),
    Refer(Refer),
    ReferShareLink(ReferShareLink),
    LoginSuccessful(LoginSuccessful),
    LoginMethodSelected(LoginMethodSelected),
    LoginJoinOverlayViewed(LoginJoinOverlayViewed),
    LoginCta(LoginCta),
    LogoutClicked(LogoutClicked),
    LogoutConfirmation(LogoutConfirmation),
    ErrorEvent(ErrorEvent),
    ProfileViewVideo(ProfileViewVideo),
    TokenCreationStarted(TokenCreationStarted),
    TokenCreationCompleted(TokenCreationCompleted),
    TokenCreationFailed(TokenCreationFailed),
    TokensClaimedFromNeuron(TokensClaimedFromNeuron),
    TokensTransferred(TokensTransferred),
}

#[derive(Default)]
pub struct VideoWatched;

impl VideoWatched {
    pub fn send_event(
        &self,
        vid_details: MaybeSignal<Option<PostDetails>>,
        container_ref: NodeRef<Video>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let (is_connected, _) = account_connected_reader();

            // video_viewed - analytics
            let (video_watched, set_video_watched) = create_signal(false);
            let (full_video_watched, set_full_video_watched) = create_signal(false);

            let cans_store: RwSignal<Option<Canisters<true>>> = auth_canisters_store();

            let post_for_time = vid_details.clone();
            let _ = use_event_listener(container_ref, ev::timeupdate, move |evt| {
                let user = user_details_can_store_or_ret!(cans_store);
                let post_o = post_for_time();
                let post = post_o.as_ref();

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
                            "publisher_user_id": post.map(|p| p.poster_principal),
                            "user_id": user.details.principal,
                            "is_loggedIn": is_connected(),
                            "display_name": user.details.display_name.clone(),
                            "canister_id": user.canister_id,
                            "video_id": post.map(|p| p.uid.clone()),
                            "video_category": "NA",
                            "creator_category": "NApublisher_canister_id(",
                            "hashtag_count": post.map(|p| p.hastags.len()),
                            "is_NSFW": post.map(|p| p.is_nsfw),
                            "is_hotorNot": post.map(|p| p.is_hot_or_not()),
                            "feed_type": "NA",
                            "view_count": post.map(|p| p.views),
                            "like_count": post.map(|p| p.likes),
                            "share_count": 0,
                            "percentage_watched": 100.0,
                            "absolute_watched": duration,
                            "video_duration": duration,
                            "post_id": post.map(|p| p.post_id),
                            "publisher_canister_id": post.map(|p| p.canister_id),
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
                            "publisher_user_id": post.map(|p| p.poster_principal),
                            "user_id": user.details.principal,
                            "is_loggedIn": is_connected(),
                            "display_name": user.details.display_name,
                            "canister_id": user.canister_id,
                            "video_id": post.map(|p| p.uid.clone()),
                            "video_category": "NA",
                            "creator_category": "NA",
                            "hashtag_count": post.map(|p| p.hastags.len()),
                            "is_NSFW": post.map(|p| p.is_nsfw),
                            "is_hotorNot": post.map(|p| p.is_hot_or_not()),
                            "feed_type": "NA",
                            "view_count": post.map(|p| p.views),
                            "like_count": post.map(|p| p.likes),
                            "share_count": 0,
                            "post_id": post.map(|p| p.post_id),
                            "publisher_canister_id": post.map(|p| p.canister_id),
                        }),
                    );
                    set_video_watched.set(true);
                }
            });

            // video duration watched - warehousing
            let post_for_warehouse = vid_details.clone();
            let _ = use_event_listener(container_ref, ev::pause, move |evt| {
                let user = user_details_can_store_or_ret!(cans_store);
                let post_o = post_for_warehouse();
                let post = post_o.as_ref();

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
                        "publisher_user_id": post.map(|p| p.poster_principal),
                        "user_id": user.details.principal,
                        "is_loggedIn": is_connected(),
                        "display_name": user.details.display_name.clone(),
                        "canister_id": user.canister_id,
                        "video_id": post.map(|p| p.uid.clone()),
                        "video_category": "NA",
                        "creator_category": "NA",
                        "hashtag_count": post.map(|p| p.hastags.len()),
                        "is_NSFW": post.map(|p| p.is_nsfw),
                        "is_hotorNot": post.map(|p| p.is_hot_or_not()),
                        "feed_type": "NA",
                        "view_count": post.map(|p| p.views),
                        "like_count": post.map(|p| p.likes),
                        "share_count": 0,
                        "percentage_watched": percentage_watched,
                        "absolute_watched": current_time,
                        "video_duration": duration,
                        "post_id": post.map(|p| p.post_id),
                        "publisher_canister_id": post.map(|p| p.canister_id),
                    }),
                );
            });
        }
    }
}

#[derive(Default)]
pub struct LikeVideo;

impl LikeVideo {
    pub fn send_event(
        &self,
        post_details: PostDetails,
        likes: RwSignal<u64>,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let publisher_user_id = post_details.poster_principal;
            let video_id = post_details.uid.clone();
            let hastag_count = post_details.hastags.len();
            let is_nsfw = post_details.is_nsfw;
            let is_hotornot = post_details.hot_or_not_feed_ranking_score.is_some();
            let view_count = post_details.views;
            let post_id = post_details.post_id;
            let publisher_canister_id = post_details.canister_id;

            let (is_connected, _) = account_connected_reader();
            // like_video - analytics

            let user = user_details_can_store_or_ret!(cans_store);

            send_event(
                "like_video",
                &json!({
                    "publisher_user_id":publisher_user_id,
                    "user_id": user.details.principal,
                    "is_loggedIn": is_connected(),
                    "display_name": user.details.display_name,
                    "canister_id": user.canister_id,
                    "video_id": video_id,
                    "video_category": "NA",
                    "creator_category": "NA",
                    "hashtag_count": hastag_count,
                    "is_NSFW": is_nsfw,
                    "is_hotorNot": is_hotornot,
                    "feed_type": "NA",
                    "view_count": view_count,
                    "like_count": likes.get(),
                    "share_count": 0,
                    "post_id": post_id,
                    "publisher_canister_id": publisher_canister_id,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct ShareVideo;

impl ShareVideo {
    pub fn send_event(
        &self,
        post_details: PostDetails,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let publisher_user_id = post_details.poster_principal;
            let video_id = post_details.uid.clone();
            let hastag_count = post_details.hastags.len();
            let is_nsfw = post_details.is_nsfw;
            let is_hotornot = post_details.hot_or_not_feed_ranking_score.is_some();
            let view_count = post_details.views;
            let like_count = post_details.likes;

            let (is_connected, _) = account_connected_reader();

            let user = user_details_can_store_or_ret!(cans_store);

            // share_video - analytics
            send_event(
                "share_video",
                &json!({
                    "publisher_user_id":publisher_user_id,
                    "user_id": user.details.principal,
                    "is_loggedIn": is_connected.get(),
                    "display_name": user.details.display_name,
                    "canister_id": user.canister_id,
                    "video_id": video_id,
                    "video_category": "NA",
                    "creator_category": "NA",
                    "hashtag_count": hastag_count,
                    "is_NSFW": is_nsfw,
                    "is_hotorNot": is_hotornot,
                    "feed_type": "NA",
                    "view_count": view_count,
                    "like_count": like_count,
                    "share_count": 0,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct VideoUploadInitiated;

impl VideoUploadInitiated {
    pub fn send_event(&self) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // video_upload_initiated - analytics
            let user = user_details_or_ret!();
            send_event(
                "video_upload_initiated",
                &json!({
                    "user_id": user.details.principal,
                    "display_name": user.details.display_name,
                    "canister_id": user.canister_id,
                    "creator_category": "NA",
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct VideoUploadUploadButtonClicked;

impl VideoUploadUploadButtonClicked {
    pub fn send_event(
        &self,
        hashtag_inp: NodeRef<Input>,
        is_nsfw: NodeRef<Input>,
        enable_hot_or_not: NodeRef<Input>,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // video_upload_upload_button_clicked - analytics
            let user = user_details_can_store_or_ret!(cans_store);

            let hashtag_count = hashtag_inp.get_untracked().unwrap().value().len();
            let is_nsfw_val = is_nsfw
                .get_untracked()
                .map(|v| v.checked())
                .unwrap_or_default();
            let is_hotornot_val = enable_hot_or_not
                .get_untracked()
                .map(|v| v.checked())
                .unwrap_or_default();

            create_effect(move |_| {
                send_event(
                    "video_upload_upload_button_clicked",
                    &json!({
                        "user_id": user.details.principal,
                        "display_name": user.details.display_name.clone().unwrap_or_default(),
                        "canister_id": user.canister_id,
                        "creator_category": "NA",
                        "hashtag_count": hashtag_count,
                        "is_NSFW": is_nsfw_val,
                        "is_hotorNot": is_hotornot_val,
                    }),
                );
            });
        }
    }
}

#[derive(Default)]
pub struct VideoUploadVideoSelected;

impl VideoUploadVideoSelected {
    pub fn send_event(&self, cans_store: RwSignal<Option<Canisters<true>>>) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // video_upload_video_selected - analytics
            let user = user_details_can_store_or_ret!(cans_store);

            send_event(
                "video_upload_video_selected",
                &json!({
                    "user_id": user.details.principal,
                    "display_name": user.details.display_name.unwrap_or_default(),
                    "canister_id": user.canister_id,
                    "creator_category": "NA",
                }),
            )
        }
    }
}

#[derive(Default)]
pub struct VideoUploadUnsuccessful;

impl VideoUploadUnsuccessful {
    #[allow(clippy::too_many_arguments)]
    pub fn send_event(
        &self,
        error: String,
        hashtags_len: usize,
        is_nsfw: bool,
        enable_hot_or_not: bool,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // video_upload_unsuccessful - analytics
            let user = user_details_can_store_or_ret!(cans_store);

            send_event(
                "video_upload_unsuccessful",
                &json!({
                    "user_id": user.details.principal,
                    "display_name": user.details.display_name.unwrap_or_default(),
                    "canister_id": user.canister_id,
                    "creator_category": "NA",
                    "hashtag_count": hashtags_len,
                    "is_NSFW": is_nsfw,
                    "is_hotorNot": enable_hot_or_not,
                    "fail_reason": error,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct VideoUploadSuccessful;

impl VideoUploadSuccessful {
    pub fn send_event(
        &self,
        video_id: String,
        hashtags_len: usize,
        is_nsfw: bool,
        enable_hot_or_not: bool,
        post_id: u64,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // video_upload_successful - analytics
            let user = user_details_can_store_or_ret!(cans_store);
            send_event(
                "video_upload_successful",
                &json!({
                    "user_id": user.details.principal,
                    "publisher_user_id": user.details.principal,
                    "display_name": user.details.display_name,
                    "canister_id": user.canister_id,
                    "creator_category": "NA",
                    "hashtag_count": hashtags_len,
                    "is_NSFW": is_nsfw,
                    "is_hotorNot": enable_hot_or_not,
                    "is_filter_used": false,
                    "video_id": video_id,
                    "post_id": post_id,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct Refer;

impl Refer {
    pub fn send_event(&self, logged_in: ReadSignal<bool>) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // refer - analytics

            let user = user_details_or_ret!();
            let details = user.details;
            let user_id = details.principal;
            let display_name = details.display_name;
            let canister_id = user.canister_id;

            let history_ctx: HistoryCtx = expect_context();
            let prev_site = history_ctx.prev_url_untracked();

            // refer - analytics
            send_event(
                "refer",
                &json!({
                    "user_id":user_id,
                    "is_loggedIn": logged_in.get_untracked(),
                    "display_name": display_name,
                    "canister_id": canister_id,
                    "refer_location": prev_site,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct ReferShareLink;

impl ReferShareLink {
    pub fn send_event(
        &self,
        logged_in: ReadSignal<bool>,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // refer_share_link - analytics
            let user = user_details_can_store_or_ret!(cans_store);
            let details = user.details;

            let user_id = details.principal;
            let display_name = details.display_name;
            let canister_id = user.canister_id;

            let history_ctx: HistoryCtx = expect_context();
            let prev_site = history_ctx.prev_url_untracked();

            // refer_share_link - analytics
            send_event(
                "refer_share_link",
                &json!({
                    "user_id":user_id,
                    "is_loggedIn": logged_in.get_untracked(),
                    "display_name": display_name,
                    "canister_id": canister_id,
                    "refer_location": prev_site,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct LoginSuccessful;

impl LoginSuccessful {
    pub fn send_event(&self, canisters: Canisters<true>) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // login_successful - analytics

            let user_id = canisters.identity().sender().unwrap();
            let canister_id = canisters.user_canister();

            send_user_id(user_id.to_string());

            // login_successful - analytics
            send_event(
                "login_successful",
                &json!({
                    "login_method": "google", // TODO: change this when more providers are added
                    "user_id": user_id.to_string(),
                    "canister_id": canister_id.to_string(),
                    "is_new_user": false,                   // TODO: add this info
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct LoginMethodSelected;

impl LoginMethodSelected {
    pub fn send_event(&self, prov: ProviderKind) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // login_method_selected - analytics
            send_event(
                "login_method_selected",
                &json!({
                    "login_method": match prov {
                        #[cfg(feature = "local-auth")]
                        ProviderKind::LocalStorage => "local_storage",
                        #[cfg(any(feature = "oauth-ssr", feature = "oauth-hydrate"))]
                        ProviderKind::Google => "google",
                    },
                    "attempt_count": 1,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct LoginJoinOverlayViewed;

impl LoginJoinOverlayViewed {
    pub fn send_event(&self) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // login_join_overlay_viewed - analytics
            let user = user_details_or_ret!();
            let event_history: EventHistory = expect_context();

            let user_id = user.details.principal;

            send_event(
                "login_join_overlay_viewed",
                &json!({
                    "user_id_viewer": user_id,
                    "previous_event": event_history.event_name.get_untracked(),
                }),
            );

            send_user_id(user_id.to_string());
        }
    }
}

#[derive(Default)]
pub struct LoginCta;

impl LoginCta {
    pub fn send_event(&self, cta_location: String) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            // login_cta - analytics

            let event_history: EventHistory = expect_context();

            send_event(
                "login_cta",
                &json!({
                    "previous_event": event_history.event_name.get_untracked(),
                    "cta_location": cta_location,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct LogoutClicked;

impl LogoutClicked {
    pub fn send_event(&self, cans_store: RwSignal<Option<Canisters<true>>>) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let user = user_details_can_store_or_ret!(cans_store);
            let details = user.details;
            // logout_clicked - analytics

            let user_id = details.principal;
            let display_name = details.display_name;
            let canister_id = user.canister_id;

            send_event(
                "logout_clicked",
                &json!({
                    "user_id_viewer": user_id,
                    "display_name": display_name,
                    "canister_id": canister_id,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct LogoutConfirmation;

impl LogoutConfirmation {
    pub fn send_event(&self, cans_store: RwSignal<Option<Canisters<true>>>) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let user = user_details_can_store_or_ret!(cans_store);
            let details = user.details;

            let user_id = details.principal;
            let display_name = details.display_name;
            let canister_id = user.canister_id;
            // logout_confirmation - analytics

            send_event(
                "logout_confirmation",
                &json!({
                    "user_id_viewer": user_id,
                    "display_name": display_name,
                    "canister_id": canister_id,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct ErrorEvent;

impl ErrorEvent {
    pub fn send_event(&self, error_str: String, cans_store: RwSignal<Option<Canisters<true>>>) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let event_history: EventHistory = expect_context();
            let user = user_details_can_store_or_ret!(cans_store);
            let details = user.details;

            let user_id = details.principal;
            let canister_id = user.canister_id;

            // error_event - analytics
            send_event(
                "error_event",
                &json!({
                    "user_id": user_id,
                    "canister_id": canister_id,
                    "description": error_str,
                    "previous_event": event_history.event_name.get_untracked(),
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct ProfileViewVideo;

impl ProfileViewVideo {
    pub fn send_event(
        &self,
        post_details: PostDetails,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let publisher_user_id = post_details.poster_principal;
            let video_id = post_details.uid.clone();

            let (is_connected, _) = account_connected_reader();

            let user = user_details_can_store_or_ret!(cans_store);

            send_event(
                "profile_view_video",
                &json!({
                    "publisher_user_id":publisher_user_id,
                    "user_id": user.details.principal,
                    "is_loggedIn": is_connected(),
                    "display_name": user.details.display_name,
                    "canister_id": user.canister_id,
                    "video_id": video_id,
                    "profile_feed": "main",
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct TokenCreationStarted;

impl TokenCreationStarted {
    pub fn send_event(
        &self,
        sns_init_payload: SnsInitPayload,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let user = user_details_can_store_or_ret!(cans_store);
            let details = user.details;

            let user_id = details.principal;
            let canister_id = user.canister_id;

            // token_creation_started - analytics
            send_event(
                "token_creation_started",
                &json!({
                    "user_id": user_id,
                    "canister_id": canister_id,
                    "token_name": sns_init_payload.token_name,
                    "token_symbol": sns_init_payload.token_symbol,
                    "name": sns_init_payload.name
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct TokenCreationCompleted;

impl TokenCreationCompleted {
    pub fn send_event(
        &self,
        sns_init_payload: SnsInitPayload,
        token_root: Principal,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let user = user_details_can_store_or_ret!(cans_store);
            let details = user.details;

            let user_id = details.principal;
            let canister_id = user.canister_id;

            let link = format!("/token/info/{token_root}");

            // token_creation_completed - analytics
            send_event(
                "token_creation_completed",
                &json!({
                    "user_id": user_id,
                    "canister_id": canister_id,
                    "token_name": sns_init_payload.token_name,
                    "token_symbol": sns_init_payload.token_symbol,
                    "name": sns_init_payload.name,
                    "description": sns_init_payload.description,
                    "logo": sns_init_payload.logo,
                    "link": link,
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct TokenCreationFailed;

impl TokenCreationFailed {
    pub fn send_event(
        &self,
        error_str: String,
        sns_init_payload: SnsInitPayload,
        cans_store: RwSignal<Option<Canisters<true>>>,
    ) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let user = user_details_can_store_or_ret!(cans_store);
            let details = user.details;

            let user_id = details.principal;
            let canister_id = user.canister_id;

            // token_creation_failed - analytics
            send_event(
                "token_creation_failed",
                &json!({
                    "user_id": user_id,
                    "canister_id": canister_id,
                    "token_name": sns_init_payload.token_name,
                    "token_symbol": sns_init_payload.token_symbol,
                    "name": sns_init_payload.name,
                    "description": sns_init_payload.description,
                    "error": error_str
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct TokensClaimedFromNeuron;

impl TokensClaimedFromNeuron {
    pub fn send_event(&self, amount: u64, cans_store: Canisters<true>) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let details = cans_store.profile_details();

            let user_id = details.principal;
            let canister_id = cans_store.user_canister();

            // tokens_claimed_from_neuron - analytics
            send_event(
                "tokens_claimed_from_neuron",
                &json!({
                    "user_id": user_id,
                    "canister_id": canister_id,
                    "amount": amount
                }),
            );
        }
    }
}

#[derive(Default)]
pub struct TokensTransferred;

impl TokensTransferred {
    pub fn send_event(&self, amount: String, to: Principal, cans_store: Canisters<true>) {
        #[cfg(all(feature = "hydrate", feature = "ga4"))]
        {
            let details = cans_store.profile_details();

            let user_id = details.principal;
            let canister_id = cans_store.user_canister();

            // tokens_transferred - analytics
            send_event(
                "tokens_transferred",
                &json!({
                    "user_id": user_id,
                    "canister_id": canister_id,
                    "amount": amount,
                    "to": to
                }),
            );
        }
    }
}
