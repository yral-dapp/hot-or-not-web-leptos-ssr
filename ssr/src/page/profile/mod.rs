mod ic;
pub mod overlay;
mod posts;
mod profile_iter;
pub mod profile_post;
mod speculation;
mod tokens;

use candid::Principal;
use leptos::{either::Either, prelude::*};
use leptos_icons::*;
use leptos_router::{components::Redirect, hooks::use_params, params::Params};

use crate::{
    component::connect::ConnectLogin,
    state::{auth::account_connected_reader, canisters::authenticated_canisters},
    utils::{send_wrap, MockPartialEq},
};

use posts::ProfilePosts;
use speculation::ProfileSpeculations;
use tokens::ProfileTokens;
use yral_canisters_common::{
    utils::{posts::PostDetails, profile::ProfileDetails},
    Canisters,
};

#[derive(Clone, Default)]
pub struct ProfilePostsContext {
    video_queue: RwSignal<Vec<PostDetails>>,
    start_index: RwSignal<usize>,
    current_index: RwSignal<usize>,
    queue_end: RwSignal<bool>,
}

#[derive(Params, PartialEq)]
struct ProfileParams {
    id: String,
}

#[component]
fn Stat(stat: u64, #[prop(into)] info: String) -> impl IntoView {
    view! {
        <div class="flex flex-1 flex-col items-center text-white space-y-0.5">
            <span class="font-bold text-xl">{stat}</span>
            <span class="text-md">{info}</span>
        </div>
    }
}

#[derive(Params, Clone, PartialEq)]
struct TabsParam {
    tab: String,
}

#[component]
fn ListSwitcher1(user_canister: Principal, user_principal: Principal) -> impl IntoView {
    let param = use_params::<TabsParam>();

    let current_tab = Memo::new(move |_| {
        param.with(|p| {
            let tab = p.as_ref().map(|p| p.tab.as_str()).unwrap_or("tokens");
            match tab {
                "posts" => 0,
                "stakes" => 1,
                "tokens" => 2,
                _ => 0,
            }
        })
    });

    let tab_class = move |tab_id: usize| {
        if tab_id == current_tab() {
            "text-primary-500 border-b-4 border-primary-500 flex justify-center w-full py-2"
        } else {
            "text-white flex justify-center w-full py-2"
        }
    };
    view! {
        <div class="relative flex flex-row w-11/12 md:w-9/12 text-center text-xl md:text-2xl">
            <a class=move || tab_class(0) href=move || format!("/profile/{}/posts", user_principal)>
                <Icon icon=icondata::FiGrid />
            </a>
            <a
                class=move || tab_class(1)
                href=move || format!("/profile/{}/stakes", user_principal)
            >
                <Icon icon=icondata::BsTrophy />
            </a>
            <a
                class=move || tab_class(2)
                href=move || format!("/profile/{}/tokens", user_principal)
            >
                <Icon icon=icondata::AiDollarCircleOutlined />
            </a>
        </div>

        <div class="flex flex-col gap-y-12 justify-center pb-12 w-11/12 sm:w-7/12">
            <Show when=move || current_tab() == 0>
                <ProfilePosts user_canister />
            </Show>
            <Show when=move || current_tab() == 1>
                <ProfileSpeculations user_canister />
            </Show>
            <Show when=move || current_tab() == 2>
                <ProfileTokens user_canister user_principal />
            </Show>
        </div>
    }
}

#[component]
fn ProfileViewInner(user: ProfileDetails, user_canister: Principal) -> impl IntoView {
    let username_or_principal = user.username_or_principal();
    let profile_pic = user.profile_pic_or_random();
    let display_name = user.display_name_or_fallback();
    let earnings = user.lifetime_earnings;
    let (is_connected, _) = account_connected_reader();

    view! {
        <div class="min-h-screen bg-black text-white overflow-y-scroll pt-10 pb-12">
            <div class="grid grid-cols-1 gap-5 justify-normal justify-items-center w-full">
                <div class="flex flex-row w-11/12 sm:w-7/12 justify-center">
                    <div class="flex flex-col justify-center items-center">
                        <img
                            class="h-24 w-24 rounded-full"
                            alt=username_or_principal.clone()
                            src=profile_pic
                        />
                        <div class="flex flex-col text-center items-center">
                            <span
                                class="text-md text-white font-bold"
                                class=("w-full", is_connected)
                                class=("w-5/12", move || !is_connected())
                                class=("truncate", move || !is_connected())
                            >
                                {display_name}
                            </span>
                            <div class="text-sm flex flex-row">
                                // TODO: Add username when it's available
                                // <p class="text-white">@ {username_or_principal}</p>
                                <p class="text-primary-500">{earnings} Earnings</p>
                            </div>
                            <Show when=move || !is_connected()>
                                <div class="md:w-4/12 w-6/12 pt-5">
                                    <ConnectLogin cta_location="profile" />
                                </div>
                            </Show>
                        </div>
                    </div>
                </div>
                <div class="flex justify-around text-center rounded-full divide-x-2 divide-white/20 bg-white/10 p-4 my-4 w-11/12 sm:w-7/12">
                    // <Stat stat=user.followers_cnt info="Lovers"/>
                    // <Stat stat=user.following_cnt info="Loving"/>
                    <Stat stat=user.hots info="Hots" />
                    <Stat stat=user.nots info="Nots" />
                </div>
                <ListSwitcher1 user_canister user_principal=user.principal />
            </div>
        </div>
    }
}
#[component]
pub fn ProfileView() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    let tab_params = use_params::<TabsParam>();

    let param_principal = move || {
        params.with(|p| {
            let ProfileParams { id, .. } = p.as_ref().ok()?;
            Principal::from_text(id).ok()
        })
    };

    let auth_cans = authenticated_canisters();
    let profile_info_res = Resource::new(
        move || {
            (
                auth_cans.get().map(|_| MockPartialEq(())),
                param_principal(),
            )
        },
        move |(_, principal)| {
            send_wrap(async move {
                let cans_wire = auth_cans.await?;
                let user_principal = cans_wire.profile_details.principal;

                let Some(principal) = principal else {
                    return Ok((None, Some(user_principal)));
                };

                let canisters = Canisters::from_wire(cans_wire, expect_context())?;
                if user_principal == principal {
                    let details = canisters.profile_details();
                    let user_canister = canisters.user_canister();
                    return Ok((Some((details, user_canister)), None));
                }

                let Some(user_canister) = canisters
                    .get_individual_canister_by_user_principal(principal)
                    .await?
                else {
                    return Err(ServerFnError::new("Failed to get user canister"));
                };

                let user = canisters.individual_user(user_canister).await;
                let user_details = user.get_profile_details().await?;
                Ok((Some((user_details.into(), user_canister)), None))
            })
        },
    );

    view! {
        <Suspense>
            {move || Suspend::new(async move {
                let res = profile_info_res.await;
                match res {
                    Ok((None, Some(user_principal))) => {
                        if let Ok(TabsParam { tab }) = tab_params() {
                            Either::Left(view! {
                                <Redirect path=format!(
                                    "/profile/{}/{}",
                                    user_principal,
                                    tab,
                                ) />
                            })
                        } else {
                            Either::Left(view! { <Redirect path="/" /> })
                        }
                    }
                    Err(_) => Either::Left(view! { <Redirect path="/" /> }),
                    Ok((Some((user_details, user_canister)), None)) => {
                        Either::Right(view! {
                            <ProfileComponent user_details=Some((
                                user_details,
                                user_canister,
                            )) />
                        })
                    }
                    _ => Either::Left(view! { <Redirect path="/" /> }),
                }
            })}
        </Suspense>
    }
}

#[component]
pub fn ProfileComponent(user_details: Option<(ProfileDetails, Principal)>) -> impl IntoView {
    let ProfilePostsContext {
        video_queue,
        start_index,
        ..
    } = expect_context();

    video_queue.update_untracked(|v| {
        v.drain(..);
    });
    start_index.update_untracked(|idx| {
        *idx = 0;
    });

    view! {
        {move || {
            if let Some((user, user_canister)) = user_details.clone() {
                Either::Left(view! { <ProfileViewInner user user_canister /> })
            } else {
                Either::Right(view! { <Redirect path="/" /> })
            }
        }}
    }
}
