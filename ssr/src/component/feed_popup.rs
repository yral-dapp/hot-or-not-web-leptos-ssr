use crate::component::connect::ConnectLogin;
use crate::component::download_pwa_feedpopup::DownloadPwaLink;
use leptos::{ev::MouseEvent, *};
#[component]
pub fn FeedPopUp<F: Fn(MouseEvent) + 'static>(
    on_click: F,
    header_text: &'static str,
    body_text: &'static str,
    #[prop(optional)] login_text: &'static str,
    #[prop(optional)] download_pwa_text: &'static str,
    #[prop(optional, default = false)] is_pwa_download: bool,
) -> impl IntoView {
    view! {
               <div
        class="flex absolute z-50 flex-col justify-center w-full h-full bg-black opacity-90"
        on:click=on_click
        >
        <div class="flex flex-row justify-center">
            <div class="flex relative flex-col justify-center w-9/12 sm:w-4/12">
                <img
                    class="absolute -left-4 -top-10 w-28 h-28"
                    src="/img/coins/coin-topleft.svg"
                    />
                <img
                    class="absolute -right-2 -top-14 h-18 w-18"
                    src="/img/coins/coin-topright.svg"
                    />
                <img
                    class="absolute -left-8 -bottom-14 h-18 w-18"
                    src="/img/coins/coin-bottomleft.svg"
                    />
                <img
                    class="absolute -right-2 -bottom-12 h-18 w-18"
                    src="/img/coins/coin-bottomright.svg"
                    />
                <Show when=move || !is_pwa_download>
                <span class="p-2 text-3xl text-center text-white whitespace-pre-line text-bold">
                {header_text}
                </span>
                <span class="p-2 pb-4 text-center text-white">{body_text}</span>
                <div class="flex justify-center">
                    <div class="w-7/12 sm:w-4/12 z-[60]">
                        <ConnectLogin
                            login_text=login_text cta_location="feed_popup"
                            />
                    </div>
                </div>
                </Show>
                <Show when=move || is_pwa_download>
                    <span class=  "p-2 pb-4 text-3xl font-bold text-center text-white whitespace-pre-line sm:text-3xl md:text-4xl lg:p-4 lg:px-8 lg:text-5xl">
                    {header_text}
                    </span>
                    <span class="p-1 px-2 pb-7 text-center text-white md:p-1 md:px-5 md:pb-7 lg:p-1 lg:px-7 lg:pb-7 lg:text-xl">{body_text}</span>
                    <div class="flex justify-center">
                        <div class="z-[60]">
                            <DownloadPwaLink
                                download_pwa_text=download_pwa_text />
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    </div>        }
}
