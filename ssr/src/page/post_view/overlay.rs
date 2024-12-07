use crate::{
    component::{
        canisters_prov::with_cans, hn_icons::HomeFeedShareIcon, modal::Modal, option::SelectOption,
    },
    state::canisters::auth_canisters_store,
    utils::{
        event_streaming::events::{LikeVideo, ShareVideo},
        report::ReportOption,
        send_wrap,
        user::UserDetails,
        web::{copy_to_clipboard, share_url},
    },
};
use gloo::timers::callback::Timeout;
use leptos::{prelude::*, task::spawn_local};
use leptos_icons::*;
use leptos_use::use_window;
use yral_canisters_common::{utils::posts::PostDetails, Canisters};

use super::bet::HNGameOverlay;

#[component]
fn LikeAndAuthCanLoader(post: PostDetails) -> impl IntoView {
    let likes = RwSignal::new(post.likes);

    let liked = RwSignal::new(None::<bool>);
    let icon_name = Signal::derive(move || {
        if liked().unwrap_or_default() {
            "/img/heart-icon-liked.svg"
        } else {
            "/img/heart-icon-white.svg"
        }
    });

    let post_canister = post.canister_id;
    let post_id = post.post_id;
    let initial_liked = (post.liked_by_user, post.likes);
    let canisters = auth_canisters_store();

    // TODO: use Action::new_local
    let like_toggle: Action<_, _, LocalStorage> = Action::new_unsync(move |&()| {
        let post_details = post.clone();
        let canister_store = canisters;

        async move {
            let Some(canisters) = canisters.get_untracked() else {
                log::warn!("Trying to toggle like without auth");
                return;
            };

            let mut likes_w = likes.write();
            let mut liked_w = liked.write();
            if liked_w.unwrap_or_default() {
                *likes_w -= 1;
                *liked_w = Some(false);
            } else {
                *likes_w -= 1;
                *liked_w = Some(true);
                LikeVideo.send_event(post_details, likes, canister_store);
            }
            // this is important to commit the writes to the signal
            std::mem::drop((likes_w, liked_w));

            let individual = canisters.individual_user(post_canister).await;
            match individual
                .update_post_toggle_like_status_by_caller(post_id)
                .await
            {
                Ok(_) => (),
                Err(e) => {
                    log::warn!("Error toggling like status: {:?}", e);
                    liked.update(|l| _ = l.as_mut().map(|l| *l = !*l));
                }
            }
        }
    });

    let liked_fetch = with_cans(move |cans: Canisters<true>| async move {
        if let Some(liked) = initial_liked.0 {
            return Ok((liked, initial_liked.1));
        }

        let fut = send_wrap(cans.post_like_info(post_canister, post_id));
        Ok(fut.await?)
    });

    let liking = like_toggle.pending();

    view! {
        <div class="flex flex-col gap-1 items-center">
            <button
                on:click=move |_| { like_toggle.dispatch(()); }
                disabled=move || liking() || liked.with(|l| l.is_none())
            >
                <img src=icon_name style="width: 1em; height: 1em;" />
            </button>
            <span class="text-sm md:text-md">{likes}</span>
            <Suspense>
                {move || Suspend::new(async move {
                    match liked_fetch.await {
                        Ok(res) => {
                            likes.set(res.1);
                            liked.set(Some(res.0))
                        },
                        Err(e) => {
                            log::warn!("failed to fetch like status {e}");
                        }
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
pub fn VideoDetailsOverlay(post: PostDetails) -> impl IntoView {
    let show_share = RwSignal::new(false);
    let show_report = RwSignal::new(false);
    let (report_option, set_report_option) = signal(ReportOption::Nudity.as_str().to_string());
    let show_copied_popup = RwSignal::new(false);
    let base_url = || {
        use_window()
            .as_ref()
            .and_then(|w| w.location().origin().ok())
    };
    let video_url = move || {
        base_url()
            .map(|b| format!("{b}/hot-or-not/{}/{}", post.canister_id, post.post_id))
            .unwrap_or_default()
    };

    let post_details_share = post.clone();
    let canisters = auth_canisters_store();
    let canisters_copy = canisters;

    let share = move || {
        let post_details = post_details_share.clone();
        let url = video_url();
        if share_url(&url).is_some() {
            return;
        }
        show_share.set(true);
        ShareVideo.send_event(post_details, canisters);
    };

    let profile_url = format!("/profile/{}/tokens", post.poster_principal.to_text());
    let post_c = post.clone();

    let click_copy = move |text: String| {
        _ = copy_to_clipboard(&text);
        show_copied_popup.set(true);
        Timeout::new(1200, move || show_copied_popup.set(false)).forget();
    };

    let post_details_report = post.clone();
    let click_report = Action::new(move |()| {
        #[cfg(feature = "ga4")]
        {
            use crate::utils::report::send_report_offchain;

            let post_details = post_details_report.clone();
            let user_details = UserDetails::try_get_from_canister_store(canisters_copy).unwrap();

            spawn_local(async move {
                send_report_offchain(
                    user_details.details.principal.to_string(),
                    post_details.poster_principal.to_string(),
                    post_details.canister_id.to_string(),
                    post_details.post_id.to_string(),
                    post_details.uid,
                    report_option.get_untracked(),
                    video_url(),
                )
                .await
                .unwrap();
            });
        }

        async move {
            show_report.set(false);
        }
    });

    view! {
        <div class="flex flex-col pointer-events-none flex-nowrap h-full justify-between pt-5 pb-20 px-2 md:px-6 w-full text-white absolute bottom-0 left-0 bg-transparent z-[4]">
            <div class="flex pointer-events-auto flex-row gap-2 w-9/12 rounded-s-full bg-gradient-to-r from-black/25 via-80% via-black/10 items-center p-2">
                <div class="w-fit flex">
                    <a
                        href=profile_url.clone()
                        class="w-10 md:w-12 h-10 md:h-12 overflow-clip rounded-full border-primary-600 border-2"
                    >
                        <img class="h-full w-full object-cover" src=post.propic_url />
                    </a>
                </div>
                <div class="flex flex-col justify-center min-w-0">
                    <div class="flex flex-row text-xs md:text-sm lg:text-base gap-1">
                        <span class="font-semibold truncate">
                            <a href=profile_url>{post.display_name}</a>
                        </span>
                        <span class="font-semibold">"|"</span>
                        <span class="flex flex-row gap-1 items-center">
                            <Icon
                                class="text-sm md:text-base lg:text-lg"
                                icon=icondata::AiEyeOutlined
                            />
                            {post.views}
                        </span>
                    </div>
                    <ExpandableText description=post.description />
                </div>
            </div>
            <div class="flex flex-col gap-2 w-full">
                <div class="flex flex-col pointer-events-auto gap-6 self-end items-end text-2xl md:text-3xl lg:text-4xl">
                    <button on:click=move |_| show_report.set(true)>
                        <Icon class="drop-shadow-lg" icon=icondata::TbMessageReport />
                    </button>
                    <a href="/refer-earn">
                        <Icon class="drop-shadow-lg" icon=icondata::AiGiftFilled />
                    </a>
                    <LikeAndAuthCanLoader post=post_c.clone() />
                    <button on:click=move |_| share()>
                        <Icon class="drop-shadow-lg" icon=HomeFeedShareIcon />
                    </button>
                </div>
                <div class="w-full bg-transparent pointer-events-auto">
                    <HNGameOverlay post=post_c />
                </div>
            </div>
        </div>
        <Modal show=show_share>
            <div class="flex flex-col justify-center items-center gap-4 text-white">
                <span class="text-lg">Share</span>
                <div class="flex flex-row w-full gap-2">
                    <p class="text-md max-w-full bg-white/10 rounded-full p-2 overflow-x-scroll whitespace-nowrap">
                        {video_url}
                    </p>
                    <button on:click=move |_| click_copy(video_url())>
                        <Icon class="text-xl" icon=icondata::FaCopyRegular />
                    </button>
                </div>
            </div>

            <Show when=show_copied_popup>
                <div class="flex flex-col justify-center items-center">
                    <span class="absolute mt-80 flex flex-row justify-center items-center bg-white/90 rounded-md h-10 w-28 text-center shadow-lg">
                        <p>Link Copied!</p>
                    </span>
                </div>
            </Show>
        </Modal>
        <Modal show=show_report>
            <div class="flex flex-col justify-center items-center gap-4 text-white">
                <span class="text-lg">Report Post</span>
                <span class="text-lg">Please select a reason:</span>
                <div class="max-w-full text-md text-black">
                    <select
                        class="p-2 w-full block rounded-lg text-sm"
                        on:change=move |ev| {
                            let new_value = event_target_value(&ev);
                            set_report_option(new_value);
                        }
                    >

                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Nudity.as_str())
                        />
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Violence.as_str())
                        />
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Offensive.as_str())
                        />
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Spam.as_str())
                        />
                        <SelectOption
                            value=report_option
                            is=format!("{}", ReportOption::Other.as_str())
                        />
                    </select>
                </div>
                <button on:click=move |_| { click_report.dispatch(()); }>
                    <div class="rounded-lg bg-pink-500 p-1">Submit</div>
                </button>
            </div>
        </Modal>
    }
}

#[component]
fn ExpandableText(description: String) -> impl IntoView {
    let truncated = RwSignal::new(true);

    view! {
        <span
            class="text-xs md:text-sm lg:text-base w-full"
            class:truncate=truncated

            on:click=move |_| truncated.update(|e| *e = !*e)
        >
            {description}
        </span>
    }
}
