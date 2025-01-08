use candid::Principal;
use codee::string::FromToStringCodec;
use leptos::{
    component, create_action, create_effect, create_rw_signal, create_signal, expect_context,
    html::Div, logging, provide_context, view, For, IntoView, NodeRef, Resource, RwSignal,
    ServerFnError, Show, Signal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate,
    SignalUpdateUntracked, Suspense, WriteSignal,
};
use leptos_icons::Icon;
use leptos_use::{use_cookie, use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use yral_canisters_common::Canisters;

use crate::{
    state::canisters::authenticated_canisters,
    utils::token::icpump::{get_paginated_token_list_with_limit, TokenListItem},
};

#[component]
fn Header() -> impl IntoView {
    let data = expect_context::<RwSignal<Option<PlayerGamesCountAndBalance>>>();
    view! {
        <div class="flex items-center w-full justify-between py-2 gap-8">
            <a
                href="/pump-dump/profile"
                class="flex flex-col text-right text-sm ml-8 relative bg-[#171717] rounded-lg pt-1 pb-1.5 pr-3 pl-8"
            >
                <div class="font-bold text-sm">
                    {move || data.get().map(|d| format!("{}", d.games_count)).unwrap_or_else(|| "----".into())}
                </div>
                <div class="text-xs text-[#A3A3A3] uppercase">Games</div>
                <img
                    src="/img/gamepad.png"
                    alt="Games"
                    class="absolute select-none -left-1/4 bottom-0 h-12 w-12 -rotate-1"
                />
            </a>
            <div
                class="flex flex-col text-left overf mr-8 relative bg-[#171717] rounded-lg pt-1 pb-1.5 pl-4 pr-8"
            >
                <div
                    class="font-bold absolute top-1 text-sm"
                >
                    {move || data.get().map(|d| format!("{}", d.wallet_balance)).unwrap_or_else(|| "----".into())}
                </div>
                <div class="h-5 opacity-0"></div>
                <div class="text-xs text-[#A3A3A3]">gDOLR</div>
                <img
                    src="/img/gdolr.png"
                    alt="gDOLR"
                    class="absolute select-none -right-1/4 bottom-1 size-9 -rotate-1"
                />
                <div class="absolute rounded-sm bg-[#212121] text-[#525252] p-0.5 size-5 -left-2 top-4">
                    <Icon class="size-full" icon=icondata::FiPlus />
                </div>
            </div>
        </div>
    }
}

#[component]
fn BullBearSlider() -> impl IntoView {
    let running_data: Resource<(), Option<GameRunningData>> = expect_context();
    let position = move || {
        let ratio = running_data
            .get()
            .flatten()
            .map(|d| (d.dumps as f64 + 1.0) / (d.pumps as f64 + 1.0))
            .unwrap_or(1f64);
        if ratio == 1f64 {
            39f64
        } else {
            78f64.min(0f64.max(ratio * 78f64 / (ratio + 1f64)))
        }
    };

    view! {
        <div class="py-5 w-full">
            <div
                style="background: linear-gradient(90deg, #3D8EFF 0%, #390059 51.5%, #E2017B 100%);"
                class="relative ring-4 ring-[#212121] rounded-full w-full h-2"
            >
                <div
                    class="flex absolute inset-0 transition-all duration-700 ease-in-out gap-1 items-center"
                    style:left=move || format!("{}%", position())
                >
                    <img
                        style="filter: drop-shadow( -3px 3px 2px rgba(0, 0, 0, .7));"
                        src="/img/bear.png"
                        alt="Bear"
                        class="h-6 push-right shake"
                    />
                    <img
                        style="filter: drop-shadow( 3px 3px 2px rgba(0, 0, 0, .7));"
                        src="/img/bull.png"
                        alt="Bull"
                        class="h-7 push-left shake"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn MockBullBearSlider() -> impl IntoView {
    view! {
        <div class="py-5 w-full">
            <div
                style="background: linear-gradient(90deg, #3D8EFF 0%, #390059 51.5%, #E2017B 100%);"
                class="relative ring-4 ring-[#212121] rounded-full w-full h-2"
            >
                <div
                    class="flex absolute inset-0 transition-all duration-700 ease-in-out gap-1 items-center"
                    style:left="39%"
                >
                    <img
                        style="filter: drop-shadow( -3px 3px 2px rgba(0, 0, 0, .7));"
                        src="/img/bear.png"
                        alt="Bear"
                        class="h-6 push-right shake"
                    />
                    <img
                        style="filter: drop-shadow( 3px 3px 2px rgba(0, 0, 0, .7));"
                        src="/img/bull.png"
                        alt="Bull"
                        class="h-7 push-left shake"
                    />
                </div>
            </div>
        </div>
    }
}

#[component]
fn DumpButton() -> impl IntoView {
    let running_data = expect_context::<Resource<(), Option<GameRunningData>>>();
    let player_data = expect_context::<RwSignal<Option<PlayerGamesCountAndBalance>>>();
    let counter = move || {
        running_data
            .get()
            .flatten()
            .map(|value| value.dumps.to_string())
            .unwrap_or_else(|| "-".into())
    };
    let onclick = move |_| {
        // TODO: write an async action to send event over ws, be optimisitic about the state
        // TODO: add debouncing
        player_data.update(|value| {
            if let Some(value) = value.as_mut() {
                value.wallet_balance -= 1;
            }
        });

        running_data.update(|value| {
            if let Some(Some(value)) = value {
                value.dumps += 1;
            }
        });

        // debounceResistanceAnimation();
    };

    view! {
        <button
            aria-label="Vibration"
            on:click=onclick
            class="dump-button rounded-[28px] transition-all duration-150 ring-4 group text-white ring-white/25 gap-2 min-w-36 p-3 flex flex-col items-center justify-center"
        >
            <div class="text-xl font-bold">DUMP</div>
            <div class="bg-[#4683DC] rounded-full w-12 h-3 relative">
                <div
                    class="w-full h-full relative overflow-hidden font-bold text-xs items-center flex justify-center"
                >
                        <span
                            class="absolute inset-0 flex items-center justify-center"
                        >
                            {counter}
                        </span>
                </div>
                <img
                    src="/img/skull.png"
                    class="absolute w-6 h-6 -left-3 -top-1/2 transition group-active:saturate-150 group-active:scale-110 group-active:rotate-12"
                    alt="DUMP"
                />
            </div>
        </button>
    }
}

#[component]
fn MockDumpButton() -> impl IntoView {
    view! {
        <button
            aria-label="Vibration"
            class="dump-button rounded-[28px] transition-all duration-150 ring-4 group text-white ring-white/25 gap-2 min-w-36 p-3 flex flex-col items-center justify-center"
        >
            <div class="text-xl font-bold">DUMP</div>
            <div class="bg-[#4683DC] rounded-full w-12 h-3 relative">
                <div
                    class="w-full h-full relative overflow-hidden font-bold text-xs items-center flex justify-center"
                >
                        <span
                            class="absolute inset-0 flex items-center justify-center"
                        >
                            0
                        </span>
                </div>
                <img
                    src="/img/skull.png"
                    class="absolute w-6 h-6 -left-3 -top-1/2 transition group-active:saturate-150 group-active:scale-110 group-active:rotate-12"
                    alt="DUMP"
                />
            </div>
        </button>
    }
}

#[component]
fn PumpButton() -> impl IntoView {
    let game_state = expect_context::<Resource<(), Option<GameRunningData>>>();
    let player_data = expect_context::<RwSignal<Option<PlayerGamesCountAndBalance>>>();
    let counter = move || {
        game_state
            .get()
            .flatten()
            .map(|value| value.pumps.to_string())
            .unwrap_or_else(|| "-".into())
    };
    let onclick = move |_| {
        // TODO: write an async action to send event over ws, be optimisitic about the state
        // TODO: add debouncing
        player_data.update(|value| {
            if let Some(value) = value.as_mut() {
                value.wallet_balance -= 1;
            }
        });

        game_state.update(|value| {
            if let Some(Some(value)) = value.as_mut() {
                value.pumps += 1;
            }
        });

        // debounceResistanceAnimation();
    };
    view! {
        <button
            aria-label="Vibration"
            on:click=onclick
            class="pump-button rounded-[28px] transition-all duration-150 ring-4 group text-white ring-white/25 gap-2 min-w-36 p-3 flex flex-col items-center justify-center"
        >
            <div class="text-xl font-bold">PUMP</div>
            <div class="bg-[#E2027B] rounded-full w-12 h-3 relative">
                <div
                    class="w-full h-full relative overflow-hidden font-bold text-xs items-center flex justify-center"
                >
                    <span
                        class="absolute inset-0 flex items-center justify-center"
                    >
                        {counter}
                    </span>
                </div>
                <img
                    src="/img/fire.png"
                    class="absolute w-6 h-6 -left-3 -top-1/2 transition group-active:saturate-150 group-active:scale-110 group-active:-rotate-12"
                    alt="PUMP"
                />
            </div>
        </button>
    }
}
#[component]
fn MockPumpButton() -> impl IntoView {
    view! {
        <button
            aria-label="Vibration"
            class="pump-button rounded-[28px] transition-all duration-150 ring-4 group text-white ring-white/25 gap-2 min-w-36 p-3 flex flex-col items-center justify-center"
        >
            <div class="text-xl font-bold">PUMP</div>
            <div class="bg-[#E2027B] rounded-full w-12 h-3 relative">
                <div
                    class="w-full h-full relative overflow-hidden font-bold text-xs items-center flex justify-center"
                >
                    <span
                        class="absolute inset-0 flex items-center justify-center"
                    >
                        0
                    </span>
                </div>
                <img
                    src="/img/fire.png"
                    class="absolute w-6 h-6 -left-3 -top-1/2 transition group-active:saturate-150 group-active:scale-110 group-active:-rotate-12"
                    alt="PUMP"
                />
            </div>
        </button>
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
struct PlayerGamesCountAndBalance {
    games_count: u64,
    wallet_balance: u64,
}

impl PlayerGamesCountAndBalance {
    pub fn new(games_count: u64, wallet_balance: u64) -> Self {
        Self {
            games_count,
            wallet_balance,
        }
    }

    #[cfg(any(feature = "local-bin", feature = "local-lib"))]
    pub async fn load(_user_principal: Principal) -> Option<Self> {
        Some(Self::new(0, 1000))
    }

    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    pub async fn load(_user_principal: Principal) -> Option<Self> {
        Some(Self::new(0, 1000))
    }
}

// this data is kept out of GameState so that mutating pumps and dumps doesn't
// cause the whole game card to rerender
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
struct GameRunningData {
    pumps: u64,
    dumps: u64,
    winning_pot: u64,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
enum GameState {
    Playing,
    Pending,
    ResultDeclared(GameResult),
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
enum GameResult {
    Win { amount: u64 },
    Loss,
}

impl GameState {
    pub fn new() -> Self {
        Self::Playing
    }

    #[cfg(any(feature = "local-bin", feature = "local-lib"))]
    pub async fn load() -> Self {
        Self::new()
    }

    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    pub async fn load() -> Self {
        Self::new()
    }
}

impl GameRunningData {
    pub fn new(pumps: u64, dumps: u64, winning_pot: u64) -> Self {
        Self {
            pumps,
            dumps,
            winning_pot,
        }
    }

    #[cfg(any(feature = "local-bin", feature = "local-lib"))]
    pub async fn load() -> Option<Self> {
        Some(Self::new(0, 0, 100))
    }

    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    pub async fn load() -> Option<Self> {
        Some(Self::new(0, 0, 100))
    }
}

#[component]
fn PendingResult() -> impl IntoView {
    let running_data: Resource<(), Option<GameRunningData>> = expect_context();
    let spent = move || {
        let data = running_data
            .get()
            .flatten()
            .expect("at this point we MUST have the running data");
        data.pumps.saturating_add(data.dumps)
    };
    view! {
        <div in:fade class="flex items-center gap-8 bg-[#212121] rounded-2xl py-6 px-6">
            <img src="/img/hourglass.png" alt="Hourglass" class="shrink-0 w-[4.25rem] rotate-12" />
            <div class="flex flex-col gap-1.5 flex-1">
                <div class="font-bold text-xl">The Tide has shifted!</div>
                <div class="text-sm text-[#A3A3A3]">
                    <span>{"You've used "}</span>
                    <span class="font-bold text-[#EC55A7]">{spent} gDOLR</span>
                    <div>{" — results are on the way. Stay tuned! ⏳"}</div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn GameCardPreResult(#[prop(into)] game_state: GameState) -> impl IntoView {
    let token: TokenListItem = expect_context();
    let show_onboarding: ShowOnboarding = expect_context();
    let running_data: Resource<(), Option<GameRunningData>> = expect_context();
    let winning_pot = move || {
        running_data
            .get()
            .flatten()
            .map(|value| value.winning_pot.to_string())
            .unwrap_or_else(|| "-".into())
    };
    view! {
        <div
            class="bg-[#171717] flip-card transition-all absolute inset-0 h-full shrink-0 rounded-2xl items-center flex flex-col gap-4 w-full pt-14 pb-5 px-5 overflow-hidden"
        >
            <img class="mt-14 w-24 h-24 rounded-[4px]" alt="Avatar" src=token.logo />
            <a href="#" class="flex items-center gap-1">
                <div class="font-bold text-lg">{token.token_name}</div>
            </a>
            <div class="bg-[#212121] shrink-0 rounded-full relative w-full h-11 overflow-hidden">
                <div
                    class="w-full slide-up top-[3.5rem] flex items-center gap-2 justify-between absolute inset-0 py-2 pl-4 pr-2"
                >
                    <div class="flex items-center gap-1">
                        <div class="text-[#A3A3A3] text-xs">Winning Pot:</div>
                        <img src="/img/gdolr.png" alt="Coin" class="size-5" />
                        <div class="text-[#E5E5E5] font-bold">{winning_pot} gDOLR</div>
                    </div>
                    <button
                        on:click=move |_| show_onboarding.show()
                        class="bg-black text-[#A3A3A3] hover:bg-black/35 rounded-full text-xl w-7 h-7 flex font-light items-center justify-center leading-none"
                    >
                        ?
                    </button>
                </div>
                <div
                    style="--animation-delay:5s;"
                    class="w-full top-[3.5rem] slide-up flex items-center gap-1 absolute inset-0 py-2 pl-4 pr-2"
                >
                    <img src="/img/player.png" alt="Coin" class="w-5 h-5" />
                    <div class="text-[#E5E5E5] font-bold">80</div>
                    <div class="text-[#A3A3A3] text-xs">players are playing - join the action!</div>
                </div>
            </div>
            <div class="flex select-none flex-col gap-4 h-[8.5rem] w-full">
                <Show
                    when=move || matches!(game_state, GameState::Playing)
                    fallback=move || view!{ <PendingResult /> }
                >
                    <BullBearSlider />
                    <div
                        class="flex relative items-center gap-6 justify-center w-full"
                    >
                        <DumpButton />

                        <PumpButton />
                    </div>
                </Show>
            </div>
        </div>
    }
}

#[component]
fn WonCard(#[prop()] result: GameResult) -> impl IntoView {
    let GameResult::Win { amount } = result else {
        unreachable!("Won card must only be shown in win condition")
    };
    // TODO: add confetti animation
    view! {
        <div
            style="background-size: cover; background-position: center; background-image: url('/img/pnd-onboarding-bg.png');"
            class="rounded-2xl overflow-hidden flip-card card-flipped absolute inset-0 h-full w-full shrink-0 items-center justify-center flex flex-col gap-7 pt-14 pb-5 px-5"
        >
            <img src="/img/trophy.png" alt="Trophy" class="w-32 h-[7.6rem] translate-x-3" />
            <div class="flex flex-col gap-4 items-center">
                <div class="font-semibold text-xl">Victory is yours!</div>
                <div class="text-[#A3A3A3] text-center">
                    {"Your strategy paid off! The tide shifted to your side, and you've won big. 💰"}
                </div>
                <div
                    class="bg-[#212121] w-full px-4 py-2 rounded-full flex items-center justify-center gap-2"
                >
                    <span class="text-[#A3A3A3] text-xs">You have won:</span>
                    <img src="/img/gdolr.png" alt="Coin" class="w-5 h-5" />
                    <span class="text-[#E5E5E5] font-bold">{amount} gDOLR</span>
                </div>
            </div>
            <button
                on:click=move |_| todo!("figure out what to do")
                class="w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold"
                style:background="linear-gradient(73deg, #DA539C 0%, #E2017B 33%, #5F0938 100%)"
            >Start playing again</button>
        </div>
    }
}

#[component]
fn LostCard() -> impl IntoView {
    view! {
        <div
            style="background: radial-gradient(100% 100% at -14% 74%, rgba(46, 124, 246, 0.16) 0%, rgba(23, 23, 23, 1) 100%);"
            class="rounded-2xl flip-card card-flipped absolute inset-0 h-full w-full shrink-0 items-center justify-center flex flex-col gap-7 pt-14 pb-5 px-5"
        >
            <img src="/img/sadface.png" alt="Sad face emoji" class="h-36 w-36" />
            <div class="flex flex-col gap-4 items-center">
                <div class="font-semibold text-xl">The Tide Turned Against You!</div>
                <div class="text-[#A3A3A3] text-center">
                    The other side took the lead this time, but every vote brings you closer to your next win.
                    Stay in the game!
                </div>
            </div>
            <button
                on:click=move |_| todo!("figure out what to do")
                class="w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold"
                style:background="linear-gradient(73deg, #DA539C 0%, #E2017B 33%, #5F0938 100%)"
            >Keep Playing</button>
        </div>
    }
}

#[component]
fn ResultDeclared(#[prop()] game_state: GameState) -> impl IntoView {
    match game_state {
        GameState::Playing | GameState::Pending => {
            unreachable!("This screen is not reachable until ResultDeclared state is reached")
        }
        GameState::ResultDeclared(result) => view! {
            <Show
                when=move || matches!(result, GameResult::Loss)
                fallback=move || view! { <WonCard result /> }
            >
                <LostCard />
            </Show>
        },
    }
}

#[component]
fn GameCard(#[prop()] token: TokenListItem) -> impl IntoView {
    let running_data = Resource::new(|| (), |_| GameRunningData::load());
    provide_context(running_data);
    let game_state = Resource::new(|| (), |_| GameState::load());
    provide_context(game_state);
    provide_context(token);

    create_effect(move |_| {
        let result = running_data.get().flatten().as_ref().and_then(|data| {
            if data.pumps >= 3 {
                Some(GameResult::Win { amount: 10 })
            } else if data.dumps >= 3 {
                Some(GameResult::Loss)
            } else {
                None
            }
        });

        if let Some(result) = result {
            game_state.update(|state| {
                if let Some(state) = state.as_mut() {
                    *state = GameState::Pending;
                }
            });

            wasm_bindgen_futures::spawn_local(async move {
                gloo::timers::future::TimeoutFuture::new(1000).await;

                game_state.update(|state| {
                    if let Some(state) = state.as_mut() {
                        *state = GameState::ResultDeclared(result);
                    }
                });
            })
        }
    });

    view! {
        <Suspense>
            {move || game_state.get().map(move |game_state| view! {
                <div
                    style="perspective: 500px; transition: transform 0.4s; transform-style: preserve-3d;"
                    class="relative w-full min-h-[31rem] snap-start snap-always"
                >
                    <Show
                        when=move || { matches!(game_state, GameState::Playing | GameState::Pending)}
                        fallback=move || view! { <ResultDeclared game_state /> }
                    >
                        <GameCardPreResult game_state />
                    </Show>
                </div>
            })}
        </Suspense>
    }
}

#[derive(Copy, Clone, Debug)]
struct ShowOnboarding(Signal<Option<bool>>, WriteSignal<Option<bool>>);

impl ShowOnboarding {
    #[inline]
    fn show(&self) {
        self.1.set(Some(true));
    }

    #[inline]
    fn hide(&self) {
        self.1.set(Some(false));
    }

    #[inline]
    fn should_show(&self) -> bool {
        self.0.get().unwrap_or(true)
    }
}

#[component]
fn OnboardingPopup() -> impl IntoView {
    let (step, set_step) = create_signal(0);
    let show_onboarding = expect_context::<ShowOnboarding>();
    view! {
        <div class="fade-in fixed inset-0 bg-black/50 flex py-16 justify-center z-50 p-4">
            <div
                style="background-size: cover; background-position: left; background-image: url('/img/pnd-onboarding-bg.png');"
                class="rounded-2xl max-w-md flex flex-col h-[33.5rem] justify-center text-white gap-8 items-center pt-8 pb-5 px-8 relative"
            >
                <div
                    class="absolute flex items-center top-4 px-4 inset-x-0"
                    class=("justify-end", move || step.get() == 0)
                    class=("justify-between", move || step.get() == 1)
                >
                {move || (step.get() == 1).then(|| view! {
                    <button on:click=move |_| set_step.set(0) class="text-[#525252]">
                        <Icon class="size-5" icon=icondata::FiChevronLeft />
                    </button>
                })}
                    <button
                        on:click=move |_| show_onboarding.hide()
                        class="p-1 flex items-center justify-center bg-[#525252] rounded-full"
                    >
                        <Icon class="size-3" icon=icondata::IoClose />
                    </button>
                </div>
                {move || if step.get() == 0 {
                    view! {
                        <img src="/img/pumpndump.png" alt="Logo" class="h-32 pt-8" />
                        <div class="flex flex-col gap-5 items-center">
                            <div class="font-bold text-xl">Shape the Future of Tokens!</div>
                            <div class="text-sm text-center">
                                Your vote decides the fate of the tokens. Ride the waves of Pump and Dump and vote to
                                make the tides shift to snatch up with reward pool.
                            </div>
                            <div class="flex gap-0.5 text-sm">
                                <img src="/img/gdolr.png" alt="Coin" class="w-5 h-5" />
                                <div>1 gDOLR = 1 vote</div>
                            </div>
                        </div>
                        <div class="flex w-full justify-end pt-20 items-center gap-1">
                            <button
                                on:click=move |_| set_step.set(1)
                                class="appearance-none text-xl font-semibold">Next</button
                            >
                            <Icon class="size-3" icon=icondata::FiChevronRight />
                        </div>
                    }
                } else {
                    view! {
                        <div class="flex flex-col text-sm gap-5 items-center text-center">
                            <div class="font-bold text-xl">How it works?</div>
                            <div class="flex gap-2 justify-between items-center">
                                <div class="flex-1 text-xs text-left">
                                    <div class="text-white">Step 1</div>
                                    <div class="text-[#A3A3A3]">
                                        Vote for the Tide - Pump or Dump. Predict the next shift in momentum.
                                    </div>
                                </div>
                                <div class="flex-1 relative py-12">
                                    <div class="scale-[0.6] h-full w-full">
                                        <div class="absolute bottom-0 -left-8 z-[2]">
                                            <MockPumpButton />
                                        </div>
                                        <div class="absolute -top-2 -right-6 z-[1]">
                                            <MockDumpButton />
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class="flex flex-row-reverse gap-2 justify-between items-center">
                                <div class="flex-1 text-xs text-right">
                                    <div class="text-white">Step 2</div>
                                    <div class="text-[#A3A3A3]">
                                        The battle for dominance begins here, keep voting as each vote influences the tide
                                    </div>
                                </div>
                                <div class="flex-1 relative py-6">
                                    <div class="scale-[0.8] -translate-x-3 h-full w-full">
                                        <MockBullBearSlider />
                                    </div>
                                </div>
                            </div>
                            <div class="flex gap-2 justify-between items-center">
                                <div class="flex-1 text-xs text-left">
                                    <div class="text-white">Step 3</div>
                                    <div class="text-[#A3A3A3]">
                                        Claim your rewards when the tide turns and overtakes the majority.
                                    </div>
                                </div>
                                <div class="flex-1 flex items-center justify-center relative py-6 pl-8">
                                    <img src="/img/trophy.png" alt="Trophy" class="h-20 w-[5.5rem]" />
                                </div>
                            </div>
                        </div>
                        <button
                            on:click=move |_| show_onboarding.hide()
                            class="w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold"
                            style:background="linear-gradient(73deg, #DA539C 0%, #E2017B 33%, #5F0938 100%)"
                        >Ok, got it!</button>
                    }
                }}
            </div>
        </div>
    }
}

#[component]
pub fn PumpNDump() -> impl IntoView {
    let player_games_count_and_balance = create_rw_signal(None::<PlayerGamesCountAndBalance>);
    let cans_wire_res = authenticated_canisters();
    let fetch_user_principal = create_action(move |&()| {
        let cans_wire_res = cans_wire_res.clone();
        async move {
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|e| e.to_string())?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|_| "Unable to authenticate".to_string())?;

            let data = PlayerGamesCountAndBalance::load(cans.user_principal())
                .await
                .ok_or(|| ()) // ignore
                .map_err(|_| "Couldn't load player data".to_string())?;

            player_games_count_and_balance.set(Some(data));

            Ok::<(), String>(())
        }
    });

    create_effect(move |_| {
        if player_games_count_and_balance.get_untracked().is_none() {
            fetch_user_principal.dispatch(());
        }
    });

    provide_context(player_games_count_and_balance);

    let (should_show, set_should_show) = use_cookie::<bool, FromToStringCodec>("show_onboarding");
    let show_onboarding = ShowOnboarding(should_show, set_should_show);
    provide_context(show_onboarding);

    let tokens = create_rw_signal(Vec::<TokenListItem>::new());
    let page = create_rw_signal(1u32);
    let scroll_container = NodeRef::<Div>::new();
    let fetch_more_tokens = create_action(move |&page: &u32| async move {
        let limit = match page {
            1..5 => 1,
            _ => 5,
        };
        let more_tokens = get_paginated_token_list_with_limit(page, limit)
            .await
            .expect("TODO: handle error");
        tokens.update(|tokens| {
            tokens.extend_from_slice(&more_tokens);
        });
    });
    let _ = use_infinite_scroll_with_options(
        scroll_container,
        move |_| async move {
            page.update_untracked(|v| {
                *v += 1;
            });

            fetch_more_tokens.dispatch(page.get_untracked());
        },
        UseInfiniteScrollOptions::default().distance(400f64),
    );
    view! {
        <div class="h-screen w-screen block text-white bg-black">
            <div class="max-w-md flex flex-col relative w-full mx-auto items-center h-full px-4 py-4">
                <Header />
                <div node_ref=scroll_container class="size-full overflow-scroll flex flex-col gap-4 snap-mandatory snap-y pb-[50vh]">
                    <For each=move || tokens.get() key=|item| item.token_name.clone() let:token>
                        <GameCard token />
                    </For>
                </div>
            </div>
            <Show when=move || show_onboarding.should_show()>
                <OnboardingPopup />
            </Show>
        </div>
    }
}
