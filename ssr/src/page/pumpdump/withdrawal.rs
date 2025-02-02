use leptos::{component, view, IntoView};

use crate::component::{
    back_btn::BackButton,
    icons::{information_icon::Information, notification_icon::NotificationIcon},
    title::Title,
    tooltip::Tooltip,
};

#[component]
pub fn PndWithdrawal() -> impl IntoView {
    view! {
        <div class="min-h-screen w-full flex flex-col text-white pt-2 pb-12 bg-black items-center">
            <div id="back-nav" class="flex flex-col items-center w-full gap-20 pb-16">
                <Title justify_center=false>
                    <div class="flex flex-row justify-between">
                        <BackButton fallback="/" />
                        <span class="font-bold text-2xl">Withdraw</span>
                        <a href="/wallet/notifications" disabled=true class="text-xl font-semibold">
                            <NotificationIcon show_dot=false classes="w-8 h-8 text-neutral-600".to_string() />
                        </a>
                    </div>
                </Title>
            </div>
            <div class="w-full">
                <div class="flex flex-col items-center justify-center max-w-md mx-auto px-4 mt-4 pb-6">
                    <div id="total-balance" class="self-center flex flex-col items-center gap-1">
                        <span class="text-neutral-400 text-sm">Total gDOLR balance</span>
                        <div class="flex items-center gap-3 min-h-14 py-0.5">
                            <img class="size-9" src="/img/gdolr.png" alt="gdolr icon" />
                            <span class="font-bold text-4xl">500</span>
                        </div>
                    </div>
                    <div id="breakdown" class="self-center flex justify-around py-3 px-7 bg-neutral-800 lg:w-full min-w-72 gap-8 mt-5 rounded-lg">
                        <div id="airdrop" class="flex flex-col items-end">
                            <span class="text-xs text-neutral-400">Airdrop</span>
                            <span class="text-2xl font-bold">400</span>
                        </div>
                        <div id="divider" class="bg-neutral-700 h-10 w-0.5 rounded-full self-center"></div>
                        <div id="winnings" class="flex flex-col items-start">
                            <span class="text-xs text-neutral-400">Winnings</span>
                            <span class="text-2xl font-bold">100</span>
                        </div>
                    </div>
                    <div class="flex flex-col gap-5 mt-8 w-full">
                        <span class="text-sm">Choose how much to redeem:</span>
                        <div id="input-card" class="rounded-lg bg-neutral-900 p-3 flex flex-col gap-8">
                            <div class="flex flex-col gap-3">
                                <div class="flex justify-between">
                                    <div class="flex gap-2 items-center">
                                        <span>You withdraw</span>
                                        <Tooltip icon=Information title="Withdrawal Tokens" description="Only gDOLRs earned above your airdrop amount can be withdrawn." />
                                    </div>
                                    <input type="text" inputmode="decimal" class="bg-neutral-800 h-10 w-32 rounded focus:outline focus:outline-1 focus:outline-[#E2017B] text-right px-4 text-lg" />
                                </div>
                                <div class="flex justify-between">
                                    <div class="flex gap-2 items-center">
                                        <span>You get</span>
                                    </div>
                                    <input disabled type="text" inputmode="decimal" class="bg-neutral-800 h-10 w-32 rounded focus:outline focus:outline-1 focus:outline-[#E2017B] text-right px-4 text-lg text-neutral-400" value="1DOLR" />
                                </div>
                            </div>
                            <button
                                style:background="linear-gradient(218.27deg, #FF78C1 9.83%, #E2017B 44.79%, #5F0938 78.48%)"
                                class="rounded-lg px-5 py-2 text-sm text-center font-bold"
                            >Withdraw Now!</button>
                        </div>
                        <span class="text-sm">1 gDOLR = 0.01 DOLR</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
