use leptos::{component, expect_context, provide_context, view, IntoView, Resource, SignalGet};
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
            </div>
        </div>
    }
}
