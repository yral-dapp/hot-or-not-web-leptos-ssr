use leptos::{component, expect_context, view, IntoView, Show, SignalGet};
use leptos_icons::*;

use crate::{
    component::icons::chevron_right_icon::ChevronRightIcon,
    page::{
        icpump::ProcessedTokenListResponse,
        pumpdump::{
            components::{
                button::{DumpButton, PumpButton},
                slider::BullBearSlider,
            },
            GameResult, GameRunningDataSignal, GameState, IdentitySignal, LoadRunningDataAction,
            ShowOnboarding,
        },
    },
};

#[component]
pub fn PlayingCard() -> impl IntoView {
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

    let token_link = token.token_details.link.clone();

    let player_count = move || {
        running_data
            .get()
            .map(|data| data.player_count)
            .map(|value| value.to_string())
            .unwrap_or_else(|| "--".into())
    };
    view! {
        <div
            class="bg-[#171717] flip-card transition-all absolute inset-0 h-full shrink-0 rounded-2xl flex flex-col gap-4 pt-14 pb-5 px-5 overflow-hidden"
        >
            <div class="flex flex-col gap-6 w-full items-center">
                <img class="size-32 rounded-[4px]" alt="Avatar" src=token.token_details.logo />
                <a href=token_link class="flex items-center gap-1">
                    <div class="font-bold text-lg">{token.token_details.token_name}</div>
                    <Icon icon=ChevronRightIcon class="w-5 h-5 -mb-px" />
                </a>
                <div class="bg-[#212121] shrink-0 rounded-full relative w-full h-11 overflow-hidden">
                    <div
                        class="w-full animate-slide-up top-[3.5rem] flex items-center gap-2 justify-between absolute inset-0 py-2 pl-4 pr-2"
                    >
                        <div class="flex items-center gap-1">
                            <div class="text-[#A3A3A3] text-xs">Winning Pot:</div>
                            <img src="/img/cents.png" alt="Coin" class="size-5" />
                            <div class="text-[#E5E5E5] font-bold">{winning_pot} Cents</div>
                        </div>
                        <button
                            on:click=move |_| show_onboarding.show()
                            class="bg-black text-neutral-400 font-bold hover:bg-black/35 rounded-full text-xl w-7 h-7 flex items-center justify-center leading-none"
                        >
                            ?
                        </button>
                    </div>
                    <div
                        style="--animation-delay:5s;"
                        class="w-full top-[3.5rem] animate-slide-up flex items-center gap-1 absolute inset-0 py-2 pl-4 pr-2"
                    >
                        <img src="/img/player.png" alt="Coin" class="w-5 h-5" />
                        <div class="text-[#E5E5E5] font-bold">{player_count}</div>
                        <div class="text-[#A3A3A3] text-xs">players are playing - join the action!</div>
                    </div>
                </div>
            </div>
            <div class="flex select-none flex-col gap-4 h-[8.5rem] w-full">
                <BullBearSlider />
                <div
                    class="flex relative items-center gap-6 justify-center w-full"
                >
                    <DumpButton />

                    <PumpButton />
                </div>
            </div>
        </div>
    }
}

#[component]
fn WonCard(#[prop()] result: GameResult) -> impl IntoView {
    let GameResult::Win { amount } = result else {
        unreachable!("Won card must only be shown in win condition")
    };
    let identity: IdentitySignal = expect_context();
    let load_running_data: LoadRunningDataAction = expect_context();
    let pending = load_running_data.pending();

    let on_click = move |_| {
        let user_canister = identity
            .get()
            .expect("User Canister to exist at this point")
            .user_canister();
        load_running_data.dispatch((user_canister, true));
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
                    <img src="/img/cents.png" alt="Coin" class="w-5 h-5" />
                    <span class="text-[#E5E5E5] font-bold">{amount} Cents</span>
                </div>
            </div>
            <button
                disabled=move || pending.get()
                on:click=on_click
                class="w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold"
                style:background="linear-gradient(73deg, #DA539C 0%, #E2017B 33%, #5F0938 100%)"
            >
                {move || if load_running_data.pending().get() {
                    "Start playing again"
                } else {
                    "Starting another round..."
                }}
            </button>
        </div>
    }
}

#[component]
fn LostCard() -> impl IntoView {
    let identity: IdentitySignal = expect_context();
    let load_running_data: LoadRunningDataAction = expect_context();
    let pending = load_running_data.pending();

    let on_click = move |_| {
        let user_canister = identity
            .get()
            .expect("User Canister to exist at this point")
            .user_canister();
        load_running_data.dispatch((user_canister, true));
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
                disabled=move || pending.get()
                on:click=on_click
                class="w-full px-5 py-3 rounded-lg flex items-center transition-all justify-center gap-8 font-kumbh font-bold"
                style:background="linear-gradient(73deg, #DA539C 0%, #E2017B 33%, #5F0938 100%)"
            >
                {move || if load_running_data.pending().get() {
                    "Keep Playing"
                } else {
                    "Starting another round..."
                }}
            </button>
        </div>
    }
}

#[component]
pub fn ResultDeclaredCard(#[prop()] game_state: GameState) -> impl IntoView {
    match game_state {
        GameState::Playing => {
            unreachable!("This screen is not reachable until ResultDeclared state is reached")
        }
        GameState::ResultDeclared(result) => view! {
            <Show
                when=move || matches!(result, GameResult::Loss { .. })
                fallback=move || view! { <WonCard result /> }
            >
                <LostCard />
            </Show>
        },
    }
}
