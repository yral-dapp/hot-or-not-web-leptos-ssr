use crate::pumpdump::{PlayerData, PlayerDataRes};
use component::skeleton::Skeleton;
use leptos::{either::Either, prelude::*};
use leptos_icons::Icon;

#[component]
fn HeaderSkeleton() -> impl IntoView {
    view! {
        <Skeleton class="text-neutral-800 [--shimmer:#363636] h-3.5 w-10 rounded-sm" />
    }
}

#[component]
fn HeaderCommon(#[prop(optional, into)] player_data: Option<Signal<PlayerData>>) -> impl IntoView {
    view! {
        <div class="flex items-center w-full justify-between pt-2 pb-3.5 gap-8">
            <a
                href="/pnd/profile"
                class="flex flex-col text-right text-sm ml-8 relative bg-neutral-900 rounded-lg pt-1 pb-1.5 pr-3 pl-8"
            >
                <div class="font-bold text-sm">
                    {if let Some(pd) = player_data {
                        let game_count = pd.with(|pd| pd.games_count.to_string());
                        Either::Left(game_count)
                    } else {
                        Either::Right(view! {
                            <HeaderSkeleton/>
                        }.into_view())
                    }}
                </div>
                <div class="text-xs text-neutral-400 uppercase">Games</div>
                <img
                    src="/img/pumpdump/gamepad.webp"
                    alt="Games"
                    class="absolute select-none -left-1/4 bottom-0 h-12 w-12 -rotate-1"
                />
            </a>
            <a
                href="/wallet"
                class="flex flex-col text-left overf mr-8 relative bg-neutral-900 rounded-lg pt-1 pb-1.5 pl-4 pr-8"
            >
                <div
                    class="font-bold absolute top-1 text-sm"
                >
                    {if let Some(pd) = player_data {
                        let wallet_balance = pd.with(|pd| pd.wallet_balance.to_string().replace("_", ""));
                        Either::Left(wallet_balance)
                    } else {
                        Either::Right(view! {
                            <HeaderSkeleton/>
                        })
                    }}
                </div>
                <div class="h-5 opacity-0"></div>
                <div class="text-xs text-neutral-400">Cents</div>
                <img
                    src="/img/pumpdump/cents.webp"
                    alt="Cents"
                    class="absolute select-none -right-1/4 bottom-1 size-9 -rotate-1"
                />
                <div class="absolute rounded-sm bg-[#212121] text-neutral-600 p-0.5 size-5 -left-2 top-1/2 -translate-y-1/2">
                    <Icon attr:class="size-full" icon=icondata::FiPlus />
                </div>
            </a>
        </div>
    }
}

#[component]
pub fn Header() -> impl IntoView {
    let data: PlayerDataRes = expect_context();
    view! {
        <Suspense fallback=|| view! { <HeaderCommon/> }>
            {move || data.read.get().map(|d| match d {
                Ok(d) => view! {
                    <HeaderCommon player_data=d/>
                }.into_any(),
                Err(_) => view! {
                    <HeaderCommon/>
                }.into_any(),
            })}
        </Suspense>
    }
}
