use candid::{Nat, Principal};
use codee::string::{FromToStringCodec, JsonSerdeCodec};
use leptos::{
    component, create_action, create_effect, create_rw_signal, create_signal, expect_context,
    html::Div, logging, provide_context, view, For, IntoView, NodeRef, RwSignal, Show, Signal,
    SignalGet, SignalGetUntracked, SignalSet, SignalUpdate, SignalUpdateUntracked, Suspense,
    WriteSignal,
};
use leptos_icons::Icon;
use leptos_use::{
    use_cookie, use_infinite_scroll_with_options, use_websocket, UseInfiniteScrollOptions,
    UseWebSocketReturn,
};
use once_cell::sync::Lazy;
use reqwest::Url;
use std::rc::Rc;
use yral_canisters_common::Canisters;
use yral_pump_n_dump_common::{
    rest::UserBetsResponse,
    ws::{websocket_connection_url, WsError, WsMessage, WsRequest, WsResp},
    GameDirection,
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct WsResponse {
    pub request_id: uuid::Uuid,
    pub response: WsResp,
}

use crate::{
    page::icpump::{process_token_list_item, ProcessedTokenListResponse},
    state::canisters::authenticated_canisters,
    utils::token::icpump::get_paginated_token_list_with_limit,
};

#[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
static PUMP_AND_DUMP_WORKER_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://yral-pump-n-dump.tushar-23b.workers.dev/").unwrap());

#[cfg(any(feature = "local-bin", feature = "local-lib"))]
static PUMP_AND_DUMP_WORKER_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("http://localhost:8787/").unwrap());

type GameRunningDataSignal = RwSignal<Option<GameRunningData>>;
type PlayerDataSignal = RwSignal<Option<PlayerData>>;
type GameStateSignal = RwSignal<Option<GameState>>;

type Sendfn = Rc<dyn Fn(&WsRequest)>;

// based on https://leptos-use.rs/network/use_websocket.html#usage-with-provide_context
#[derive(Clone)]
pub struct WebsocketContext {
    pub message: Signal<Option<WsResponse>>,
    sendfn: Sendfn, // use Arc to make it easily cloneable
}

impl WebsocketContext {
    pub fn new(message: Signal<Option<WsResponse>>, send: Sendfn) -> Self {
        Self {
            message,
            sendfn: send,
        }
    }

    // create a method to avoid having to use parantheses around the field
    #[inline(always)]
    pub fn send(&self, message: &WsRequest) {
        (self.sendfn)(message);
    }
}

#[component]
fn Header() -> impl IntoView {
    let data = expect_context::<RwSignal<Option<PlayerData>>>();
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
                    {move || data.get().map(|d| d.wallet_balance.to_string().replace("_", "")).unwrap_or_else(|| "----".into())}
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
    let running_data: GameRunningDataSignal = expect_context();
    let position = move || {
        let ratio = running_data
            .get()
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
    let running_data: GameRunningDataSignal = expect_context();
    let player_data = expect_context::<PlayerDataSignal>();
    let websocket: RwSignal<Option<WebsocketContext>> = expect_context();
    let counter = move || {
        running_data
            .get()
            .map(|value| value.dumps.to_string())
            .unwrap_or_else(|| "-".into())
    };
    let onclick = move |_| {
        if let Some(websocket) = websocket.get().as_ref() {
            logging::log!("can has websocket");
            websocket.send(&WsRequest {
                request_id: uuid::Uuid::new_v4(),
                msg: WsMessage::Bet(GameDirection::Dump),
            });

            player_data.update(|value| {
                if let Some(value) = value.as_mut() {
                    value.wallet_balance -= 1;
                }
            });

            running_data.update(|value| {
                if let Some(value) = value {
                    value.dumps += 1;
                }
            });
        }

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
    let running_data: GameRunningDataSignal = expect_context();
    let player_data = expect_context::<RwSignal<Option<PlayerData>>>();
    let websocket: RwSignal<Option<WebsocketContext>> = expect_context();
    let counter = move || {
        running_data
            .get()
            .map(|value| value.pumps.to_string())
            .unwrap_or_else(|| "-".into())
    };
    let onclick = move |_| {
        // TODO: add debouncing
        if let Some(websocket) = websocket.get().as_ref() {
            logging::log!("can has websocket");
            websocket.send(&WsRequest {
                request_id: uuid::Uuid::new_v4(),
                msg: WsMessage::Bet(GameDirection::Pump),
            });

            player_data.update(|value| {
                if let Some(value) = value.as_mut() {
                    value.wallet_balance -= 1;
                }
            });

            running_data.update(|value| {
                if let Some(value) = value.as_mut() {
                    value.pumps += 1;
                }
            });
        }

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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct PlayerData {
    games_count: u64,
    wallet_balance: u128,
}

impl PlayerData {
    pub fn new(games_count: u64, wallet_balance: u128) -> Self {
        Self {
            games_count,
            wallet_balance,
        }
    }

    pub async fn load(user_principal: Principal) -> Result<Self, String> {
        let balance_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/balance/{user_principal}"))
            .expect("Url to be valid");
        let games_count_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/game_count/{user_principal}"))
            .expect("Url to be valid");

        let games_count: u64 = reqwest::get(games_count_url)
            .await
            .map_err(|_| "Failed to load games count")?
            .text()
            .await
            .map_err(|_| "failed to read response body".to_string())?
            .parse()
            .map_err(|_| "Couldn't parse nat number".to_string())?;

        let wallet_balance: Nat = reqwest::get(balance_url)
            .await
            .map_err(|_| "failed to load balance".to_string())?
            .text()
            .await
            .map_err(|_| "failed to read response body".to_string())?
            .parse()
            .map_err(|_| "Couldn't parse nat number".to_string())?;

        let wallet_balance = convert_e8s_to_gdolr(wallet_balance);

        Ok(Self::new(games_count, wallet_balance))
    }
}

/// Convert e8s to gdolr
/// backend returns dolr in e8s, and 1dolr = 100gdolr
fn convert_e8s_to_gdolr(num: Nat) -> u128 {
    (num * 100u64 / 10u64.pow(8))
        .0
        .try_into()
        .expect("gdolr, scoped at individual player, to be small enough to fit in a u128")
}

// this data is kept out of GameState so that mutating pumps and dumps doesn't
// cause the whole game card to rerender
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
struct GameRunningData {
    pumps: u64,
    dumps: u64,
    winning_pot: Option<u64>,
    player_count: u64,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
enum GameState {
    Playing,
    Pending,
    ResultDeclared(GameResult),
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
enum GameResult {
    Win { amount: u128 },
    Loss,
}

impl GameState {
    pub async fn load(
        owner_principal: Principal,
        root_principal: Principal,
    ) -> Result<Self, String> {
        // hit /status/:owner_principal/:root_principal
        // if ready => Playing
        // otherwise => Pending

        let status_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/status/{owner_principal}/{root_principal}"))
            .expect("url to be valid");

        let status_res = reqwest::get(status_url)
            .await
            .map_err(|err| format!("Couldn't get status for the game: {err}"))?
            .text()
            .await
            .map_err(|err| format!("couldn't read response body: {err}"))?;

        if status_res == "ready" {
            return Ok(Self::Playing);
        }

        logging::warn!("game status response: {status_res}");

        Ok(Self::Pending)
    }
}

impl GameRunningData {
    pub fn new(pumps: u64, dumps: u64, player_count: u64, winning_pot: Option<u64>) -> Self {
        Self {
            pumps,
            dumps,
            player_count,
            winning_pot,
        }
    }

    // #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    pub async fn load(
        owner: Principal,
        token_root: Principal,
        user_canister: Principal,
    ) -> Result<Self, String> {
        let bets_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/bets/{owner}/{token_root}/{user_canister}"))
            .expect("url to be valid");

        let player_count_url = PUMP_AND_DUMP_WORKER_URL
            .join(&format!("/player_count/{owner}/{token_root}"))
            .expect("url to be valid");

        let bets: UserBetsResponse = reqwest::get(bets_url)
            .await
            .map_err(|err| format!("Coulnd't load bets: {err}"))?
            .json()
            .await
            .map_err(|err| format!("Couldn't parse bets out of repsonse: {err}"))?;

        let player_count: u64 = reqwest::get(player_count_url)
            .await
            .map_err(|err| format!("Coulnd't load player count: {err}"))?
            .text()
            .await
            .map_err(|err| format!("Couldn't read response for player count: {err}"))?
            .parse()
            .map_err(|err| format!("Couldn't parse player count from response: {err}"))?;

        // Maybe we should also load winning pot as part of game running data
        Ok(Self::new(bets.pumps, bets.dumps, player_count, None))
    }
}

#[component]
fn PendingResult() -> impl IntoView {
    let running_data: GameRunningDataSignal = expect_context();
    let spent = move || {
        let data = running_data
            .get()
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
    let token: ProcessedTokenListResponse = expect_context();
    let show_onboarding: ShowOnboarding = expect_context();
    let running_data: GameRunningDataSignal = expect_context();
    // let running_data: RwSignal<Option<GameRunningData>> = expect_context();
    let winning_pot = move || {
        running_data
            .get()
            .and_then(|data| data.winning_pot)
            .map(|value| value.to_string())
            .unwrap_or_else(|| "--".into())
    };

    let player_count = move || {
        running_data
            .get()
            .map(|data| data.player_count)
            .map(|value| value.to_string())
            .unwrap_or_else(|| "--".into())
    };
    view! {
        <div
            class="bg-[#171717] flip-card transition-all absolute inset-0 h-full shrink-0 rounded-2xl items-center flex flex-col gap-4 w-full pt-14 pb-5 px-5 overflow-hidden"
        >
            <img class="mt-14 w-24 h-24 rounded-[4px]" alt="Avatar" src=token.token_details.logo />
            <a href="#" class="flex items-center gap-1">
                <div class="font-bold text-lg">{token.token_details.token_name}</div>
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
                    <div class="text-[#E5E5E5] font-bold">{player_count}</div>
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

    let game_state: GameStateSignal = expect_context();
    let running_data: GameRunningDataSignal = expect_context();

    let on_click = move |_| {
        game_state.update(|s| *s = Some(GameState::Playing));
        // player count of zero doesn't make sense
        // dispatch another load call after this update
        running_data.update(|s| *s = Some(GameRunningData::new(0, 0, 0, None)));
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
                on:click=on_click
                class="w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold"
                style:background="linear-gradient(73deg, #DA539C 0%, #E2017B 33%, #5F0938 100%)"
            >Start playing again</button>
        </div>
    }
}

#[component]
fn LostCard() -> impl IntoView {
    let game_state: GameStateSignal = expect_context();
    let running_data: GameRunningDataSignal = expect_context();

    let on_click = move |_| {
        game_state.update(|s| *s = Some(GameState::Playing));
        // dispatch another load call after this update
        running_data.update(|s| *s = Some(GameRunningData::new(0, 0, 0, None)));
    };

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
                on:click=on_click
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

fn compute_game_result(
    running_data: GameRunningData,
    raw_result: yral_pump_n_dump_common::ws::GameResult,
) -> GameResult {
    if running_data.pumps == running_data.dumps {
        return GameResult::Win { amount: 0 };
    }

    let (user_direction, user_bet_count) = if running_data.pumps > running_data.dumps {
        (GameDirection::Pump, running_data.pumps)
    } else {
        (GameDirection::Dump, running_data.dumps)
    };

    // TODO: impl eq on GameDirection in yral-common
    let m = |direction: GameDirection| match direction {
        GameDirection::Pump => 0,
        GameDirection::Dump => 1,
    };
    if m(user_direction) != m(raw_result.direction) {
        GameResult::Loss
    } else {
        let amount = (user_bet_count / raw_result.bet_count) * raw_result.reward_pool;
        let amount = convert_e8s_to_gdolr(amount);
        GameResult::Win { amount }
    }
}

#[component]
fn GameCard(#[prop()] token: ProcessedTokenListResponse) -> impl IntoView {
    let owner_canister_id = token.token_owner.as_ref().unwrap().canister_id;
    let token_root = token.root;

    let websocket = create_rw_signal(None::<WebsocketContext>);
    provide_context(websocket);
    let running_data = create_rw_signal(None::<GameRunningData>);
    provide_context(running_data);
    let game_state = create_rw_signal(None::<GameState>);
    provide_context(game_state);
    provide_context(token);

    let player_data = expect_context::<RwSignal<Option<PlayerData>>>();

    let identity: RwSignal<Option<Canisters<true>>> = expect_context();
    let load_running_data = create_action(move |&user_canister| async move {
        let data = GameRunningData::load(owner_canister_id, token_root, user_canister)
            .await
            .inspect_err(|err| {
                logging::warn!("couldn't load running data: {err}");
            })
            .ok();

        running_data.update(|d| *d = data);
    });
    create_effect(move |_| {
        let ident = identity.get();
        if ident.as_ref().is_some() && running_data.get_untracked().is_none() {
            load_running_data.dispatch(ident.unwrap().user_canister());
        }
    });
    // start websocket connection
    create_effect(move |_| {
        let ident = identity.get();
        if let Some(value) = &ident {
            let mut ws_url = PUMP_AND_DUMP_WORKER_URL.clone();
            #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
            ws_url.set_scheme("wss").expect("schema to valid");

            #[cfg(any(feature = "local-bin", feature = "local-lib"))]
            ws_url.set_scheme("ws").expect("schema to valid");

            let websocket_url =
                websocket_connection_url(ws_url, value.identity(), owner_canister_id, token_root)
                    .map_err(|err| format!("Coulnd't create ws connection url: {err}"))?;

            let UseWebSocketReturn {
                message,
                send: sendfn,
                ..
            } = use_websocket::<WsRequest, WsResponse, JsonSerdeCodec>(websocket_url.as_str());

            // erase type, because sendfn is not send/sync
            let context = WebsocketContext::new(
                message,
                Rc::new(move |message: &WsRequest| {
                    sendfn(message);
                }),
            );

            websocket.update(|ws| *ws = Some(context));
        }
        Ok::<(), String>(())
    });

    create_effect(move |_| {
        if let Some(websocket) = websocket.get() {
            if let Some(message) = websocket.message.get() {
                match message.response {
                    WsResp::Ok => {
                        logging::log!("ws: received ok");
                    }
                    WsResp::Error(err) => {
                        // TODO: handle this error
                        logging::error!("ws: received error: {err:?}");

                        if let WsError::BetFailure { direction, .. } = err {
                            running_data.update(move |data| {
                                if let Some(data) = data {
                                    match direction {
                                        GameDirection::Pump => data.pumps -= 1,
                                        GameDirection::Dump => data.dumps -= 1,
                                    }
                                }
                            });
                        }
                    }
                    WsResp::GameResultEvent(result) => {
                        logging::log!("ws: received result: winning direction = {}, bet_count = {}, reward_pool = {}", match result.direction {
                             GameDirection::Pump => "pump",
                             GameDirection::Dump => "dump",
                        }, result.bet_count, result.reward_pool);
                        let running_data = running_data
                            .get()
                            .expect("running data to exist if we have recieved results");
                        let result = compute_game_result(running_data, result);

                        game_state.update(|s| *s = Some(GameState::ResultDeclared(result)));
                        match result {
                            GameResult::Win { amount } => {
                                player_data.update(|data| {
                                    if let Some(data) = data {
                                        data.games_count += 1;
                                        data.wallet_balance += amount;
                                    }
                                });
                            }
                            GameResult::Loss => {
                                player_data.update(|data| {
                                    if let Some(data) = data {
                                        data.games_count += 1;
                                    }
                                });
                            }
                        }
                    }
                    WsResp::WinningPoolEvent(pot) => {
                        logging::log!("ws: received new winning pot: {pot}");
                        running_data.update(|data| {
                            if let Some(data) = data {
                                data.winning_pot = Some(pot);
                            }
                        })
                    }
                }
            }
        }
    });

    let load_game_state = create_action(move |&()| async move {
        let state = GameState::load(owner_canister_id, token_root)
            .await
            .inspect_err(|err| {
                logging::error!("couldn't load game state: {err}");
            })
            .ok();

        game_state.update(|s| *s = state);

        Ok::<(), String>(())
    });

    create_effect(move |_| {
        // might dispatch multiple times, need a way to ensure game state is
        // loaded only once
        if game_state.get().is_none() {
            load_game_state.dispatch(());
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
    let player_games_count_and_balance = create_rw_signal(None::<PlayerData>);
    let cans_wire_res = authenticated_canisters();
    // i wonder if we remove this excessive cloning somehow
    let cans_wire_res_for_tokens = cans_wire_res.clone();
    let cans_wire_res_for_game_data = cans_wire_res.clone();
    let identity = create_rw_signal::<Option<Canisters<true>>>(None);
    provide_context(identity);
    let fetch_identity = create_action(move |&()| {
        let cans_wire_res = cans_wire_res.clone();
        async move {
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|e| e.to_string())?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|_| "Unable to authenticate".to_string())?;

            identity.update(|i| *i = Some(cans));

            Ok::<_, String>(())
        }
    });
    create_effect(move |_| {
        if identity.get_untracked().is_none() {
            fetch_identity.dispatch(());
        }
    });

    let fetch_user_principal = create_action(move |&()| {
        let cans_wire_res = cans_wire_res_for_game_data.clone();
        async move {
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|e| e.to_string())?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|_| "Unable to authenticate".to_string())?;

            let data = PlayerData::load(cans.user_canister())
                .await
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

    let tokens = create_rw_signal(Vec::<ProcessedTokenListResponse>::new());
    let page = create_rw_signal(1u32);
    let scroll_container = NodeRef::<Div>::new();
    let fetch_more_tokens = create_action(move |&page: &u32| {
        let cans_wire_res = cans_wire_res_for_tokens.clone();
        async move {
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|_| "Couldn't get cans_wire")?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|_| "Unable to authenticate".to_string())?;

            let user_principal = cans.user_principal();
            // to reduce the tokens loaded on initial load
            let limit = match page {
                1..10 => 1, // this number is based on what produced least token in preview
                _ => 5,
            };

            let more_tokens = get_paginated_token_list_with_limit(page, limit)
                .await
                .expect("TODO: handle error");
            let mut more_tokens =
                process_token_list_item(more_tokens.clone(), user_principal).await;
            // ignore tokens with no owners
            more_tokens.retain(|item| item.token_owner.is_some());

            tokens.update(|tokens| {
                tokens.extend_from_slice(&more_tokens);
            });

            Ok::<_, String>(())
        }
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
                    <For each=move || tokens.get() key=|item| item.token_details.token_name.clone() let:token>
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
