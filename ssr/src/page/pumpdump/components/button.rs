use leptos::{component, expect_context, logging, view, IntoView, SignalGet, SignalUpdate};
use yral_pump_n_dump_common::{
    ws::{WsMessage, WsRequest},
    GameDirection,
};

use crate::page::pumpdump::{
    CurrentRoundSignal, GameRunningDataSignal, PlayerDataSignal, WebsocketContextSignal,
};

#[cfg(not(feature = "hydrate"))]
fn non_visual_feedback() {}

#[cfg(feature = "hydrate")]
fn non_visual_feedback() {
    use leptos_use::use_window;
    use wasm_bindgen::JsValue;
    use web_sys::HtmlAudioElement;
    let navigator = use_window().navigator();
    match navigator {
        Some(navigator) => {
            if js_sys::Reflect::has(&navigator, &JsValue::from_str("vibrate")).unwrap_or(false) {
                navigator.vibrate_with_duration(5);
            } else {
                logging::warn!("Browser doesn't support vibrate api");
            }
        }
        None => logging::warn!("Couldn't get navigator for vibration"),
    }

    if let Err(err) = HtmlAudioElement::new_with_src("/pnd-tap.mp3").and_then(|d| {
        d.set_volume(0.5);
        d.play()
    }) {
        web_sys::console::warn_2(&JsValue::from_str("error playing tap audio"), &err);
    }
}

#[component]
pub fn DumpButton() -> impl IntoView {
    let running_data: GameRunningDataSignal = expect_context();
    let player_data: PlayerDataSignal = expect_context();
    let websocket: WebsocketContextSignal = expect_context();
    let current_round: CurrentRoundSignal = expect_context();
    let counter = move || {
        running_data
            .get()
            .map(|value| value.dumps.to_string())
            .unwrap_or_else(|| "-".into())
    };
    let onclick = move |_| {
        non_visual_feedback();
        if let (Some(websocket), Some(round)) = (websocket.get().as_ref(), current_round.get()) {
            websocket.send(&WsRequest {
                request_id: uuid::Uuid::new_v4(),
                msg: WsMessage::Bet {
                    direction: GameDirection::Dump,
                    round: round.0,
                },
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
    let current_round: CurrentRoundSignal = expect_context();
    let counter = move || {
        running_data
            .get()
            .map(|value| value.pumps.to_string())
            .unwrap_or_else(|| "-".into())
    };
    let onclick = move |_| {
        non_visual_feedback();
        if let (Some(websocket), Some(round)) = (websocket.get().as_ref(), current_round.get()) {
            websocket.send(&WsRequest {
                request_id: uuid::Uuid::new_v4(),
                msg: WsMessage::Bet {
                    direction: GameDirection::Pump,
                    round: round.0,
                },
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
