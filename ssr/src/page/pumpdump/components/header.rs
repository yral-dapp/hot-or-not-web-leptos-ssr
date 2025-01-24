use leptos::{component, expect_context, view, IntoView, SignalGet};
use leptos_icons::Icon;

use crate::page::pumpdump::PlayerDataSignal;

#[component]
pub fn Header() -> impl IntoView {
    let data: PlayerDataSignal = expect_context();
    view! {
        <div class="flex items-center w-full justify-between pt-2 pb-3.5 gap-8">
            <a
                href="/pnd/profile"
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
