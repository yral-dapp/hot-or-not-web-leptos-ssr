mod ic;
pub mod overlay;
mod posts;
mod profile_iter;
pub mod profile_post;
mod speculation;
mod tokens;

use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

use crate::{
    component::{
        canisters_prov::AuthCansProvider, connect::ConnectLogin, spinner::FullScreenSpinner,
    },
    state::{auth::account_connected_reader, canisters::unauth_canisters},
    utils::{posts::PostDetails, profile::ProfileDetails},
};

use posts::ProfilePosts;
use speculation::ProfileSpeculations;
use tokens::ProfileTokens;

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
#[component]
fn ListSwitcher1() -> impl IntoView{
    let loc = use_location();
    let navigate = use_navigate();
    let current_tab = create_memo(move |_|{
        let pathname = loc.pathname.get();
        if pathname.ends_with("posts"){
           return 0
        }
        else if pathname.ends_with("stakes"){
            return 1
        }
        else if pathname.ends_with("tokens"){
            return 2
        }
        else{
            return    0
        }
    });

    let tab_class = move |tab_id: usize| {
        if tab_id == current_tab() {
            "text-primary-500 border-b-4 border-primary-500 flex justify-center w-full py-2"
        } else {
            "text-white flex justify-center w-full py-2"
        }
    };

    view!{
        <div class="relative flex flex-row w-11/12 md:w-9/12 text-center text-xl md:text-2xl">
        <button
            class=move || tab_class(0)
            on:click={
                let navigate = navigate.clone();
                let loc = loc.clone();
                move |_| {
                    let p = loc.pathname.get();
                    if let Some(last_slash_index) = p.rfind('/') {
                        // Replace everything after the last '/' with the replacement string, excluding the '/'
                        navigate(&format!("{}{}", &p.as_str()[..last_slash_index + 1], "posts"), Default::default())
                    } else {
                        // If no slash is found, return the original string
                        navigate(p.as_str(), Default::default())
                    }
                }
            }
        >
            <Icon icon=icondata::FiGrid />
        </button>
        <button
            class=move || tab_class(1)
            on:click={
                let navigate = navigate.clone();
                let loc = loc.clone();
                move |_| {
                    if let Some(last_slash_index) = loc.pathname.get().rfind('/') {
                        // Replace everything after the last '/' with the replacement string, excluding the '/'
                        navigate(&format!("{}{}", &loc.pathname.get().as_str()[..last_slash_index + 1], "stakes"), Default::default())
                    } else {
                        // If no slash is found, return the original string
                        navigate(loc.pathname.get().as_str(), Default::default())
                    }
                }
            }
        >
            <Icon icon=icondata::BsTrophy />
        </button>
        <button
            class=move || tab_class(2)
            on:click={
                let navigate = navigate.clone();
                let loc = loc.clone();
                move |_| {
                    if let Some(last_slash_index) = loc.pathname.get().rfind('/') {
                        // Replace everything after the last '/' with the replacement string, excluding the '/'
                        navigate(&format!("{}{}", &loc.pathname.get().as_str()[..last_slash_index + 1], "tokens"), Default::default())
                    } else {
                        // If no slash is found, return the original string
                        navigate(loc.pathname.get().as_str(), Default::default())
                    }
                }
            }
        >
            <Icon icon=icondata::AiDollarCircleOutlined />
        </button>
    </div>

    <div class="flex flex-col gap-y-12 justify-center pb-12 w-11/12 sm:w-7/12">
    <Show when=move || current_tab() == 0>
        <ProfilePostsRoute />
    </Show>
    <Show when=move || current_tab() == 1>
        <ProfileStakesRoute />
    </Show>
    <Show when=move || current_tab() == 2>
        <ProfileTokenRoute />
    </Show>
</div>
    }

}
#[component]
pub fn ProfilePostsRoute() -> impl IntoView{
    let params = use_params::<ProfileParams>();
    let param_principal = create_memo(move |_| {
        params.with(|p| {
            let ProfileParams { id } = p.as_ref().ok()?;
            Principal::from_text(id).ok()
        })
    });
    
    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canister>
        {
            if let Some(param_principal) = param_principal.get() {
                view! {
                    <Suspense>
                    {move || {
                        view! { <ProfilePosts user_canister=param_principal /> }
                    }}
                </Suspense>
                }
            } else {
                let user_canister_principal = canister.user_canister();
                view! {
                    <ProfilePosts user_canister=user_canister_principal />
                }
            }
        }
        </AuthCansProvider>
    }
}

#[component]
pub fn ProfileStakesRoute() -> impl IntoView{
    let params = use_params::<ProfileParams>();
    let param_principal = create_memo(move |_| {
        params.with(|p| {
            let ProfileParams { id } = p.as_ref().ok()?;
            Principal::from_text(id).ok()
        })
    });
    
    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canister>
        {
            if let Some(param_principal) = param_principal.get() {
                view! {
                    <Suspense>
                    {move || {
                        view! { <ProfileSpeculations user_canister=param_principal /> }
                    }}
                </Suspense>
                }
            } else {
                let user_canister_principal = canister.user_canister();
                view! {
                    <ProfileSpeculations user_canister=user_canister_principal />
                }
            }
        }
        </AuthCansProvider>
    }
}
#[component]
pub fn ProfileTokenRoute() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    let param_principal = create_memo(move |_| {
        params.with(|p| {
            let ProfileParams { id } = p.as_ref().ok()?;
            Principal::from_text(id).ok()
        })
    });

    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canister>
        {
            if let Some(param_principal) = param_principal.get() {
                let user_details = create_resource(
                    move || Some(param_principal),
                    move |param_principal| async move {
                        let param_principal = param_principal?;
                        let canisters = unauth_canisters();

                        let user_canister = canisters
                            .get_individual_canister_by_user_principal(param_principal)
                            .await
                            .ok()??;
                        Some((user_canister, param_principal))
                    },
                );

                view! {
                    <Suspense>
                        {move || {
                            user_details.get().map(|maybe| {
                                view! { <ProfileTokens user_canister=maybe.clone().unwrap().0 user_principal=maybe.clone().unwrap().1 /> }
                            })
                        }}
                    </Suspense>
                }
            } else {
                let user_canister_principal = canister.user_canister();
                let user_principal = canister.user_principal();
                view! {
                    <ProfileTokens user_canister=user_canister_principal user_principal=user_principal />
                }
            }
        }
        </AuthCansProvider>
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
                                <p class="text-primary-500">{earnings}Earnings</p>
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
                <ListSwitcher1/>
            </div>
        </div>
    }
}


#[component]
pub fn ProfileView() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    let param_principal = create_memo(move |_| {
        params.with(|p| {
            let ProfileParams { id } = p.as_ref().ok()?;
            Principal::from_text(id).ok()
        })
    });

    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canister>
            {
                if let Some(param_principal) = param_principal.get() {
                    let user_canister_principal = canister.user_canister();
                    if param_principal == user_canister_principal {
                        view! { <YourProfileView /> }
                    } else {
                        let user_details = create_resource(
                            move || Some(param_principal),
                            move |param_principal| async move {
                                let param_principal = param_principal?;
                                let canisters = unauth_canisters();

                                let user_canister = canisters
                                    .get_individual_canister_by_user_principal(param_principal)
                                    .await
                                    .ok()??;
                                let user = canisters.individual_user(user_canister).await;
                                let user_details = user.get_profile_details().await.ok()?;
                                Some((user_details.into(), user_canister))
                            },
                        );

                        view! {
                            <Suspense>
                                {move || {
                                    user_details.get().map(|user_details| {
                                        view! { <ProfileComponent user_details /> }
                                    })
                                }}
                            </Suspense>
                        }
                    }
                } else {
                    view! {
                        <YourProfileView />
                    }
                }
            }
        </AuthCansProvider>
    }
}
#[component]
pub fn YourProfileView() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=FullScreenSpinner let:canister>
            <ProfileComponent user_details=Some((
                canister.profile_details(),
                canister.user_canister(),
            )) />
        </AuthCansProvider>
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
                view! { <ProfileViewInner user user_canister /> }
            } else {
                view! { <Redirect path="/" /> }
            }
        }}
    }
}
