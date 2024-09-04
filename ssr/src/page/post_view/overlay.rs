use crate::{
    component::{
        canisters_prov::WithAuthCans, hn_icons::HomeFeedShareIcon, modal::Modal,
        option::SelectOption,
    },
    state::canisters::{auth_canisters_store, Canisters},
    utils::{
        event_streaming::events::{LikeVideo, ShareVideo},
        posts::PostDetails,
        report::ReportOption,
        route::failure_redirect,
        user::UserDetails,
        web::{copy_to_clipboard, share_url},
    },
};
use gloo::timers::callback::Timeout;
use leptos::*;
use leptos_icons::*;
use leptos_use::use_window;

use super::{bet::HNGameOverlay, video_iter::post_liked_by_me};

#[component]
fn LikeAndAuthCanLoader(post: PostDetails) -> impl IntoView {
    let likes = create_rw_signal(post.likes);

    let liked = create_rw_signal(None::<bool>);
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

    let like_toggle = create_action(move |&()| {
        let post_details = post.clone();
        let canister_store = canisters;

        async move {
            let Some(canisters) = canisters.get_untracked() else {
                log::warn!("Trying to toggle like without auth");
                return;
            };
            batch(move || {
                if liked.get_untracked().unwrap_or_default() {
                    likes.update(|l| *l -= 1);
                    liked.set(Some(false));
                } else {
                    likes.update(|l| *l += 1);
                    liked.set(Some(true));

                    LikeVideo.send_event(post_details, likes, canister_store);
                }
            });
            let Ok(individual) = canisters.individual_user(post_canister).await else {
                return;
            };
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

    let liked_fetch = move |cans: Canisters<true>| async move {
        if let Some(liked) = initial_liked.0 {
            return (liked, initial_liked.1);
        }

        match post_liked_by_me(&cans, post_canister, post_id).await {
            Ok(liked) => liked,
            Err(e) => {
                failure_redirect(e);
                (false, likes.try_get_untracked().unwrap_or_default())
            }
        }
    };

    let liking = like_toggle.pending();

    view! {
        <div class="flex flex-col items-center ">
            <button
                on:click=move |_| like_toggle.dispatch(())
                disabled=move || liking() || liked.with(|l| l.is_none())
            >
                <img src=icon_name style="width: 27.55px; height: 27.55px;"/>
            </button>
            <span class="text-sm md:text-md ">{likes}</span>
            <WithAuthCans with=liked_fetch let:d>
                {move || {
                    likes.set(d.1.1);
                    liked.set(Some(d.1.0))
                }}

            </WithAuthCans>
        </div>

    }
}

#[component]
pub fn VideoDetailsOverlay(post: PostDetails) -> impl IntoView {
    let show_share = create_rw_signal(false);
    let show_report = create_rw_signal(false);
    let (report_option, set_report_option) =
        create_signal(ReportOption::Nudity.as_str().to_string());
    let show_copied_popup = create_rw_signal(false);
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

    let profile_url = format!("/profile/{}", post.poster_principal.to_text());
    let post_c = post.clone();

    let click_copy = move |text: String| {
        _ = copy_to_clipboard(&text);
        show_copied_popup.set(true);
        Timeout::new(1200, move || show_copied_popup.set(false)).forget();
    };

    let post_details_report = post.clone();
    let click_report = create_action(move |()| {
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
        <div class="flex absolute bottom-0 left-0 flex-col flex-nowrap justify-between px-2 pt-5 pb-20 w-full h-full text-white bg-transparent pointer-events-none md:px-6 z-[4]">
            <div class="flex pointer-events-auto flex-row gap-2 w-9/12 rounded-s-full bg-gradient-to-r from-black/25 via-80% via-black/10 items-center p-2">
                <div class="flex w-fit">
                    <a
                        href=profile_url.clone()
                        class="w-10 h-10 rounded-full border-2 md:w-12 md:h-12 overflow-clip border-primary-600"
                    >
                        <img class="object-cover w-full h-full" src=post.propic_url/>
                    </a>
                </div>
                <div class="flex flex-col justify-center min-w-0">
                    <div class="flex flex-row gap-1 text-xs md:text-sm lg:text-base">
                        <span class="font-semibold truncate">
                            <a
                                href=profile_url
                            >
                                {post.display_name}
                            </a>
                        </span>
                        <span class="font-semibold">"|"</span>
                        <span class="flex flex-row gap-1 items-center">
                            <Icon class="text-sm md:text-base lg:text-lg" icon=icondata::AiEyeOutlined/>
                            {post.views}
                        </span>
                    </div>
                    <ExpandableText description=post.description/>
                </div>
            </div>
            <div class="flex flex-col gap-2 w-full">
                <div class="flex flex-col gap-6 items-end self-end text-2xl pointer-events-auto md:text-3xl lg:text-4xl">
                    <button on:click=move |_| show_report.set(true)>
                        <Icon class="drop-shadow-lg w-[27.55px] h-[27.55px]" icon=icondata::TbMessageReport/>
                    </button>
                    <a href="/refer-earn">
                        <Icon class="drop-shadow-lg w-[27.55px] h-[27.55px]" icon=icondata::AiGiftFilled/>
                    </a>
                    <LikeAndAuthCanLoader post=post_c.clone() />
                    <button on:click=move |_| share()  >
                        <Icon class="drop-shadow-lg w-[27.55px] h-[27.55px] " icon=HomeFeedShareIcon />
                    </button>
                </div>
                <div class="w-full bg-transparent pointer-events-auto">
                    <HNGameOverlay post=post_c />
                </div>
            </div>
        </div>
        <Modal show=show_share>
            <div class="flex flex-col gap-4 justify-center items-center text-white">
                <span class="text-lg">Share</span>
                <div class="flex flex-row gap-2 w-full">
                    <p class="overflow-x-scroll p-2 max-w-full whitespace-nowrap rounded-full text-md bg-white/10">
                        {video_url}
                    </p>
                    <button on:click=move |_| click_copy(video_url())>
                        <Icon class="text-xl" icon=icondata::FaCopyRegular/>
                    </button>
                </div>
            </div>

            <Show when=show_copied_popup>
                <div class="flex flex-col justify-center items-center">
                    <span class="flex absolute flex-row justify-center items-center mt-80 w-28 h-10 text-center rounded-md shadow-lg bg-white/90">
                        <p>Link Copied!</p>
                    </span>
                </div>
            </Show>
        </Modal>
        <Modal show=show_report>
            <div class="flex flex-col gap-4 justify-center items-center text-white">
                <span class="text-lg">Report Post</span>
                <span class="text-lg">Please select a reason:</span>
                <div class="max-w-full text-black text-md">
                    <select
                        class="block p-2 w-full text-sm rounded-lg"
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
                <button on:click=move |_| click_report.dispatch(())>
                    <div class="p-1 bg-pink-500 rounded-lg">Submit</div>
                </button>
            </div>
        </Modal>
    }
}

#[component]
fn ExpandableText(description: String) -> impl IntoView {
    let truncated = create_rw_signal(true);

    view! {
        <span
            class="w-full text-xs md:text-sm lg:text-base"
            class:truncate=truncated

            on:click=move |_| truncated.update(|e| *e = !*e)
        >
            {description}
        </span>
    }
}
