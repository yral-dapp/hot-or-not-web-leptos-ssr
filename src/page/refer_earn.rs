use leptos::*;
use leptos_icons::*;

#[component]
fn WorkButton(#[prop(into)] text: String, #[prop(into)] icon: icondata::Icon) -> impl IntoView {
    view! {
        <div class="flex flex-col items-center gap-3">
            <div class="grid h-12 w-12 place-items-center rounded-sm bg-white/10">
                <Icon class="text-orange-600 text-xl" icon=icon/>
            </div>
            <span class="text-sm">{text}</span>
        </div>
    }
}

#[component]
pub fn ReferEarn() -> impl IntoView {
    view! {
        <div class="flex flex-col w-screen h-screen items-center bg-black text-white py-4 px-8 gap-10">
            <span class="text-lg font-bold">Refer & Earn</span>
            <img class="shrink-0 h-40 select-none" src="/img/coins-stash.webp"/>
            <div class="flex flex-col w-full items-center gap-4">
                <span class="font-bold text-2xl">Invite & Win 500 Tokens</span>
                <span class="text-white/50 text-xs">Please login to see your referral link</span>
            </div>
            <button class="rounded-full w-full bg-orange-600 text-xl px-4 py-2">Login</button>
            <div class="flex flex-col w-full items-center gap-8 mt-4">
                <span class="font-xl">How does it work?</span>
                <div class="flex flex-row gap-8">
                    <WorkButton text="Share your link with a friend" icon=icondata::TbShare3/>
                    <WorkButton
                        text="Your friends download and log into the app"
                        icon=icondata::TbCloudDownload
                    />
                    <WorkButton
                        text="You both win 500 tokens each"
                        icon=icondata::AiDollarCircleOutlined
                    />
                </div>
            </div>
        </div>
    }
}
