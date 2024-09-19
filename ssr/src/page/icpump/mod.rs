use leptos::*;

use crate::component::spinner::FullScreenSpinner;
use crate::consts::ICPUMP_LISTING_PAGE_SIZE;
use crate::utils::token::icpump::get_paginated_token_list;
use crate::utils::token::icpump::TokenListItem;

#[component]
pub fn TokenListing(details: TokenListItem) -> impl IntoView {
    view! {
        <div class="relative w-full md:basis-1/3 xl:basis-1/3">
            <div class="relative rounded-md m-2">
                <div class="flex flex-row justify-center">
                    <img class="w-16 h-16" src={details.logo} alt={details.token_name.clone()}/>
                    <div class="flex flex-col gap-1">
                        <span class="text-xs text-gray-500">"Created by: " {details.user_id}</span>
                        <span class="text-sm text-gray-500">"Name: " {details.token_name}</span>
                        <span class="text-sm text-gray-500 font-bold">"$ " {details.token_symbol}</span>
                        <span class="text-sm text-gray-500">{details.description}</span>
                        <span class="text-xs text-gray-500">{details.created_at}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ICPumpListing() -> impl IntoView {
    let page = create_rw_signal(1);
    let token_list: RwSignal<Vec<TokenListItem>> = create_rw_signal(vec![]);
    let end_of_list = create_rw_signal(false);

    let act = create_resource(page, move |_| async move {
        get_paginated_token_list(page.get_untracked())
            .await
            .unwrap()
    });

    view! {

        <div class="flex flex-col justify-center mt-10 mb-10">
            <Suspense fallback=FullScreenSpinner>
                {move || {
                    let _ = act.get().map(|res| {
                            if res.len() < ICPUMP_LISTING_PAGE_SIZE {
                                end_of_list.set(true);
                            }
                            update!(move |token_list| {
                                *token_list = res.clone();
                            });
                    });

                    view!{
                            <div class="flex flex-row gap-y-3 flex-wrap justify-center w-full">
                                <For
                                    each=move || token_list.get()
                                    key=|t| t.token_symbol.clone()
                                    children=move |token: TokenListItem| {
                                    view! {
                                        <TokenListing details=token />
                                    }
                                    }
                                />
                            </div>

                            <div class="flex flex-row justify-center mt-5">
                                <button on:click={
                                    move |_| {
                                        page.update(|page| *page -= 1);
                                        end_of_list.set(false);
                                    }
                                }
                                disabled=move || page.get()==1> {"[ << ]"} </button>
                                <span class="mx-2"> {page} </span>
                                <button on:click={
                                    move |_| {
                                        page.update(|page| *page += 1);
                                    }
                                }
                                disabled=move || end_of_list.get()
                                > {"[ >> ]"} </button>
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
            <div class="flex ml-4 space-x-2">
                <div class="text-white"> <a href="https://twitter.com/Yral_app" target="_blank"> [twitter] </a> </div>
                <div class="text-white"> <a href="https://www.instagram.com/yral_app/" target="_blank"> [instagram] </a> </div>
                <div class="text-white"> <a href="https://t.me/+c-LTX0Cp-ENmMzI1" target="_blank"> [telegram] </a> </div>
            </div>
            <div class="flex justify-center items-center">
                <div class="font-bold text-3xl hover:font-extrabold"> <a href="/your-profile?tab=tokens"> [start a new coin] </a> </div>
            </div>
            <ICPumpListing />
        </div>
    }
}
