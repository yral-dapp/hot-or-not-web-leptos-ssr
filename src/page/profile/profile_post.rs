use candid::Principal;
use leptos::{html::Video, *};
use leptos_router::*;

use crate::{
    canister::utils::{bg_url, mp4_url},
    component::{back_btn::BackButton, spinner::FullScreenSpinner, video_player::VideoPlayer},
    page::post_view::video_iter::get_post_uid,
    state::canisters::{authenticated_canisters, unauth_canisters},
    try_or_redirect_opt,
    utils::route::failure_redirect,
};

use crate::page::post_view::overlay::{HomeButtonOverlay, VideoDetailsOverlay};

#[derive(Params, PartialEq)]
struct ProfileVideoParams {
    canister_id: String,
    post_id: u64,
}

#[component]
pub fn ProfilePost() -> impl IntoView {
    let params = use_params::<ProfileVideoParams>();
    let your_profile = create_rw_signal(false);

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
            Some(canisters) => {
                if canisters.user_canister() == creator_canister_id {
                    your_profile.update(|yu| *yu = true);
                }
                match get_post_uid(&canisters, creator_canister_id, post_id).await {
                    Ok(pd) => pd,
                    Err(e) => {
                        failure_redirect(e);
                        None
                    }
                }
            }
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
        // let post_details_for_frontend = auth_canisters(creator_canister_id).get_individual_post_details_by_id(post_id).await.unwrap();
        // log::warn!("Fetched post");
        // Some(PostDetails::from_canister_post(post_details_for_frontend))
    });

    let view_video_url = move || post_details.get().flatten().map(|pd| mp4_url(pd.uid));

    let view_bg_url = move || post_details.get().flatten().map(|pd| bg_url(pd.uid));

    let video_node_ref = create_node_ref::<Video>();

    // Handles autoplay
    create_effect(move |_| {
        let Some(vid) = video_node_ref() else {
            return;
        };

        log::warn!("set autoplay as true");
        vid.set_autoplay(true);
        _ = vid.play();
    });

    view! {
        <Suspense fallback = FullScreenSpinner >
        {move || {
                post_details.get()
                    .flatten().map(|pd| {

                        Some(view!{
                            <div class ="absolute left-4 top-4 bg-transparent z-10 text-white">
                                <BackButton fallback="/".to_string()/>
                            </div>
                            <Show when=your_profile>
                                <HomeButtonOverlay/>
                            </Show>
                            <div class="snap-always snap-end w-full h-screen">
                                <div class="bg-transparent w-full h-full relative overflow-hidden">
                                    <div
                                        class="absolute top-0 left-0 bg-cover bg-center w-full h-full z-[1] blur-lg"
                                        style:background-color="rgb(0, 0, 0)"
                                        style:background-image=move || format!("url({})", view_bg_url().unwrap_or_default())
                                    ></div>
                                    </div>
                                <VideoDetailsOverlay post = pd/>
                                <VideoPlayer node_ref=video_node_ref view_bg_url view_video_url/>
                            </div>
                        })
                    })
            }
        }
        </Suspense>
    }
}
