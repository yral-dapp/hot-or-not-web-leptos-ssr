use leptos::{component, expect_context, view, IntoView, SignalGet, SignalUpdate};
use yral_pump_n_dump_common::{
    ws::{WsMessage, WsRequest},
    GameDirection,
};

use crate::page::pumpdump::{GameRunningDataSignal, PlayerDataSignal, WebsocketContextSignal};

#[component]
pub fn DumpButton() -> impl IntoView {
    let running_data: GameRunningDataSignal = expect_context();
    let player_data: PlayerDataSignal = expect_context();
    let websocket: WebsocketContextSignal = expect_context();
    let counter = move || {
        running_data
            .get()
            .map(|value| value.dumps.to_string())
            .unwrap_or_else(|| "-".into())
    };
    let onclick = move |_| {
        if let Some(websocket) = websocket.get().as_ref() {
            websocket.send(&WsRequest {
                request_id: uuid::Uuid::new_v4(),
                msg: WsMessage::Bet(GameDirection::Dump),
            });

            player_data.update(|value| {
                if let Some(value) = value.as_mut() {
                    value.wallet_balance = value.wallet_balance.saturating_sub(1);
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
            class="dump-button rounded-[28px] transition-all duration-150 ring-4 group text-white ring-white/25 gap-2 min-w-36 p-3 flex flex-col items-center justify-center touch-none"
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
pub fn MockDumpButton() -> impl IntoView {
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
pub fn PumpButton() -> impl IntoView {
    let running_data: GameRunningDataSignal = expect_context();
    let player_data: PlayerDataSignal = expect_context();
    let websocket: WebsocketContextSignal = expect_context();
    let counter = move || {
        running_data
            .get()
            .map(|value| value.pumps.to_string())
            .unwrap_or_else(|| "-".into())
    };
    let onclick = move |_| {
        // TODO: add debouncing
        if let Some(websocket) = websocket.get().as_ref() {
            websocket.send(&WsRequest {
                request_id: uuid::Uuid::new_v4(),
                msg: WsMessage::Bet(GameDirection::Pump),
            });

            player_data.update(|value| {
                if let Some(value) = value.as_mut() {
                    value.wallet_balance = value.wallet_balance.saturating_sub(1);
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
            class="pump-button rounded-[28px] transition-all duration-150 ring-4 group text-white ring-white/25 gap-2 min-w-36 p-3 flex flex-col items-center justify-center touch-none"
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
pub fn MockPumpButton() -> impl IntoView {
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
