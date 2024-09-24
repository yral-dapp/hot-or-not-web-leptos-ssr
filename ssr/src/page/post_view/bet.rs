use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_use::use_interval_fn;
use web_time::Duration;

use crate::{
    canister::individual_user_template::{BettingStatus, PlaceBetArg, Result3},
    component::{
        bullet_loader::BulletLoader, canisters_prov::AuthCansProvider, hn_icons::*,
        spinner::SpinnerFit,
    },
    page::post_view::BetEligiblePostCtx,
    state::canisters::{unauth_canisters, Canisters},
    try_or_redirect_opt,
    utils::{
        posts::PostDetails,
        profile::{BetDetails, BetKind, BetOutcome},
        time::to_hh_mm_ss,
        MockPartialEq,
    },
};

#[derive(Clone, Copy, Debug, PartialEq)]
enum CoinState {
    C50,
    C100,
    C200,
}

impl CoinState {
    fn wrapping_next(self) -> Self {
        match self {
            CoinState::C50 => CoinState::C100,
            CoinState::C100 => CoinState::C200,
            CoinState::C200 => CoinState::C50,
        }
    }

    fn wrapping_prev(self) -> Self {
        match self {
            CoinState::C50 => CoinState::C200,
            CoinState::C100 => CoinState::C50,
            CoinState::C200 => CoinState::C100,
        }
    }
}

impl From<CoinState> for u64 {
    fn from(coin: CoinState) -> u64 {
        match coin {
            CoinState::C50 => 50,
            CoinState::C100 => 100,
            CoinState::C200 => 200,
        }
    }
}

async fn bet_on_post(
    canisters: Canisters<true>,
    bet_amount: u64,
    bet_direction: BetKind,
    post_id: u64,
    post_canister_id: Principal,
) -> Result<BettingStatus, ServerFnError> {
    let user = canisters.authenticated_user().await;

    let place_bet_arg = PlaceBetArg {
        bet_amount,
        post_id,
        bet_direction: bet_direction.into(),
        post_canister_id,
    };

    let res = user.bet_on_currently_viewing_post(place_bet_arg).await?;

    let betting_status = match res {
        Result3::Ok(p) => p,
        Result3::Err(_e) => {
            // todo send event that betting failed
            return Err(ServerFnError::new(
                "bet on bet_on_currently_viewing_post error".to_string(),
            ));
        }
    };

    Ok(betting_status)
}

#[component]
fn CoinStateView(
    #[prop(into)] coin: MaybeSignal<CoinState>,
    #[prop(into)] class: String,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let icon = Signal::derive(move || match coin() {
        CoinState::C50 => C50Icon,
        CoinState::C100 => C100Icon,
        CoinState::C200 => C200Icon,
    });

    view! {
        <div class:grayscale=disabled>
            <Icon class=class icon/>
        </div>
    }
}

#[component]
fn HNButton(
    bet_direction: RwSignal<Option<BetKind>>,
    kind: BetKind,
    #[prop(into)] disabled: Signal<bool>,
) -> impl IntoView {
    let grayscale = create_memo(move |_| bet_direction() != Some(kind) && disabled());
    let show_spinner = move || disabled() && bet_direction() == Some(kind);
    let icon = if kind == BetKind::Hot {
        HotIcon
    } else {
        DislikeIcon
    };

    view! {
        <button
           class="w-14 h-14 md:w-16 md:h-16 md:w-18 lg:h-18"
            class=("grayscale", grayscale)
            disabled=disabled
            on:click=move |_| bet_direction.set(Some(kind))
        >
            <Show when=move || !show_spinner() fallback=SpinnerFit>
                <Icon class="w-full h-full drop-shadow-lg" icon=icon/>
            </Show>
        </button>
    }
}

#[component]
fn HNButtonOverlay(
    post: PostDetails,
    coin: RwSignal<CoinState>,
    bet_direction: RwSignal<Option<BetKind>>,
    refetch_bet: Trigger,
) -> impl IntoView {
    let place_bet_action = create_action(
        move |(canisters, bet_direction, bet_amount): &(Canisters<true>, BetKind, u64)| {
            let post_can_id = post.canister_id;
            let post_id = post.post_id;
            let cans = canisters.clone();
            let bet_amount = *bet_amount;
            let bet_direction = *bet_direction;
            async move {
                match bet_on_post(cans, bet_amount, bet_direction, post_id, post_can_id).await {
                    Ok(_) => Some(()),
                    Err(e) => {
                        log::error!("{e}");
                        None
                    }
                }
            }
        },
    );
    let place_bet_res = place_bet_action.value();
    create_effect(move |_| {
        if place_bet_res().flatten().is_some() {
            refetch_bet.notify();
        }
    });
    let running = place_bet_action.pending();

    let BetEligiblePostCtx { can_place_bet } = expect_context();

    create_effect(move |_| {
        if !running.get() {
            can_place_bet.set(true)
        } else {
            can_place_bet.set(false)
        }
    });

    view! {
        <AuthCansProvider let:canisters>

            {
                create_effect(move |_| {
                    let Some(bet_direction) = bet_direction() else {
                        return;
                    };
                    let bet_amount = coin.get_untracked().into();
                    place_bet_action.dispatch((canisters.clone(), bet_direction, bet_amount));
                });
            }

        </AuthCansProvider>
        <div class="flex relative justify-center w-full touch-manipulation"  >
            <button
                disabled=running
                on:click=move |_| coin.update(|c| *c =  c.wrapping_next())
            >
                <Icon
                    class="justify-self-end text-2xl text-white"
                    icon=icondata::AiUpOutlined
                />
            </button>
        </div>
        <div class="flex flex-row gap-6 justify-center items-center w-full touch-manipulation">
            <HNButton disabled=running bet_direction kind=BetKind::Hot  />
            <button disabled=running on:click=move |_| coin.update(|c| *c = c.wrapping_next())>
                <CoinStateView disabled=running class="w-12 h-12 md:w-14 md:h-14 lg:w-16 lg:h-16 drop-shadow-lg" coin />

            </button>
            <HNButton disabled=running bet_direction kind=BetKind::Not/>
        </div>
        // Bottom row: Hot <down arrow> Not
        // most of the CSS is for alignment with above icons
        <div class="flex gap-6 justify-center items-center pt-2 mb-2 w-full text-base font-medium text-center md:text-lg lg:text-xl touch-manipulation">
            <p class="pb-4 w-14 md:w-16 lg:w-18">Hot</p>
            <div class="flex relative bottom-4 justify-center w-12 md:w-14 lg:w-16" style="bottom: 4px;"  >
                <button
                    disabled=running
                    on:click=move |_| coin.update(|c| *c = c.wrapping_prev())
                >
                    <Icon
                        class="mb-5 text-2xl text-white"
                        icon=icondata::AiDownOutlined
                    />
                </button>
            </div>
            <p class="pb-4 w-14 md:w-16 lg:w-18">Not</p>
        </div>
        <ShadowBg/>
    }
}

#[component]
fn WinBadge() -> impl IntoView {
    view! {
        // <!-- Win Badge as a full-width button -->
        <button class="py-2 px-4 w-full text-sm font-bold text-white rounded-sm bg-primary-600">
            <div class="flex justify-center items-center">
                <span class="">
                    <Icon class="fill-white" style="" icon=icondata::RiTrophyFinanceFill/>
                </span>
                <span class="ml-2">"You Won"</span>
            </div>
        </button>
    }
}

#[component]
fn LostBadge() -> impl IntoView {
    view! {
        <button class="py-2 px-4 w-full text-sm font-bold text-black bg-white rounded-sm">
            <Icon class="fill-white" style="" icon=icondata::RiTrophyFinanceFill />
          "You Lost"
        </button>
    }
}

#[component]
fn HNWonLost(participation: BetDetails) -> impl IntoView {
    let won = matches!(participation.outcome, BetOutcome::Won(_));
    let bet_amount = participation.bet_amount;
    let coin = match bet_amount {
        50 => CoinState::C50,
        100 => CoinState::C100,
        200 => CoinState::C200,
        amt => {
            log::warn!("Invalid bet amount: {amt}, using fallback");
            CoinState::C50
        }
    };
    let is_hot = matches!(participation.bet_kind, BetKind::Hot);
    let hn_icon = if is_hot { HotIcon } else { DislikeIcon };

    view! {
        <div class="flex gap-6 justify-center items-center p-4 w-full bg-transparent rounded-xl shadow-sm">
            <div class="relative flex-shrink-0 drop-shadow-lg">
                <CoinStateView class="w-14 h-14 md:w-16 md:h-16" coin/>
                <Icon class="absolute -bottom-0.5 -right-3 w-7 h-7 md:w-9 md:h-9" icon=hn_icon />
         </div>

            // <!-- Text and Badge Column -->
            <div class="flex flex-col gap-2 w-full md:w-1/2 lg:w-1/3">
                // <!-- Result Text -->
                <div class="p-1 text-sm leading-snug text-white rounded-full">
                    <p>You staked {bet_amount} tokens on {if is_hot { "Hot" } else { "Not" }}.</p>
                    <p>{if let Some(reward) = participation.reward() {
                        format!("You received {reward} tokens.")
                    } else {
                        format!("You lost {bet_amount} tokens.")
                    }}</p>
            </div>
                {if won {
                    view! { <WinBadge/> }
                } else {
                    view! { <LostBadge/> }
                }}

            </div>

        </div>
    }
}

#[component]
fn BetTimer(post: PostDetails, participation: BetDetails, refetch_bet: Trigger) -> impl IntoView {
    let bet_duration = participation.bet_duration().as_secs();
    let time_remaining = create_rw_signal(participation.time_remaining(post.created_at));
    _ = use_interval_fn(
        move || {
            time_remaining.try_update(|t| *t = t.saturating_sub(Duration::from_secs(1)));
            _ = refetch_bet;
            // if time_remaining.try_get_untracked() == Some(Duration::ZERO) {
            //     refetch_bet.notify();
            // }
        },
        1000,
    );

    let percentage = create_memo(move |_| {
        let remaining_secs = time_remaining().as_secs();
        100 - ((remaining_secs * 100) / bet_duration).min(100)
    });
    let gradient = move || {
        let perc = percentage();
        format!("background: linear-gradient(to right, rgb(var(--color-primary-600)) {perc}%, #00000020 0 {}%);", 100 - perc)
    };

    view! {
        <div class="flex flex-row gap-1 justify-end items-center py-px w-full text-base text-white rounded-full md:text-lg pe-4" style=gradient>
       <Icon icon=icondata::AiClockCircleFilled/>
            <span>{move || to_hh_mm_ss(time_remaining())}</span>
        </div>
    }
}

#[component]
fn HNAwaitingResults(
    post: PostDetails,
    participation: BetDetails,
    refetch_bet: Trigger,
) -> impl IntoView {
    let is_hot = matches!(participation.bet_kind, BetKind::Hot);
    let bet_direction_text = if is_hot { "Hot" } else { "Not" };
    let hn_icon = if is_hot { HotIcon } else { NotIcon };

    let bet_amount = participation.bet_amount;
    let coin = match bet_amount {
        50 => CoinState::C50,
        100 => CoinState::C100,
        200 => CoinState::C200,
        amt => {
            log::warn!("Invalid bet amount: {amt}, using fallback");
            CoinState::C50
        }
    };

    view! {
        <div class="flex flex-col gap-1 items-center p-4 w-full shadow-sm">
            <div class="flex flex-row gap-4 justify-center items-end w-full">
                <div class="relative flex-shrink-0 drop-shadow-lg">
                    <Icon class="w-12 h-12 md:w-14 md:h-14 lg:w-16 lg:h-16" icon=hn_icon/>
                    <CoinStateView class="absolute bottom-0 -right-3 w-7 h-7 md:w-9 md:h-9 lg:w-11 lg:h-11" coin/>
                </div>

                <div class="w-1/2 md:w-1/3 lg:w-1/4">
                    <BetTimer post refetch_bet participation/>
                </div>
            </div>
            <p class="p-1 text-center text-white rounded-full bg-black/15 ps-2">
                You staked {bet_amount} tokens on {bet_direction_text}
                Result is still pending
                You staked {bet_amount} tokens on {bet_direction_text}.
                Result is still pending.
            </p>
        </div>
    }
}

#[component]
pub fn HNUserParticipation(
    post: PostDetails,
    participation: BetDetails,
    refetch_bet: Trigger,
) -> impl IntoView {
    view! {
        {match participation.outcome {
            BetOutcome::AwaitingResult => {
                view! { <HNAwaitingResults post refetch_bet participation/> }
            }
            BetOutcome::Won(_) => {
                view! { <HNWonLost participation/> }
            }
            BetOutcome::Draw(_) => view! { "Draw" }.into_view(),
            BetOutcome::Lost => {
                view! { <HNWonLost participation/> }
            }
        }
            .into_view()}
        <ShadowBg/>
    }
}

#[component]
fn MaybeHNButtons(
    post: PostDetails,
    bet_direction: RwSignal<Option<BetKind>>,
    coin: RwSignal<CoinState>,
    refetch_bet: Trigger,
) -> impl IntoView {
    let post = store_value(post);
    let is_betting_enabled: Resource<(), Option<bool>> = create_resource(
        move || (),
        move |_| {
            let post = post.get_value();
            async move {
                let canisters = unauth_canisters();
                let user = canisters.individual_user(post.canister_id).await;
                let res = user
                    .get_hot_or_not_bet_details_for_this_post(post.post_id)
                    .await
                    .ok()?;
                Some(matches!(res, BettingStatus::BettingOpen { .. }))
            }
        },
    );
    let BetEligiblePostCtx { can_place_bet } = expect_context();

    view! {
        <Suspense fallback=LoaderWithShadowBg>
            {move || {
                is_betting_enabled()
                    .and_then(|enabled| {
                        if !enabled.unwrap_or_default() {
                            can_place_bet.set(false);
                            return None;
                        }
                        Some(
                            view! {
                                <HNButtonOverlay
                                    post=post.get_value()
                                    bet_direction
                                    coin
                                    refetch_bet
                                />
                            },
                        )
                    })
            }}

        </Suspense>
    }
}

#[component]
fn LoaderWithShadowBg() -> impl IntoView {
    view! {
        <BulletLoader/>
        <ShadowBg/>
    }
}

#[component]
fn ShadowBg() -> impl IntoView {
    view! {
        <div
            class="absolute bottom-0 left-0 h-2/5 w-dvw -z-[1]"
            style="background: linear-gradient(to bottom, #00000000 0%, #00000099 45%, #000000a8 100%, #000000cc 100%, #000000a8 100%);"
        ></div>
    }
}

#[component]
pub fn HNGameOverlay(post: PostDetails) -> impl IntoView {
    let bet_direction = create_rw_signal(None::<BetKind>);
    let coin = create_rw_signal(CoinState::C50);

    let refetch_bet = create_trigger();
    let post = store_value(post);

    let create_bet_participation_outcome = move |canisters: Canisters<true>| {
        // TODO: leptos 0.7, switch to `create_resource`
        create_local_resource(
            // MockPartialEq is necessary
            // See: https://github.com/leptos-rs/leptos/issues/2661
            move || {
                refetch_bet.track();
                MockPartialEq(())
            },
            move |_| {
                let cans = canisters.clone();
                async move {
                    let post = post.get_value();
                    let user = cans.authenticated_user().await;
                    let bet_participation = user
                        .get_individual_hot_or_not_bet_placed_by_this_profile(
                            post.canister_id,
                            post.post_id,
                        )
                        .await?;
                    Ok::<_, ServerFnError>(bet_participation.map(BetDetails::from))
                }
            },
        )
    };

    view! {
        <AuthCansProvider fallback=LoaderWithShadowBg let:canisters>

            {
                let bet_participation_outcome = create_bet_participation_outcome(canisters);
                view! {
                    {move || {
                        bet_participation_outcome()
                            .and_then(|res| {
                                let participation = try_or_redirect_opt!(res);
                                let post = post.get_value();
                                Some(
                                    if let Some(participation) = participation {
                                        view! {
                                            <HNUserParticipation post refetch_bet participation/>
                                        }
                                    } else {
                                        view! {
                                            <MaybeHNButtons post bet_direction coin refetch_bet/>
                                        }
                                    },
                                )
                            })
                            .unwrap_or_else(|| view! { <LoaderWithShadowBg/> })
                    }}
                }
            }

        </AuthCansProvider>
    }
}
