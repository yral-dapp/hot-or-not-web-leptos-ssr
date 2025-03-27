use codee::string::FromToStringCodec;
use indexmap::IndexSet;
use leptos::{html::Video, prelude::*};
use leptos_use::storage::use_local_storage;

use crate::post_view::BetEligiblePostCtx;
use component::show_any::ShowAny;
use component::{
    feed_popup::FeedPopUp, onboarding_flow::OnboardingPopUp, video_player::VideoPlayer,
};
use consts::USER_ONBOARDING_STORE;
use state::local_storage::use_referrer_store;
use utils::event_streaming::events::VideoWatched;
use utils::{bg_url, event_streaming::events::account_connected_reader, mp4_url};

use super::{overlay::VideoDetailsOverlay, PostDetails};

#[component]
pub fn BgView(
    video_queue: RwSignal<IndexSet<PostDetails>>,
    current_idx: RwSignal<usize>,
    idx: usize,
    children: Children,
) -> impl IntoView {
    let post = Memo::new(move |_| video_queue.with(|q| q.get_index(idx).cloned()));
    let uid = move || post().as_ref().map(|q| q.uid.clone()).unwrap_or_default();

    let (is_connected, _) = account_connected_reader();
    let (show_login_popup, set_show_login_popup) = signal(true);

    let (show_refer_login_popup, set_show_refer_login_popup) = signal(true);
    let (referrer_store, _, _) = use_referrer_store();

    let onboarding_eligible_post_context = BetEligiblePostCtx::default();
    provide_context(onboarding_eligible_post_context.clone());

    let (show_onboarding_popup, set_show_onboarding_popup) = signal(false);
    let (is_onboarded, set_onboarded, _) =
        use_local_storage::<bool, FromToStringCodec>(USER_ONBOARDING_STORE);

    Effect::new(move |_| {
        if current_idx.get() % 5 != 0 {
            set_show_login_popup.update(|n| *n = false);
        } else {
            set_show_login_popup.update(|n| *n = true);
        }
        Some(())
    });

    Effect::new(move |_| {
        if onboarding_eligible_post_context.can_place_bet.get() && (!is_onboarded.get()) {
            set_show_onboarding_popup.update(|show| *show = true);
        } else {
            set_show_onboarding_popup.update(|show| *show = false);
        }
    });

    view! {
        <div class="bg-transparent w-full h-full relative overflow-hidden">
            <div
                class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                style:background-color="rgb(0, 0, 0)"
                style:background-image=move || format!("url({})", bg_url(uid()))
            ></div>
            <ShowAny when=move || {
                current_idx.get() != 0 && current_idx.get() % 5 == 0 && !is_connected.get()
                    && show_login_popup.get()
            }>
                <FeedPopUp
                    on_click=move |_| set_show_login_popup.set(false)
                    header_text="Your 1000 COYNs
                    Await You!"
                    body_text="SignUp/Login to save your progress and claim your rewards."
                    login_text="Login"
                />
            </ShowAny>
            <ShowAny when=move || {
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
            </ShowAny>
            <ShowAny when=move || { show_onboarding_popup.get() }>
                <OnboardingPopUp onboard_on_click=set_onboarded />
            </ShowAny>
            {move || post().map(|post| view! { <VideoDetailsOverlay post /> })}
            {children()}
        </div>
    }
    .into_any()
}

#[component]
pub fn VideoView(
    #[prop(into)] post: Signal<Option<PostDetails>>,
    #[prop(optional)] _ref: NodeRef<Video>,
    #[prop(optional)] autoplay_at_render: bool,
    muted: RwSignal<bool>,
) -> impl IntoView {
    let post_for_uid = post;
    let uid = Memo::new(move |_| post_for_uid.with(|p| p.as_ref().map(|p| p.uid.clone())));
    let view_bg_url = move || uid().map(bg_url);
    let view_video_url = move || uid().map(mp4_url);

    // Handles mute/unmute
    Effect::new(move |_| {
        let vid = _ref.get()?;
        vid.set_muted(muted());
        Some(())
    });

    Effect::new(move |_| {
        let vid = _ref.get()?;
        // the attributes in DOM don't seem to be working
        vid.set_muted(muted.get_untracked());
        vid.set_loop(true);
        if autoplay_at_render {
            vid.set_autoplay(true);
            _ = vid.play();
        }
        Some(())
    });

    VideoWatched.send_event(post, _ref);

    view! {
        <VideoPlayer
            node_ref=_ref
            view_bg_url=Signal::derive(view_bg_url)
            view_video_url=Signal::derive(view_video_url)
        />
    }
}

#[component]
pub fn VideoViewForQueue(
    video_queue: RwSignal<IndexSet<PostDetails>>,
    current_idx: RwSignal<usize>,
    idx: usize,
    muted: RwSignal<bool>,
) -> impl IntoView {
    let container_ref = NodeRef::<Video>::new();

    // Handles autoplay
    Effect::new(move |_| {
        let Some(vid) = container_ref.get() else {
            return;
        };
        if idx != current_idx() {
            _ = vid.pause();
            return;
        }
        vid.set_autoplay(true);
        _ = vid.play();
    });

    let post = Signal::derive(move || video_queue.with(|q| q.get_index(idx).cloned()));

    view! { <VideoView post _ref=container_ref muted /> }
}
