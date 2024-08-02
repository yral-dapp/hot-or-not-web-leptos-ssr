use crate::{
    component::{canisters_prov::WithAuthCans, modal::Modal, option::SelectOption},
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

use super::video_iter::post_liked_by_me;

#[component]
fn LikeAndAuthCanLoader(post: PostDetails) -> impl IntoView {
    let likes = create_rw_signal(post.likes);

    let liked = create_rw_signal(None::<bool>);
    let icon_class = Signal::derive(move || {
        if liked().unwrap_or_default() {
            Some(TextProp::from("fill-primary-600 size-8"))
        } else {
            Some(TextProp::from("size-8"))
        }
    });
    let icon_style = Signal::derive(move || {
        if liked().unwrap_or_default() {
            Some(TextProp::from("filter: drop-shadow(2px 0 0 white) drop-shadow(-2px 0 0 white) drop-shadow(0 2px 0 white) drop-shadow(0 -2px 0 white);"))
        } else {
            Some(TextProp::from("filter: drop-shadow(2px 0 0 white) drop-shadow(-2px 0 0 white) drop-shadow(0 2px 0 white) drop-shadow(0 -2px 0 white);"))
        }
    });

    let post_canister = post.canister_id;
    let post_id = post.post_id;
    let initial_liked = post.liked_by_user;
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
        if let Some(liked) = initial_liked {
            return liked;
        }

        match post_liked_by_me(&cans, post_canister, post_id).await {
            Ok(liked) => liked,
            Err(e) => {
                failure_redirect(e);
                false
            }
        }
    };

    let liking = like_toggle.pending();

    view! {
        <div class="relative flex flex-col gap-1 items-center">
            <button
                on:click=move |_| like_toggle.dispatch(())
                class="drop-shadow-lg"
                disabled=move || liking() || liked.with(|l| l.is_none())
            >
                <Icon class=icon_class style=icon_style icon=icondata::AiHeartFilled />
            </button>
            <span class="absolute -bottom-5 text-sm md:text-md">{likes}</span>
        </div>
        <WithAuthCans with=liked_fetch let:d>
            {move || liked.set(Some(d.1))}
        </WithAuthCans>
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
        <div class="flex flex-row flex-nowrap justify-between items-end pb-20 px-2 md:px-6 w-full text-white absolute bottom-0 left-0 bg-transparent z-[4]">
            <div class="flex flex-col gap-2 w-9/12">
                <div class="flex flex-row items-center gap-2 min-w-0">
                    <a
                        href=profile_url
                        class="w-10 md:w-12 h-10 md:h-12 overflow-clip rounded-full border-white border-2"
                    >
                        <img class="h-full w-full object-cover" src=post.propic_url />
                    </a>
                    <div class="flex flex-col w-7/12">
                        <span class="text-md md:text-lg font-bold truncate">
                            {post.display_name}
                        </span>
                        <span class="flex flex-row gap-1 items-center text-sm md:text-md">
                            <Icon icon=icondata::AiEyeOutlined />
                            {post.views}
                        </span>
                    </div>
                </div>
                <ExpandableText description=post.description />
            </div>
            <div class="flex flex-col gap-8 pb-10 items-end w-3/12 text-4xl">
                <button on:click=move |_| show_report.set(true)>
                    <Icon class="drop-shadow-lg" icon=icondata::TbMessageReport />
                </button>
                <a href="/refer-earn">
                    <Icon class="drop-shadow-lg" icon=icondata::AiGiftFilled />
                </a>
                <LikeAndAuthCanLoader post=post_c />
                <button on:click=move |_| share()>
                    <Icon class="drop-shadow-lg" icon=icondata::RiSendPlaneBusinessFill />
                </button>
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
                <button on:click=move |_| click_report.dispatch(())>
                    <div class="rounded-lg bg-pink-500 p-1">Submit</div>
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
            class="text-sm md:text-md ms-2 md:ms-4 w-full"
            class:truncate=truncated

            on:click=move |_| truncated.update(|e| *e = !*e)
        >
            {description}
        </span>
    }
}

#[component]
pub fn HomeButtonOverlay() -> impl IntoView {
    view! {
        <div class="flex w-full items-center justify-center pt-4 absolute top-0 left-0 bg-transparent z-[4]">
            // <div class="flex justify-center items-center">
            // <img src="/img/yral-logo.svg" alt="Logo"/>
            // </div>
            <div class="rounded-full p-2 text-white bg-black/20">
                <div class="flex flex-row items-center gap-1 py-2 px-6 rounded-full">
                    // <Icon class="w-3 h-3" icon=HomeSymbolFilled/>
                    <span class="font-sans font-semibold">Home Feed</span>
                </div>
            </div>
        </div>
    }
}
