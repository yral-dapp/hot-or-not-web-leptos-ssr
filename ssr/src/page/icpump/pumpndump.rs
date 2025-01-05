use leptos::{
    component, expect_context, provide_context, view, IntoView, Resource, SignalGet, SignalUpdate,
};
use leptos_icons::Icon;

#[component]
fn Header() -> impl IntoView {
    let data = expect_context::<Resource<(), PlayerGamesCountAndBalance>>();
    view! {
        <div class="flex items-center w-full justify-between py-2 gap-8">
            <a
                href="/pump-dump/profile"
                class="flex flex-col text-right text-sm ml-8 relative bg-[#171717] rounded-lg pt-1 pb-1.5 pr-3 pl-8"
            >
                <div class="font-bold text-sm">{move || data.get().map(|d| format!("{}", d.games_count)).unwrap_or_else(|| "----".into())}</div>
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
    // TODO: set the position for the bear and bull based on current pump/dump number of parent
    let ratio = 1f64;
    let position = move || {
        if ratio == 1.0 {
            39f64
        } else {
            78f64.min(0f64.max(ratio / (ratio + 1.0)))
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
fn DumpButton() -> impl IntoView {
    let game_state = expect_context::<Resource<(), GameState>>();
    let player_data = expect_context::<Resource<(), PlayerGamesCountAndBalance>>();
    let counter = move || {
        game_state
            .get()
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

        game_state.update(|value| {
            if let Some(value) = value.as_mut() {
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
fn PumpButton() -> impl IntoView {
    let game_state = expect_context::<Resource<(), GameState>>();
    let player_data = expect_context::<Resource<(), PlayerGamesCountAndBalance>>();
    let counter = move || {
        game_state
            .get()
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
            if let Some(value) = value.as_mut() {
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
    pub async fn load() -> Self {
        Self::new(0, 1000)
    }

    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    pub async fn load() -> Self {
        unimplemented!("Haven't figured out how to load games count and wallet balance yet")
    }
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
struct GameState {
    pumps: u64,
    dumps: u64,
    winning_pot: u64,
}

impl GameState {
    pub fn new(pumps: u64, dumps: u64, winning_pot: u64) -> Self {
        Self {
            pumps,
            dumps,
            winning_pot,
        }
    }

    #[cfg(any(feature = "local-bin", feature = "local-lib"))]
    pub async fn load() -> Self {
        Self::new(0, 0, 100)
    }

    #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
    pub async fn load() -> Self {
        unimplemented!("Haven't figured out how to load game state")
    }
}

#[component]
fn GameCard() -> impl IntoView {
    provide_context(Resource::new(move || (), |_| GameState::load()));

    let winning_pot = 100;
    view! {
        <div
            style="perspective: 500px; transition: transform 0.4s; transform-style: preserve-3d;"
            class="relative w-full h-[31rem]"
        >
            <div
                class="bg-[#171717] flip-card transition-all absolute inset-0 h-full shrink-0 rounded-2xl items-center flex flex-col gap-4 w-full pt-14 pb-5 px-5 overflow-hidden"
            >
                <img class="mt-14 w-24 h-24 rounded-[4px]" alt="Avatar" src="/img/gamepad.png" />
                <a href="#" class="flex items-center gap-1">
                    <div class="font-bold text-lg">iseng iseng Token</div>
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
                            on:click=move |_| unimplemented!("show onboarding card")
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
                    <BullBearSlider />
                    <div
                        class="flex relative items-center gap-6 justify-center w-full"
                    >
                        <DumpButton />

                        <PumpButton />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn PumpNDump() -> impl IntoView {
    provide_context(Resource::new(
        move || (),
        |_| PlayerGamesCountAndBalance::load(),
    ));

    view! {
        <div class="h-screen w-screen block text-white bg-black">
            <div class="max-w-md flex flex-col relative w-full mx-auto items-center h-full px-4 py-4">
                <Header />
                <GameCard />
            </div>
        </div>
    }
}
