use candid::Principal;
use leptos::*;
use leptos_icons::*;

use super::ic::ProfileStream;
use crate::{
    canister::utils::bg_url,
    state::canisters::unauth_canisters,
    utils::{
        posts::PostDetails,
        profile::{BetDetails, BetOutcome, BetsProvider, ProfileDetails},
    },
};

#[component]
pub fn ExternalUser(user: Option<ProfileDetails>) -> impl IntoView {
    let propic = user
        .as_ref()
        .map(|u| u.profile_pic_or_random())
        .unwrap_or_default();
    let name = user
        .as_ref()
        .map(|u| u.display_name_or_fallback())
        .unwrap_or_default();

    view! {
        <div class="flex flex-row items-center gap-1 w-full h-8 px-2 pt-2 text-ellipsis z-20">
            <img
                class="w-6 h-6 rounded-full border-2 border-white object-cover object-center"
                src=propic
            />
            <div class="text-xs line-clamp-1 text-nowrap text-ellipsis font-semibold sm:text-sm">
                {name}
            </div>
        </div>
    }
}

#[component]
pub fn ExternalPost(post: Option<PostDetails>) -> impl IntoView {
    let bg_url = post.map(|p| bg_url(p.uid));
    view! {
        <div class="absolute top-0 left-0 h-full w-full z-10 rounded-md overflow-clip">
            {move || {
                bg_url
                    .clone()
                    .map(|bgurl| {
                        view! { <img class="object-cover h-full w-full" src=bgurl.clone()/> }
                    })
            }}

        </div>
    }
}

#[component]
pub fn FallbackUser() -> impl IntoView {
    view! {
        <div
            class="flex flex-row gap-2 items-center p-2 animate-pulse"
            style:animation-delay="-500ms"
        >
            <div class="w-6 h-6 rounded-full bg-white/20"></div>
            <div class="w-20 h-1 rounded-full bg-white/20"></div>
        </div>
    }
}

#[component]
pub fn Speculation(details: BetDetails, _ref: NodeRef<html::Div>) -> impl IntoView {
    let (bet_res, amt, icon) = match details.outcome {
        BetOutcome::Won(amt) => (
            "RECEIVED",
            amt,
            view! {
                <div class="flex mt-2 w-full place-items-center place-content-center rounded-full bg-green-400 py-1 text-sm md:py-2">
                    Won
                </div>
            },
        ),
        BetOutcome::Draw(amt) => (
            "RECEIVED",
            amt,
            view! {
                <div class="flex mt-2 w-full place-items-center place-content-center rounded-full bg-yellow-400 py-1 text-sm md:py-2">
                    Draw
                </div>
            },
        ),
        BetOutcome::Lost => (
            "LOST",
            details.bet_amount,
            view! {
                <div class="flex mt-2 w-full place-items-center place-content-center rounded-full bg-red-400 py-1 text-sm md:py-2">
                    Lost
                </div>
            },
        ),
        BetOutcome::AwaitingResult => (
            "VOTED",
            details.bet_amount,
            view! {
                <div class="flex mt-2 w-full place-items-center place-content-center rounded-full bg-primary-500 py-1 text-sm md:py-2">
                    <Icon icon=icondata::FiClock/>
                </div>
            },
        ),
    };
    let profile_details = create_resource(
        move || details.canister_id,
        move |canister_id| async move {
            let canister = unauth_canisters();
            let user = canister.individual_user(canister_id).await.ok()?;
            let profile_details = user.get_profile_details().await.ok()?;
            Some(ProfileDetails::from(profile_details))
        },
    );
    let post_details = create_resource(
        move || (details.canister_id, details.post_id),
        move |(canister_id, post_id)| async move {
            let canister = unauth_canisters();
            let user = canister.individual_user(canister_id).await.ok()?;
            let post_details = user.get_individual_post_details_by_id(post_id).await.ok()?;
            Some(PostDetails::from_canister_post(
                false,
                canister_id,
                post_details,
            ))
        },
    );

    view! {
        <div _ref=_ref class="relative w-full basis-1/2 md:basis-1/3 lg:basis-1/4">
            <div class="relative flex flex-col justify-between aspect-[3/5] rounded-md m-2 text-white">
                <Suspense fallback=|| {
                    view! {
                        <div class="absolute top-0 left-0 h-full w-full z-10 bg-white/10 animate-pulse rounded-md"></div>
                    }
                }>
                    {move || {
                        post_details
                            .get()
                            .map(|post| {
                                view! { <ExternalPost post/> }
                            })
                    }}

                </Suspense>
                <Suspense fallback=FallbackUser>
                    {move || {
                        profile_details
                            .get()
                            .map(|user| {
                                view! { <ExternalUser user/> }
                            })
                    }}

                </Suspense>
                <div class="flex flex-col px-2 py-3 z-20">
                    <span class="text-xs font-thin uppercase">{bet_res}</span>
                    <span class="text-sm font-bold md:text-lg">{amt} Tokens</span>
                    {icon}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ProfileSpeculations(user_canister: Principal) -> impl IntoView {
    let provider = BetsProvider::new(unauth_canisters(), user_canister);

    view! {
        <ProfileStream
            provider
            children=|details, _ref| {
                view! { <Speculation details _ref=_ref.unwrap_or_default()/> }
            }
        />
    }
}
