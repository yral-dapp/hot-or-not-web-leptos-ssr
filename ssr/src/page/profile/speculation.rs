use candid::Principal;
use leptos::{either::EitherOf4, html, prelude::*};
use leptos_icons::*;
use leptos_router::hooks::use_location;
use leptos_use::use_interval_fn;
use web_time::Duration;

use super::ic::ProfileStream;
use crate::{
    component::{canisters_prov::AuthCansProvider, profile_placeholders::NoMoreBetsGraphic},
    state::canisters::unauth_canisters,
    utils::{bg_url, send_wrap, time::to_hh_mm_ss},
};
use yral_canisters_common::{
    cursored_data::vote::VotesProvider,
    utils::{
        posts::PostDetails,
        profile::ProfileDetails,
        vote::{VoteDetails, VoteOutcome},
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
        <div class="flex flex-row items-center gap-1 w-full h-8 px-3 pt-3 text-ellipsis z-20">
            <div class="w-5 h-5 flex-shrink-0 rounded-full border-2 border-white bg-white">
                <img class="rounded-full object-cover object-center" src=propic />
            </div>
            <div class="max-w-full text-xs truncate font-semibold">{name}</div>
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
                        view! { <img class="object-cover h-full w-full" src=bgurl.clone() /> }
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
fn BetTimer(post: PostDetails, details: VoteDetails) -> impl IntoView {
    let bet_duration = details.vote_duration().as_secs();
    let time_remaining = RwSignal::new(details.time_remaining(post.created_at));
    _ = use_interval_fn(
        move || {
            time_remaining.try_update(|t| *t = t.saturating_sub(Duration::from_secs(1)));
        },
        1000,
    );

    let percentage = Memo::new(move |_| {
        let remaining_secs = time_remaining().as_secs();
        100 - ((remaining_secs * 100) / bet_duration).min(100)
    });
    let gradient = move || {
        let perc = percentage();
        format!("background: linear-gradient(to right, rgb(var(--color-primary-600)) {perc}%, #00000020 0 {}%);", 100 - perc)
    };

    view! {
        <div class="flex items-start w-full h-6 px-3 bg-transparent">
            <div
                class="flex flex-row justify-center items-center gap-1 w-full rounded-full py-px text-white text-xs"
                style=gradient
            >
                <Icon icon=icondata::AiClockCircleFilled />
                <span>{move || to_hh_mm_ss(time_remaining())}</span>
            </div>
        </div>
    }
}

#[component]
pub fn Speculation(details: VoteDetails, _ref: NodeRef<html::Div>) -> impl IntoView {
    // TODO: enable scrolling videos for bets
    let profile_post_url = format!("/post/{}/{}", details.canister_id, details.post_id);

    let bet_canister = details.canister_id;

    let post_details = Resource::new(
        move || (bet_canister, details.post_id),
        move |(canister_id, post_id)| {
            send_wrap(async move {
                let canister = unauth_canisters();
                let user = canister.individual_user(canister_id).await;
                let post_details = user.get_individual_post_details_by_id(post_id).await.ok()?;
                Some(PostDetails::from_canister_post(
                    false,
                    canister_id,
                    post_details,
                ))
            })
        },
    );

    let profile_details = Resource::new(
        move || bet_canister,
        move |canister_id| {
            send_wrap(async move {
                let canister = unauth_canisters();
                let user = canister.individual_user(canister_id).await;
                let profile_details = user.get_profile_details().await.ok()?;
                Some(ProfileDetails::from(profile_details))
            })
        },
    );

    let details = StoredValue::new(details);
    let (bet_res, amt, icon) = match details.with_value(|d| d.outcome) {
        VoteOutcome::Won(amt) => (
            "RECEIVED",
            amt,
            EitherOf4::A(view! {
                <div class="flex w-full justify-center items-center text-white bg-primary-600 h-6 gap-0.5">
                    <Icon class="text-sm fill-white" icon=icondata::RiTrophyFinanceFill />
                    <span class="text-xs font-medium">You Won</span>
                </div>
            }),
        ),
        VoteOutcome::Draw(amt) => (
            "RECEIVED",
            amt,
            EitherOf4::B(view! {
                <div class="flex w-full justify-center items-center bg-yellow-500 text-xs font-medium text-white h-6">
                    Draw
                </div>
            }),
        ),
        VoteOutcome::Lost => (
            "VOTE",
            details.with_value(|d| d.vote_amount),
            EitherOf4::C(view! {
                <div class="flex w-full justify-center items-center h-6 bg-white text-black py-2 text-xs font-medium">
                    You Lost
                </div>
            }),
        ),
        VoteOutcome::AwaitingResult => (
            "VOTE",
            details.with_value(|d| d.vote_amount),
            EitherOf4::D(view! {
                <Suspense>
                    {move || {
                        let post = post_details.get().flatten()?;
                        Some(view! { <BetTimer post details=details.get_value() /> })
                    }}

                </Suspense>
            }),
        ),
    };

    view! {
        <div node_ref=_ref class="relative w-1/2 md:w-1/3 lg:w-1/4 px-1">
            <a
                href=profile_post_url
                class="relative flex flex-col justify-between aspect-[3/5] rounded-md text-white"
            >
                <Suspense fallback=|| {
                    view! {
                        <div class="absolute top-0 left-0 h-full w-full z-10 bg-white/10 animate-pulse rounded-md"></div>
                    }
                }>
                    {move || {
                        post_details
                            .get()
                            .map(|post| {
                                view! { <ExternalPost post /> }
                            })
                    }}

                </Suspense>
                <Suspense fallback=FallbackUser>
                    {move || {
                        profile_details
                            .get()
                            .map(|user| {
                                view! { <ExternalUser user /> }
                            })
                    }}

                </Suspense>
                <div class="flex flex-col gap-y-5 z-20">
                    <div class="flex flex-col px-3">
                        <span class="text-xs font-medium uppercase">{bet_res}</span>
                        <span class="text-sm font-semibold md:text-base">{amt}Tokens</span>
                    </div>
                    {icon}
                </div>
            </a>
        </div>
    }
}

#[component]
pub fn ProfileSpeculations(user_canister: Principal) -> impl IntoView {
    view! {
        <AuthCansProvider let:canister>
            {
                let provider = VotesProvider::new(unauth_canisters(), user_canister);
                let location = use_location();
                let empty_text = if location
                    .pathname
                    .get_untracked()
                    .starts_with(&format!("/profile/{}", canister.user_principal()))
                {
                    "You haven't placed any votes yet!"
                } else {
                    "Not played any games yet!"
                };
                view! {
                    <ProfileStream
                        provider
                        empty_graphic=NoMoreBetsGraphic
                        empty_text
                        children=move |details, _ref| {
                            view! { <Speculation details _ref=_ref.unwrap_or_default() /> }
                        }
                    />
                }
            }
        </AuthCansProvider>
    }
}
