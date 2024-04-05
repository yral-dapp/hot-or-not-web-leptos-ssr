use super::{overlay::VideoDetailsOverlay, PostViewCtx};
use crate::{
    canister::{
        individual_user_template::PostViewDetailsFromFrontend,
        utils::{bg_url, mp4_url},
    },
    component::feed_popup::FeedPopUp,
    state::{
        auth::{account_connected_reader, auth_state, AuthState},
        canisters::{authenticated_canisters, unauth_canisters},
        local_storage::use_referrer_store,
    },
    try_or_redirect, try_or_redirect_opt,
};
use gloo::console::info;
use leptos::{html::Video, *};
use leptos_use::use_event_listener;
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
                    header_text = "Your Rewards are
                                    Waiting!"
                    body_text = "SignUp/Login to save your progress and claim your rewards."
                    login_text = "Login"
                />
            </Show>
            <Show when=move || { referrer_store.get().is_some() && idx == 0 && !is_connected.get() && show_refer_login_popup.get() }>
                <FeedPopUp
                    on_click=move |_| set_show_refer_login_popup.set(false)
                    header_text = "Claim Your Referral
                                    Rewards Now!"
                    body_text = "SignUp from this link to get 500 COYNs as referral rewards."
                    login_text = "Sign Up"
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

    #[cfg(all(feature = "hydrate"))]
    {
        let (watched_percentage, set_watched_percentage) = create_signal(0 as u8);
        let (watched_count, set_watched_count) = create_signal(0 as u8);

        let _ = use_event_listener(container_ref, ev::timeupdate, move |event| {
            let target = event.target().unwrap();
            let video = target.unchecked_into::<web_sys::HtmlVideoElement>();
            let duration = video.duration();
            let current_time = video.current_time();

            set_watched_percentage.update(|watched_percentage| {
                *watched_percentage = (100.0 * (current_time / duration)) as u8;
            });

            if current_time == 0.0 {
                set_watched_count.update(|count| *count += 1);
            }
        });

        let post = move || video_queue.get_untracked().get(idx).cloned();
        let canister_id = move || post().as_ref().map(|q| q.canister_id).unwrap();
        let post_id = move || post().as_ref().map(|q| q.post_id).unwrap();

        let send_view_detail_action = create_action(move |()| async move {
            let canisters = unauth_canisters();
            let payload = match watched_count.get_untracked() {
                0 => PostViewDetailsFromFrontend::WatchedPartially {
                    percentage_watched: watched_percentage.get_untracked(),
                },
                _ => PostViewDetailsFromFrontend::WatchedMultipleTimes {
                    percentage_watched: watched_percentage.get_untracked(),
                    watch_count: watched_count.get_untracked(),
                },
            };
            canisters
                .individual_user(canister_id())
                .update_post_add_view_details(post_id(), payload)
                .await
        });

        create_effect(move |_| {
            if current_idx() != idx {
                send_view_detail_action.dispatch(());
            }
        });
    }

    view! {

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
    }
}
