use leptos::{component, expect_context, view, IntoView, SignalGet};

use crate::page::pumpdump::GameRunningDataSignal;

#[component]
pub fn BullBearSlider() -> impl IntoView {
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
pub fn MockBullBearSlider() -> impl IntoView {
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
