use candid::Principal;
use leptos::{html::Video, *};
use leptos_router::*;

use super::overlay::YourProfileOverlay;
use crate::{
    canister::utils::bg_url,
    component::{back_btn::BackButton, spinner::FullScreenSpinner, video_player::VideoPlayer},
    state::canisters::{authenticated_canisters, unauth_canisters},
    try_or_redirect_opt,
    utils::{posts::get_post_uid, route::failure_redirect},
};

use crate::page::post_view::overlay::VideoDetailsOverlay;

#[derive(Params, PartialEq)]
struct ProfileVideoParams {
    canister_id: String,
    post_id: u64,
}

#[component]
pub fn YourProfilePost() -> impl IntoView {
    let params = use_params::<ProfileVideoParams>();

    let canister_and_post = move || {
        params.with(|p| {
            let p = p.as_ref().ok()?;
            let canister_id = Principal::from_text(&p.canister_id).ok()?;

            Some((canister_id, p.post_id))
        })
    };

    let auth_cans_res = authenticated_canisters();

    let post_details = create_resource(canister_and_post, move |canister_and_post| async move {
        let (creator_canister_id, post_id) = canister_and_post?;
        let auth_canisters = leptos::untrack(|| auth_cans_res.get().transpose());
        let auth_canisters = try_or_redirect_opt!(auth_canisters);
        match auth_canisters {
            Some(canisters) => match get_post_uid(&canisters, creator_canister_id, post_id).await {
                Ok(pd) => pd,
                Err(e) => {
                    failure_redirect(e);
                    None
                }
            },
            None => {
                let canisters = unauth_canisters();
                match get_post_uid(&canisters, creator_canister_id, post_id).await {
                    Ok(pd) => pd,
                    Err(e) => {
                        failure_redirect(e);
                        None
                    }
                }
            }
        }
    });

    let video_node_ref = create_node_ref::<Video>();

    // Handles autoplay
    create_effect(move |_| {
        let Some(vid) = video_node_ref() else {
            return;
        };

        vid.set_autoplay(true);
        _ = vid.play();

        vid.set_muted(false);
    });
    let muted = create_rw_signal(false);

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                post_details
                    .get()
                    .flatten()
                    .map(|post| {
                        let uid = post.uid.clone();
                        let view_bg_url = bg_url(&uid);
                        let bg_img = format!("url({view_bg_url}");
                        Some(
                            view! {
                                <div class="absolute left-4 top-4 bg-transparent z-10 text-white">
                                    <BackButton fallback="/".to_string()/>
                                </div>
                                <YourProfileOverlay/>
                                <div class="snap-always snap-end w-dvh h-dvh">
                                    <div class="bg-transparent w-full h-full relative overflow-hidden">
                                        <div
                                            class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                                            style:background-color="rgb(0, 0, 0)"
                                            style:background-image=bg_img
                                        ></div>
                                    </div>
                                    <VideoDetailsOverlay post/>
                                    <VideoPlayer
                                        muted=muted.write_only()
                                        node_ref=video_node_ref
                                        uid=Some(uid.clone())
                                    />
                                </div>
                            },
                        )
                    })
            }}

        </Suspense>
    }
}
