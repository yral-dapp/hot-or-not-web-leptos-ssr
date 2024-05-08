use leptos::*;
use leptos_icons::*;

use candid::Principal;

use crate::{
    canister::utils::bg_url,
    component::no_more_posts::NoMorePostsGraphic,
    state::canisters::{authenticated_canisters, unauth_canisters},
    utils::profile::{PostDetails, PostsProvider},
};

use super::ic::ProfileStream;

#[component]
fn Post(details: PostDetails, user_canister: Principal, _ref: NodeRef<html::Div>) -> impl IntoView {
    let image_error = create_rw_signal(false);

    let auth_canister = authenticated_canisters();

    let auth_canister_id = auth_canister()
        .transpose()
        .ok()
        .flatten()
        .map(|canisters| canisters.user_canister());

    let profile_post_url = match auth_canister_id {
        Some(canister_id) if canister_id == user_canister => {
            format!("/your-profile/{}/{}", canister_id, details.id)
        }
        _ => {
            format!("/profile/{}/{}", user_canister, details.id)
        }
    };

    let handle_image_error =
        move |_| image_error.update(|image_error| *image_error = !*image_error);

    let handle_image_error =
        move |_| image_error.update(|image_error| *image_error = !*image_error);

    let profile_post_url = format!("/profile/{}/{}", user_canister, details.id);
    view! {
        <div _ref=_ref class="relative w-full basis-1/3 md:basis-1/4 xl:basis-1/5">
            <div class="relative aspect-[9/16] h-full rounded-md border-white/20 m-2 border-[1px]">
                <a class = "h-full w-full" href = profile_post_url>
                    <Show when=image_error
                        fallback=move || view! {
                            <img class="object-cover w-full h-full" on:error=handle_image_error src=bg_url(details.uid.clone())/>
                        }
                    >
                        <div class="h-full flex text-center flex-col place-content-center items-center text-white">
                            <Icon class="h-8 w-8" icon=icondata::TbCloudX/>
                            <span class="text-md">Not Available</span>
                        </div>
                    </Show>

                    <div class="absolute bottom-1 left-1 grid grid-cols-2 items-center gap-1">
                        <Icon
                            class="h-5 w-5 p-1 text-primary-500 rounded-full bg-black/30"
                            icon=icondata::AiHeartOutlined
                        />
                        <span class="text-white text-xs">{details.likes}</span>
                    </div>
                    <div class="absolute bottom-1 right-1 grid grid-cols-2 items-center gap-1">
                        <Icon
                            class="h-5 w-5 p-1 text-white rounded-full bg-black/30"
                            icon=icondata::AiEyeOutlined
                        />
                        <span class="text-white text-xs">{details.views}</span>
                    </div>
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn ProfilePosts(user_canister: Principal) -> impl IntoView {
    let provider = PostsProvider::new(unauth_canisters(), user_canister);

    view! {
        <ProfileStream
            provider
            empty_graphic=NoMorePostsGraphic
            empty_text="No Videos Uploaded yet".into()
            children=move |details, _ref| {
                view! { <Post details=details user_canister=user_canister _ref=_ref.unwrap_or_default()/> }
            }
        />
    }
}
