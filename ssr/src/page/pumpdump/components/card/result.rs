use leptos::{component, expect_context, view, IntoView, Show, SignalGet};

use crate::page::{
    icpump::ProcessedTokenListResponse,
    pumpdump::{
        components::{
            button::{DumpButton, PumpButton},
            slider::BullBearSlider,
        },
        GameResult, GameRunningDataSignal, GameState, IdentitySignal, LoadRunningDataAction,
        ShowOnboarding,
    },
};

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
pub fn GameCardPreResult(#[prop(into)] game_state: GameState) -> impl IntoView {
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
    let identity: IdentitySignal = expect_context();
    let load_running_data: LoadRunningDataAction = expect_context();

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
                    <img src="/img/gdolr.png" alt="Coin" class="w-5 h-5" />
                    <span class="text-[#E5E5E5] font-bold">{amount} gDOLR</span>
                </div>
            </div>
            <button
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
pub fn ResultDeclared(#[prop()] game_state: GameState) -> impl IntoView {
    match game_state {
        GameState::Playing | GameState::Pending => {
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
