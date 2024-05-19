use crate::{
    component::{canisters_prov::WithAuthCans, modal::Modal},
    state::canisters::{auth_canisters_store, Canisters},
    utils::{
        event_streaming::events::{LikeVideo, ShareVideo},
        route::failure_redirect,
        web::{copy_to_clipboard, share_url},
    },
};
use gloo::timers::callback::Timeout;
use leptos::*;
use leptos_icons::*;
use leptos_use::use_window;

use super::video_iter::{post_liked_by_me, PostDetails};

#[component]
fn LikeAndAuthCanLoader(post: PostDetails) -> impl IntoView {
    let likes = create_rw_signal(post.likes);

    let liked = create_rw_signal(None::<bool>);
    let icon_class = Signal::derive(move || {
        if liked().unwrap_or_default() {
            Some(TextProp::from("fill-primary-600"))
        } else {
            None
        }
    });
    let icon_style = Signal::derive(move || {
        if liked().unwrap_or_default() {
            Some(TextProp::from("filter: drop-shadow(2px 0 0 white) drop-shadow(-2px 0 0 white) drop-shadow(0 2px 0 white) drop-shadow(0 -2px 0 white);"))
        } else {
            None
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
            let individual = canisters.individual_user(post_canister);
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
                <Icon class=icon_class style=icon_style icon=icondata::AiHeartFilled/>
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

    let post_details = post.clone();
    let canisters = auth_canisters_store();

    let share = move || {
        let post_details = post_details.clone();
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

    view! {
        <div class="flex flex-row flex-nowrap justify-between items-end pb-20 px-2 md:px-6 w-full text-white absolute bottom-0 left-0 bg-transparent z-[4]">
            <div class="flex flex-col gap-2 w-9/12">
                <div class="flex flex-row items-center gap-2 min-w-0">
                    <a
                        href=profile_url
                        class="w-10 md:w-12 h-10 md:h-12 overflow-clip rounded-full border-white border-2"
                    >
                        <img class="h-full w-full object-cover" src=post.propic_url/>
                    </a>
                    <div class="flex flex-col w-7/12">
                        <span class="text-md md:text-lg font-bold truncate">
                            {post.display_name}
                        </span>
                        <span class="flex flex-row gap-1 items-center text-sm md:text-md">
                            <Icon icon=icondata::AiEyeOutlined/>
                            {post.views}
                        </span>
                    </div>
                </div>
                <ExpandableText description=post.description/>
            </div>
            <div class="flex flex-col gap-8 pb-10 items-end w-3/12 text-4xl">
                <a href="/refer-earn">
                    <Icon class="drop-shadow-lg" icon=icondata::AiGiftFilled/>
                </a>
                <LikeAndAuthCanLoader post=post_c/>
                <button on:click=move |_| share()>
                    <Icon class="drop-shadow-lg" icon=icondata::RiSendPlaneBusinessFill/>
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
                        <Icon class="text-xl" icon=icondata::FaCopyRegular/>
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
