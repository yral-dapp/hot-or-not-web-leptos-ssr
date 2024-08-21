use candid::Principal;
use leptos::*;
use leptos_icons::*;

use crate::{
    canister::individual_user_template::{BettingStatus, PlaceBetArg, Result1},
    component::{
        bullet_loader::BulletLoader, canisters_prov::AuthCansProvider, hn_icons::*,
        spinner::SpinnerFit,
    },
    state::canisters::{unauth_canisters, Canisters},
    try_or_redirect_opt,
    utils::{
        posts::PostDetails,
        profile::{BetDetails, BetKind, BetOutcome},
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
    let user = canisters.authenticated_user().await?;

    let place_bet_arg = PlaceBetArg {
        bet_amount,
        post_id,
        bet_direction: bet_direction.into(),
        post_canister_id,
    };

    let res = user.bet_on_currently_viewing_post(place_bet_arg).await?;

    let betting_status = match res {
        Result1::Ok(p) => p,
        Result1::Err(_e) => {
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
    #[prop(into, default = "h-14 w-14".into())] class: String,
) -> impl IntoView {
    view! {
        <div class=class>
        {move || match coin() {
            CoinState::C50 => view! { <C50Coin/> },
            CoinState::C100 => view! { <C100Coin/> },
            CoinState::C200 => view! { <C200Coin/> },
        }}
        </div>
    }
}

#[component]
fn HotButton(
    #[prop(into)] bet_direction: WriteSignal<Option<BetKind>>,
    #[prop(into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <button class="h-14 w-14 md:h-16 md:w-16 lg:h-18 md:w-18" disabled=disabled on:click=move |_| bet_direction.set(Some(BetKind::Hot))>
            <Show when=move || !disabled() fallback=SpinnerFit>
                <HotIcon/>
            </Show>
        </button>
    }
}

#[component]
fn NotButton(
    #[prop(into)] bet_direction: WriteSignal<Option<BetKind>>,
    #[prop(into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <button class="h-14 w-14 md:h-16 md:w-16 lg:h-18 md:w-18" disabled=disabled on:click=move |_| bet_direction.set(Some(BetKind::Not))>
            <Show when=move || !disabled() fallback=SpinnerFit>
                <NotIcon/>
            </Show>
        </button>
    }
}

#[component]
fn HNButtonOverlay(
    post: PostDetails,
    coin: RwSignal<CoinState>,
    bet_direction: RwSignal<Option<BetKind>>,
    tried_to_place_bet: Trigger,
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
    let set_bet_direction = bet_direction.write_only();
    let place_bet_res = place_bet_action.value();
    create_effect(move |_| {
        if place_bet_res().flatten().is_some() {
            tried_to_place_bet.notify();
        }
    });
    let running = place_bet_action.pending();

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
        <div class="flex w-full justify-center">
            <Icon
                class="text-2xl justify-self-end text-white hover:cursor-pointer"
                icon=icondata::AiUpOutlined
                on:click=move |ev| {
                    ev.stop_propagation();
                    coin.update(|c| *c = c.wrapping_next());
                }
            />
        </div>
        <div class="flex flex-row w-full items-center justify-center gap-6">
            <HotButton disabled=running bet_direction=set_bet_direction />
            <button on:click=move |ev| {
                ev.stop_propagation();
                coin.update(|c| *c = c.wrapping_next())
            }>
                <CoinStateView class="w-12 h-12" coin />
            </button>
            <NotButton disabled=running bet_direction=set_bet_direction />
        </div>
        // Bottom row: Hot <down arrow> Not
        // most of the CSS is for alignment with above icons
        <div class="flex w-full justify-center items-center gap-6 text-base md:text-lg lg:text-xl text-center font-medium pt-2">
            <p class="w-14">Hot</p>
            <div class="flex justify-center w-12">
                <Icon
                    class="text-2xl text-white hover:cursor-pointer"
                    icon=icondata::AiDownOutlined
                    on:click=move |ev| {
                        ev.stop_propagation();
                        coin.update(|c| *c = c.wrapping_prev());
                    }
                />
            </div>
            <p class="w-14">Not</p>
        </div>
    }
}

#[component]
fn WinBadge() -> impl IntoView {
    view! {
        // <!-- Win Badge as a full-width button -->
        <button class="mt-2 w-full rounded-sm bg-pink-500 px-4 py-2 text-sm font-bold text-white">
            <div class="flex justify-center items-center">
                <span class="">
                    <Icon
                        class="fill-white"
                        style=""
                        icon=icondata::RiTrophyFinanceFill
                    />
                </span>
                <span class="ml-2">"You Won"</span>
            </div>
        </button>
    }
}

#[component]
fn LostBadge() -> impl IntoView {
    view! {
        <button class="mt-2 w-full rounded-sm bg-white px-4 py-2 text-sm font-bold text-black">
            <Icon class="fill-white" style="" icon=icondata::RiTrophyFinanceFill />
            "You Lost"
        </button>
    }
}

#[component]
fn HNWonLost(participation: BetDetails) -> impl IntoView {
    let won = move || matches!(participation.outcome, BetOutcome::Won(_));
    let bet_amount = move || participation.bet_amount;
    let coin = Signal::derive(move || match bet_amount() {
        50 => CoinState::C50,
        100 => CoinState::C100,
        200 => CoinState::C200,
        amt => {
            log::warn!("Invalid bet amount: {amt}, using fallback");
            CoinState::C50
        }
    });
    let is_hot = move || matches!(participation.bet_kind, BetKind::Hot);

    view! {
        <div class="flex w-auto items-center rounded-xl bg-transparent p-4 shadow-sm backdrop-blur-sm">
            <div class="relative flex-shrink-0">
                <CoinStateView class="w-20 h-20" coin/>
                <div class="absolute -bottom-1 -right-2 flex items-center justify-center rounded-full h-9 w-9">
                    <Show when=is_hot fallback=NotIcon>
                        <HotIcon/>
                    </Show>
                </div>
            </div>

            // <!-- Text and Badge Column -->
            <div class="ml-4 flex flex-grow flex-col">
                // <!-- Result Text -->
                <div class="text-sm leading-snug text-white bg-black/15 rounded-full p-1">
                    <p>You staked placed_bet_detail.value tokens on Hot.</p>
                    <p>You received placed_bet_detail.reward tokens.</p>
                </div>

                <Show when=won fallback=LostBadge>
                    <WinBadge/>
                </Show>
            </div>

        </div>
    }
}

#[component]
fn HNAwaitingResults(participation: BetDetails) -> impl IntoView {
    let is_hot = move || matches!(participation.bet_kind, BetKind::Hot);
    let bet_direction_text = move || if is_hot() { "Hot" } else { "Not" };

    let bet_amount = move || participation.bet_amount;
    let coin = Signal::derive(move || match bet_amount() {
        50 => CoinState::C50,
        100 => CoinState::C100,
        200 => CoinState::C200,
        amt => {
            log::warn!("Invalid bet amount: {amt}, using fallback");
            CoinState::C50
        }
    });
    let time_remaining = move || {
        let (hh, mm, ss) = participation.time_remaining_hms();
        format!("{hh:02}:{mm:02}:{ss:02}")
    };

    view! {
        <div class="flex w-auto items-center rounded-xl bg-transparent p-4 shadow-sm backdrop-blur-sm">
            <div class="relative flex-shrink-0">
                <CoinStateView class="w-20 h-20" coin/>
                <div class="absolute -bottom-1 -right-2 flex items-center justify-center rounded-full h-9 w-9">
                    <Show when=is_hot fallback=NotIcon>
                        <HotIcon/>
                    </Show>
                </div>
            </div>

            // timer component
            <div class="flex flex-col gap-1 ps-2 w-fit">
                <p class="font-semibold text-white bg-black/25 rounded-full text-sm p-1 ps-2">{time_remaining}</p>
                <p class="text-center text-white bg-black/15 rounded-full p-1 ps-2">
                    You staked {bet_amount} tokens on {bet_direction_text}
                    Result is still pending
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn HNUserParticipation(participation: BetDetails) -> impl IntoView {
    view! {
        {match participation.outcome {
            BetOutcome::AwaitingResult => {
                view! { <HNAwaitingResults participation /> }
            }
            BetOutcome::Won(_) => {
                view! { <HNWonLost participation /> }
            }
            BetOutcome::Draw(_) => view! { "Draw" }.into_view(),
            BetOutcome::Lost => {
                view! { <HNWonLost participation /> }
            }
        }.into_view()}
    }
}

#[component]
fn MaybeHNButtons(
    post: PostDetails,
    bet_direction: RwSignal<Option<BetKind>>,
    coin: RwSignal<CoinState>,
    tried_to_place_bet: Trigger,
) -> impl IntoView {
    let post = store_value(post);
    let is_betting_enabled = create_resource(
        move || (),
        move |_| {
            let post = post.get_value();
            async move {
                let canisters = unauth_canisters();
                let user = canisters.individual_user(post.canister_id).await.ok()?;
                let res = user
                    .get_hot_or_not_bet_details_for_this_post(post.post_id)
                    .await
                    .ok()?;
                Some(matches!(res, BettingStatus::BettingOpen { .. }))
            }
        },
    );

    view! {
        <Suspense fallback=BulletLoader>
        {move || is_betting_enabled().and_then(|enabled| {
            if !enabled.unwrap_or_default() {
                return None;
            }
            Some(view! {
                <HNButtonOverlay post=post.get_value() bet_direction coin tried_to_place_bet/>
            })
        })}
        </Suspense>
    }
}

#[component]
pub fn HNGameOverlay(post: PostDetails) -> impl IntoView {
    let bet_direction = create_rw_signal(None::<BetKind>);
    let coin = create_rw_signal(CoinState::C50);

    let tried_to_place_bet = create_trigger();
    let post = store_value(post);

    let create_bet_participation_outcome = move |canisters: Canisters<true>| {
        // TODO: leptos 0.7, switch to `create_resource`
        create_local_resource(
            // MockPartialEq is necessary
            // See: https://github.com/leptos-rs/leptos/issues/2661
            move || {
                tried_to_place_bet.track();
                MockPartialEq(())
            },
            move |_| {
                let cans = canisters.clone();
                async move {
                    let post = post.get_value();
                    let user = cans.authenticated_user().await?;
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
        <AuthCansProvider fallback=BulletLoader let:canisters>
        {
            let bet_participation_outcome = create_bet_participation_outcome(canisters);
            view! {
                {move || bet_participation_outcome().and_then(|res| {
                    let participation = try_or_redirect_opt!(res);
                    Some(if let Some(participation) = participation {
                        view! {
                            <HNUserParticipation participation/>
                        }
                    } else {
                        view! {
                            <MaybeHNButtons
                                post=post.get_value()
                                bet_direction coin
                                tried_to_place_bet
                            />
                        }
                    })
                }).unwrap_or_else(|| view! { <BulletLoader/> })}
            }
        }
        </AuthCansProvider>
    }
}
