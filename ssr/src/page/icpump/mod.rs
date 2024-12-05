use std::collections::HashMap;
use std::collections::VecDeque;

use futures::StreamExt;
use leptos::*;

use crate::component::spinner::FullScreenSpinner;
use crate::component::icons::airdrop_icon::AirdropIcon;
use crate::component::icons::arrow_left_right_icon::ArrowLeftRightIcon;
use crate::component::icons::chevron_right_icon::ChevronRightIcon;
use crate::component::icons::eye_hide_icon::EyeHiddenIcon;
use crate::component::icons::send_icon::SendIcon;
use crate::component::icons::share_icon::ShareIcon;
use crate::consts::ICPUMP_LISTING_PAGE_SIZE;
use crate::utils::token::firestore::init_firebase;
use crate::utils::token::firestore::listen_to_documents;
use crate::utils::token::icpump::get_paginated_token_list;
use crate::utils::token::icpump::TokenListItem;

pub mod ai;



#[component]
pub fn ICPumpListing() -> impl IntoView {
    let page = create_rw_signal(1);
    let token_list: RwSignal<Vec<TokenListItem>> = create_rw_signal(vec![]);
    let end_of_list = create_rw_signal(false);
    let cache = create_rw_signal(HashMap::<u64, Vec<TokenListItem>>::new());
    let new_token_list: RwSignal<VecDeque<TokenListItem>> = create_rw_signal(VecDeque::new());

    let act = create_resource(page, move |page| async move {
        // reset new_token_list
        new_token_list.set(VecDeque::new());

        if let Some(cached) = cache.with_untracked(|c| c.get(&page).cloned()) {
            return cached.clone();
        }

        get_paginated_token_list(page as u32).await.unwrap()
    });

    create_effect(move |_| {
        spawn_local(async move {
            let (_app, firestore) = init_firebase();
            let mut stream = listen_to_documents(&firestore);
            while let Some(doc) = stream.next().await {
                // push each item in doc to new_token_list
                for item in doc {
                    new_token_list.update(move |list| {
                        list.push_front(item.clone());
                    });
                }
            }
        });
    });

    view! {
        <div class="flex flex-col justify-center mt-6 mb-10">
            <Suspense fallback=FullScreenSpinner>
                {move || {
                    let _ = act
                        .get()
                        .map(|res| {
                            if res.len() < ICPUMP_LISTING_PAGE_SIZE {
                                end_of_list.set(true);
                            }
                            update!(
                                move |token_list, cache| {
                                *token_list = res.clone();
                                cache.insert(page.get_untracked(), res.clone());
                            }
                            );
                        });
                    view! {
                        <div class="grid grid-col-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                            <For
                                each=move || new_token_list.get()
                                key=|t| t.token_symbol.clone()
                                children=move |token: TokenListItem| {
                                    view! { <TokenListing details=token is_new_token=true /> }
                                }
                            />
                            <For
                                each=move || token_list.get()
                                key=|t| t.token_symbol.clone()
                                children=move |token: TokenListItem| {
                                    view! { <TokenListing details=token /> }
                                }
                            />
                        </div>

                        <div class="flex flex-row justify-center mt-5">
                            <button
                                class="text-gray-100 active:italic hover:enabled:text-pink-200 hover:enabled:underline active:enabled:text-pink-500 disabled:cursor-not-allowed disabled:text-gray-500"
                                on:click=move |_| {
                                    page.update(|page| *page -= 1);
                                    end_of_list.set(false);
                                }
                                disabled=move || page.get() == 1
                            >
                                {"[ << ]"}
                            </button>
                            <span class="mx-2">{page}</span>
                            <button
                                class="text-gray-100 active:italic hover:enabled:text-pink-200 hover:enabled:underline active:enabled:text-pink-500 disabled:cursor-not-allowed disabled:text-gray-500"
                                on:click=move |_| {
                                    page.update(|page| *page += 1);
                                }
                                disabled=move || end_of_list.get()
                            >
                                {"[ >> ]"}
                            </button>
                        </div>
                    }
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn ICPumpLanding() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-black text-white overflow-y-scroll pt-5 pb-12">
            <div class="flex ml-4 space-x-2 justify-center items-center sm:justify-start">
                <div class="text-gray-100 hover:text-pink-200 hover:underline active:text-pink-500 active:italic">
                    <a href="https://twitter.com/Yral_app" target="_blank">
                        [twitter]
                    </a>
                </div>
                <div class="text-gray-100 hover:text-pink-200 hover:underline active:text-pink-500 active:italic">
                    <a href="https://www.instagram.com/yral_app/" target="_blank">
                        [instagram]
                    </a>
                </div>
                <div class="text-gray-100 hover:text-pink-200 hover:underline active:text-pink-500 active:italic">
                    <a href="https://t.me/+c-LTX0Cp-ENmMzI1" target="_blank">
                        [telegram]
                    </a>
                </div>
            </div>
            <div class="flex justify-center items-center pt-6">
                <div class="font-bold text-3xl hover:font-extrabold hover:underline active:italic animate-blink-colors">
                    <a href="/token/create">[ create a new coin ]</a>
                </div>
            </div>
            <div class="px-4">
                <ICPumpListing />
            </div>
        </div>
    }
}


#[component]
pub fn TokenListing(
    details: TokenListItem,
    #[prop(optional, default = false)] is_new_token: bool,
) -> impl IntoView {
    let show_nsfw = create_rw_signal(false);

    view! {
        <div
            class:tada=is_new_token
            class="flex flex-col gap-2 py-3 px-3 w-full text-xs rounded-lg transition-colors md:px-4 hover:bg-gradient-to-b group bg-[#131313] font-kumbh hover:from-[#626262] hover:to-[#3A3A3A]"
        >
            <div class="flex gap-3">
                <div
                    style="box-shadow: 0px 0px 4px rgba(255, 255, 255, 0.16);"
                    class="overflow-hidden relative w-[7rem] h-[7rem] rounded-[4px] shrink-0"
                >
                    <Show when=move || details.is_nsfw && !show_nsfw.get()>
                        <button
                            on:click=move |_| show_nsfw.set(!show_nsfw.get())
                            class="flex absolute inset-0 justify-center items-center w-full h-full z-[2] backdrop-blur-[4px] bg-black/50 rounded-[4px]"
                        >
                            <div class="flex flex-col gap-1 items-center text-xs">
                                <EyeHiddenIcon classes="w-6 h-6".to_string() />
                                <span class="uppercase">nsfw</span>
                            </div>
                        </button>
                    </Show>
                    <img
                        alt=details.token_name.clone()
                        src=details.logo.clone()
                        class="w-full h-full"
                    />
                </div>
                <div class="flex flex-col gap-3 text-left">
                    <div class="flex gap-4 justify-between items-center w-full text-lg">
                        <span class="font-medium shrink line-clamp-1">{details.name}</span>
                        <span class="font-bold shrink-0">{details.token_symbol}</span>
                    </div>
                    <span class="text-sm transition-colors group-hover:text-white line-clamp-2 text-[#A0A1A6]">
                        {details.description}
                    </span>
                    <div class="flex gap-2 justify-between items-center text-sm font-medium group-hover:text-white text-[#505156]">
                        <span class="line-clamp-1">"Created by" {details.user_id}</span>
                        <span class="shrink-0">{details.formatted_created_at}</span>
                    </div>
                </div>
            </div>
            <div class="flex gap-4 justify-between items-center p-2">
                <BoardCardButton label="Send".to_string() href="#".to_string()>
                    <SendIcon classes="w-full h-full".to_string() />
                </BoardCardButton>
                <BoardCardButton label="Buy/Sell".to_string() href="#".to_string()>
                    <ArrowLeftRightIcon classes="w-full h-full".to_string() />
                </BoardCardButton>
                <BoardCardButton label="Airdrop".to_string() href="#".to_string()>
                    <AirdropIcon classes="w-full h-full".to_string() />
                </BoardCardButton>
                <BoardCardButton label="Share".to_string() href="#".to_string()>
                    <ShareIcon classes="w-full h-full".to_string() />
                </BoardCardButton>
                <BoardCardButton label="Details".to_string() href=details.link>
                    <ChevronRightIcon classes="w-full h-full".to_string() />
                </BoardCardButton>
            </div>
        </div>
    } 
    
}

#[component]
pub fn BoardCardButton(href: String, label: String, children: Children) -> impl IntoView {
	view! {
        <a
            href=href
            class="flex flex-col gap-1 justify-center items-center text-xs transition-colors group-hover:text-white text-[#A0A1A6]"
        >
            <div class="w-[1.875rem] h-[1.875rem]">{children()}</div>

            <div>{label}</div>
        </a>
    }
}	
