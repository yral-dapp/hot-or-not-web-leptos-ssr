use std::collections::HashMap;
use std::collections::VecDeque;

use futures::StreamExt;
use leptos::*;
use leptos_icons::*;

use crate::component::spinner::FullScreenSpinner;
use crate::consts::ICPUMP_LISTING_PAGE_SIZE;
use crate::utils::token::firestore::init_firebase;
use crate::utils::token::firestore::listen_to_documents;
use crate::utils::token::icpump::get_paginated_token_list;
use crate::utils::token::icpump::TokenListItem;

pub mod ai;

#[component]
pub fn TokenListing(
    details: TokenListItem,
    #[prop(optional, default = false)] is_new_token: bool,
) -> impl IntoView {
    view! {
        <a
            href=details.link
            class="relative flex h-fit max-h-[300px] w-full gap-2 overflow-hidden border border-transparent p-2 transition-colors hover:border-gray-700 active:border-gray-200"
            class:tada=is_new_token
        >
            <div class="min-w-32 relative self-start p-1">
                <img
                    class="mr-4 w-32 h-auto select-none"
                    class:blur-lg=details.is_nsfw
                    src=details.logo
                    alt=details.token_name.clone()
                />
                {move || {
                    if details.is_nsfw {
                        view! {
                            <div class="absolute inset-0 flex items-center justify-center">
                                <Icon
                                    icon=icondata::AiEyeInvisibleOutlined
                                    class="w-8 h-8 text-gray-200"
                                />
                            </div>
                        }
                    } else {
                        // empty div
                        view! { <div></div> }
                    }
                }}
            </div>
            <div class="gap-1 flex-col flex h-fit">
                <div class="flex items-center justify-between gap-4 text-gray-200">
                    <span
                        class="line-clamp-1 w-full overflow-hidden"
                        style="word-break: break-word;"
                    >
                        {details.token_name}
                    </span>
                    <span class="shrink-0 font-bold underline">
                        <span class="text-gray-400 italic select-none">"$"</span>
                        {details.token_symbol}
                    </span>
                </div>
                <div
                    title=details.description.clone()
                    class="text-sm line-clamp-3 text-gray-400"
                    style="word-break: break-word"
                >
                    {details.description}
                </div>
                <div class="text-xs text-gray-500 line-clamp-2 pr-10">
                    "Created by: "<span class="select-all">{details.user_id}</span>
                    <span class="invisible">{details.formatted_created_at.clone()}</span>
                </div>
                <span class="absolute bottom-3 right-2 shrink-0 text-xs text-gray-500 underline">
                    {details.formatted_created_at}
                </span>
            </div>
        </a>
    }
}

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
                                    view! { <TokenListing details=token is_new_token=true/> }
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
