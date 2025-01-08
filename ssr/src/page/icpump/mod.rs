use crate::component::overlay::PopupOverlay;
use crate::consts::ICPUMP_LISTING_PAGE_SIZE;
use crate::consts::USER_PRINCIPAL_STORE;
use crate::state::canisters::unauth_canisters;
use std::collections::HashMap;
use std::collections::VecDeque;

use candid::Principal;
use codee::string::FromToStringCodec;
use futures::stream::FuturesOrdered;
use futures::StreamExt;
use html::Div;
use leptos::*;
use leptos_icons::Icon;
use leptos_use::use_cookie;
use leptos_use::use_intersection_observer_with_options;
use leptos_use::UseIntersectionObserverOptions;
use serde::Deserialize;
use serde::Serialize;

use crate::component::buttons::HighlightedLinkButton;
use crate::component::icons::airdrop_icon::AirdropIcon;
use crate::component::icons::arrow_left_right_icon::ArrowLeftRightIcon;
use crate::component::icons::chevron_right_icon::ChevronRightIcon;
use crate::component::icons::eye_hide_icon::EyeHiddenIcon;
use crate::component::icons::send_icon::SendIcon;
use crate::component::icons::share_icon::ShareIcon;
use crate::component::share_popup::ShareContent;
use crate::utils::host::get_host;
use crate::utils::token::firestore::init_firebase;
use crate::utils::token::firestore::listen_to_documents;
use crate::utils::token::icpump::get_paginated_token_list;
use crate::utils::token::icpump::TokenListItem;

pub mod ai;
#[derive(Serialize, Deserialize, Clone, Debug)]
struct ProcessedTokenListResponse {
    token_details: TokenListItem,
    root: Principal,
    is_airdrop_claimed: bool,
}

async fn process_token_list_item(
    token_list_item: Vec<TokenListItem>,
    key_principal: Principal,
) -> Vec<ProcessedTokenListResponse> {
    let mut fut = FuturesOrdered::new();

    for token in token_list_item {
        fut.push_back(async move {
            let cans = unauth_canisters();
            let root_principal = Principal::from_text(
                token
                    .link
                    .trim_end_matches('/')
                    .split('/')
                    .last()
                    .ok_or(ServerFnError::new("Not root given"))?,
            )?;

            let token_owner_canister_id = cans.get_token_owner(root_principal).await?;
            let is_airdrop_claimed = if let Some(token_owner) = token_owner_canister_id {
                cans.get_airdrop_status(token_owner.canister_id, root_principal, key_principal)
                    .await?
            } else {
                true
            };
            // let token_owner = cans.individual_user(token_owner_canister_id.unwrap()).await;
            // token_owner_principal_id: token_owner.get_profile_details().await.unwrap().principal_id,

            Ok::<_, ServerFnError>(ProcessedTokenListResponse {
                token_details: token,
                root: root_principal,
                is_airdrop_claimed,
            })
        });
    }

    fut.filter_map(|result| async move { result.ok() })
        .collect()
        .await
}

#[component]
pub fn ICPumpListingFeed() -> impl IntoView {
    let page = create_rw_signal(1);
    let end = create_rw_signal(false);
    let loading = create_rw_signal(true);
    let (curr_principal, _) = use_cookie::<Principal, FromToStringCodec>(USER_PRINCIPAL_STORE);
    let token_list: RwSignal<Vec<ProcessedTokenListResponse>> = create_rw_signal(vec![]);
    let new_token_list: RwSignal<VecDeque<ProcessedTokenListResponse>> =
        create_rw_signal(VecDeque::new());

    let fetch_res = create_resource(
        move || (page(), curr_principal()),
        move |(page, curr_principal)| async move {
            loading.set(true);

            let mut fetched_token_list = process_token_list_item(
                get_paginated_token_list(page).await.unwrap(),
                curr_principal.unwrap(),
            )
            .await;

            if fetched_token_list.len() < ICPUMP_LISTING_PAGE_SIZE {
                end.set(true);
            }

            token_list.update(|t| t.append(&mut fetched_token_list));

            loading.set(false);
        },
    );

    create_effect(move |_| {
        fetch_res.refetch();
        spawn_local(async move {
            let (_app, firestore) = init_firebase();
            let mut stream = listen_to_documents(&firestore);
            let curr_principal = curr_principal.get().unwrap();
            while let Some(doc) = stream.next().await {
                let doc = process_token_list_item(doc, curr_principal).await;
                // push each item in doc to new_token_list
                for item in doc {
                    new_token_list.update(move |list| {
                        list.push_front(item.clone());
                    });
                }
            }
        });
    });

    let target = NodeRef::<Div>::new();

    use_intersection_observer_with_options(
        target,
        move |entries, _| {
            let is_intersecting = entries.get(0).map(|entry| entry.is_intersecting());

            let loading = loading.get_untracked();
            let end = end.get_untracked();

            if let (Some(true), false, false) = (is_intersecting, loading, end) {
                page.update(|p| {
                    *p += ICPUMP_LISTING_PAGE_SIZE as u32;
                });
            }
        },
        UseIntersectionObserverOptions::default().thresholds(vec![0.1]),
    );

    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 ">
            <For
                each=move || new_token_list.get()
                key=|t| t.token_details.token_symbol.clone()
                children=move |t| {
                    view! {
                        <TokenCard
                            is_new_token=true
                            details=t.token_details
                            is_airdrop_claimed=t.is_airdrop_claimed
                            root=t.root
                        />
                    }
                }
            />

            <For
                each=move || token_list.get()
                key=|t| t.token_details.token_symbol.clone()
                children=move |t| {
                    view! {
                        <TokenCard
                            details=t.token_details
                            is_airdrop_claimed=t.is_airdrop_claimed
                            root=t.root
                        />
                    }
                }
            />

            <Show when=move || loading.get()>
                <TokenCardLoading />
                <TokenCardLoading />
                <TokenCardLoading />
                <TokenCardLoading />
                <TokenCardLoading />
                <TokenCardLoading />
            </Show>
        </div>

        <div class="w-full p-4" node_ref=target></div>
    }
}

#[component]
pub fn ICPumpLanding() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-black text-white  flex flex-col gap-4 px-4 md:px-8 py-6 font-kumbh">
            <div class="flex lg:flex-row gap-4 flex-col items-center justify-center">
                <div class="lg:left-0 lg:top-0 flex items-center gap-4">
                    <div>Follow us:</div>
                    <div class="flex items-center gap-4">
                        <XIcon
                            href="https://x.com/Yral_app".to_string()
                            classes="w-10 h-10".to_string()
                        />
                        <InstagramIcon
                            href="https://instagram.com/yral_app".to_string()
                            classes="w-10 h-10".to_string()
                        />
                        <TelegramIcon
                            href="https://t.me/+c-LTX0Cp-ENmMzI1".to_string()
                            classes="w-10 h-10".to_string()
                        />
                    </div>
                </div>
                <HighlightedLinkButton
                    classes="max-w-96 lg:max-w-[32.5%]".to_string()
                    href="/token/create".to_string()
                >
                    "Create a new coin"
                </HighlightedLinkButton>
            </div>
            <div class="flex flex-col gap-8 pb-24">
                <ICPumpListingFeed />
            </div>

        </div>
    }
}

#[component]
pub fn TokenCard(
    details: TokenListItem,
    #[prop(optional, default = false)] is_new_token: bool,
    root: Principal,
    is_airdrop_claimed: bool,
) -> impl IntoView {
    let show_nsfw = create_rw_signal(false);

    let share_link = create_rw_signal("".to_string());
    let share_link_coin = format!("/token/info/{}/{}", root, details.user_id);
    let symbol = details.token_symbol.clone();
    let share_message = move || {
        format!(
        "Hey! Check out the token: {} I created on YRAL 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        details.token_symbol.clone(),
        share_link.get(),
    )
    };
    let pop_up = create_rw_signal(false);
    let base_url = get_host();

    view! {
        <div
            class:tada=is_new_token
            class="flex flex-col gap-2 py-3 px-3 w-full text-xs rounded-lg transition-colors md:px-4 hover:bg-gradient-to-b group bg-neutral-900/90 font-kumbh hover:from-neutral-600 hover:to-neutral-800"
        >
            <div class="flex gap-3 items-stretch">
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
                        class="w-full h-full object-cover"
                    />
                </div>
                <div class="flex flex-col justify-between overflow-hidden w-full">
                    <div class="flex flex-col gap-2">
                        <div class="flex gap-4 justify-between items-center w-full text-lg">
                            <span class="font-medium shrink line-clamp-1">{details.name}</span>
                            <span class="font-bold shrink-0">{symbol}</span>
                        </div>
                        <span class="text-sm line-clamp-2 text-neutral-400">
                            {details.description}
                        </span>
                    </div>
                    <div class="flex gap-2 justify-between items-center text-sm font-medium group-hover:text-white text-neutral-600">
                        <span class="line-clamp-1">"Created by" {details.user_id.clone()}</span>
                        <span class="shrink-0">{details.formatted_created_at}</span>
                    </div>
                </div>
            </div>

            <div class="flex gap-4 justify-between items-center p-2">
                <ActionButton label="Send".to_string() href=format!("/token/transfer/{root}")>
                    <Icon class="w-full h-full" icon=SendIcon />
                </ActionButton>
                <ActionButton label="Buy/Sell".to_string() href="#".to_string() disabled=true>
                    <Icon class="w-full h-full" icon=ArrowLeftRightIcon />
                </ActionButton>
                {if is_airdrop_claimed {
                    view! {
                        <ActionButtonLink
                            on:click=move |_| {
                                pop_up.set(true);
                                share_link
                                    .set(
                                        format!(
                                            "/token/info/{}/{}?airdrop_amt=100",
                                            root,
                                            details.user_id,
                                        ),
                                    )
                            }
                            label="Airdrop".to_string()
                        >
                            <Icon class="h-6 w-6" icon=AirdropIcon />
                        </ActionButtonLink>
                    }
                } else {
                    view! {
                        <ActionButton
                            href=format!("/token/info/{}/{}?airdrop_amt=100", root, details.user_id)
                            label="Airdrop".to_string()
                        >
                            <Icon class="h-6 w-6" icon=AirdropIcon />
                        </ActionButton>
                    }
                }}
                <ActionButton label="Share".to_string() href="#".to_string()>
                    <Icon
                        class="w-full h-full"
                        icon=ShareIcon
                        on:click=move |_| {
                            pop_up.set(true);
                            share_link.set(share_link_coin.clone())
                        }
                    />
                </ActionButton>
                <ActionButton label="Details".to_string() href=details.link>
                    <Icon class="w-full h-full" icon=ChevronRightIcon />
                </ActionButton>
            </div>
            <PopupOverlay show=pop_up>
                <ShareContent
                    share_link=format!("{base_url}{}", share_link())
                    message=share_message()
                    show_popup=pop_up
                />
            </PopupOverlay>
        </div>
    }
}

pub fn TokenCardLoading() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 py-3 px-3 w-full rounded-lg md:px-4 group bg-neutral-900/90">
            <div class="flex gap-3">
                <div class="w-[7rem] h-[7rem] bg-loading rounded-[4px] relative shrink-0"></div>

                <div class="w-full bg-loading rounded-[4px]"></div>
            </div>

            <div class="h-[4.125rem] bg-loading rounded-[4px]"></div>
        </div>
    }
}

#[component]
pub fn PageSelector(page: RwSignal<u64>, end_of_list: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="flex gap-1 items-start text-sm font-medium text-gray-400">
            <button
                class="flex justify-center items-center w-8 h-8 rounded-lg bg-neutral-800"
                on:click=move |_| {
                    page.update(|page| *page -= 1);
                    end_of_list.set(false);
                }
                disabled=move || page.get() == 1
            >
                <Icon class="w-4 h-4 rotate-180" icon=ChevronRightIcon />
            </button>
            <div class="w-8 h-8 rounded-lg flex items-center justify-center text-white bg-blue-500">
                {page}
            </div>
            <button
                class="flex justify-center items-center w-8 h-8 rounded-lg bg-neutral-800"
                on:click=move |_| {
                    page.update(|page| *page += 1);
                }
                disabled=move || end_of_list.get()
            >
                <Icon class="w-4 h-4" icon=ChevronRightIcon />
            </button>
        </div>
    }
}

#[component]
pub fn ActionButton(
    href: String,
    label: String,
    children: Children,
    #[prop(optional, default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <a
            disabled=disabled
            href=href
            class=move || {
                format!(
                    "flex flex-col gap-1 justify-center items-center text-xs transition-colors {}",
                    if !disabled {
                        "group-hover:text-white text-neutral-300"
                    } else {
                        "group-hover:cursor-default text-neutral-600"
                    },
                )
            }
        >
            <div class="w-[1.875rem] h-[1.875rem] flex items-center justify-center">
                {children()}
            </div>

            <div>{label}</div>
        </a>
    }
}

#[component]
pub fn ActionButtonLink(
    label: String,
    children: Children,
    #[prop(optional, default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <button
            disabled=disabled
            class=move || {
                format!(
                    "flex flex-col gap-1 justify-center items-center text-xs transition-colors {}",
                    if !disabled {
                        "group-hover:text-white text-neutral-300"
                    } else {
                        "group-hover:cursor-default text-neutral-600"
                    },
                )
            }
        >
            <div class="w-[1.875rem] h-[1.875rem] flex items-center justify-center">
                {children()}
            </div>

            <div>{label}</div>
        </button>
    }
}

#[component]
pub fn TelegramIcon(href: String, classes: String) -> impl IntoView {
    view! {
        <a href=href target="_blank">
            <svg class=classes viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect
                    width="40"
                    height="40"
                    rx="20"
                    fill="white"
                    style="fill:white;fill-opacity:1;"
                />
                <path
                    d="M29.6778 11.8012L25.7231 28.1008C25.537 28.8685 24.6763 29.264 23.9706 28.8995L18.9691 26.3173L16.604 30.1867C15.9604 31.2413 14.332 30.7838 14.332 29.5509V25.2395C14.332 24.906 14.4716 24.5881 14.712 24.3555L24.4592 15.0503C24.4514 14.934 24.3273 14.8332 24.2033 14.9185L12.5718 23.014L8.66359 20.9978C7.74858 20.5248 7.78735 19.1988 8.73338 18.7879L28.0029 10.3899C28.9257 9.9867 29.9182 10.8164 29.6778 11.8012Z"
                    fill="black"
                    style="fill:black;fill-opacity:1;"
                />
            </svg>

        </a>
    }
}

#[component]
pub fn XIcon(href: String, classes: String) -> impl IntoView {
    view! {
        <a href=href target="_blank">

            <svg class=classes viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect
                    width="40"
                    height="40"
                    rx="20"
                    fill="white"
                    style="fill:white;fill-opacity:1;"
                />
                <path
                    d="M22.0682 18.3383L30.1527 9.14286H28.2372L21.2143 17.1255L15.6092 9.14286H9.14282L17.6208 21.2151L9.14282 30.8571H11.0583L18.4702 22.4252L24.3908 30.8571H30.8571L22.0682 18.3383ZM19.4438 21.3211L18.5835 20.1182L11.7491 10.5559H14.6917L20.2089 18.2758L21.0656 19.4787L28.2363 29.513H25.2937L19.4438 21.3211Z"
                    fill="black"
                    style="fill:black;fill-opacity:1;"
                />
            </svg>

        </a>
    }
}

#[component]
pub fn InstagramIcon(href: String, classes: String) -> impl IntoView {
    view! {
        <a href=href target="_blank">
            <svg class=classes viewBox="0 0 40 40" fill="none" xmlns="http://www.w3.org/2000/svg">
                <rect
                    width="40"
                    height="40"
                    rx="20"
                    fill="white"
                    style="fill:white;fill-opacity:1;"
                />
                <path
                    d="M25.493 9.14286H14.5069C13.0847 9.1442 11.7211 9.70978 10.7154 10.7155C9.70975 11.7211 9.14417 13.0847 9.14282 14.507V25.493C9.14417 26.9153 9.70975 28.2789 10.7154 29.2845C11.7211 30.2902 13.0847 30.8558 14.5069 30.8571H25.493C26.9151 30.8553 28.2784 30.2895 29.2839 29.2839C30.2895 28.2784 30.8552 26.9151 30.8571 25.493V14.507C30.8552 13.0849 30.2895 11.7216 29.2839 10.716C28.2784 9.71049 26.9151 9.14474 25.493 9.14286ZM25.381 15.9964C25.1083 15.9961 24.8417 15.9149 24.6151 15.763C24.3885 15.6112 24.212 15.3956 24.1079 15.1435C24.0038 14.8913 23.9768 14.614 24.0304 14.3465C24.0839 14.079 24.2156 13.8334 24.4087 13.6408C24.6018 13.4481 24.8477 13.3171 25.1153 13.2642C25.3829 13.2113 25.6602 13.239 25.9121 13.3437C26.164 13.4484 26.3792 13.6254 26.5304 13.8524C26.6817 14.0794 26.7623 14.3461 26.7619 14.6189C26.7617 14.8 26.7258 14.9793 26.6563 15.1466C26.5868 15.3138 26.485 15.4658 26.3568 15.5937C26.2285 15.7216 26.0764 15.823 25.909 15.8921C25.7415 15.9612 25.5622 15.9967 25.381 15.9964ZM20 14.8293C21.0226 14.8293 22.0223 15.1325 22.8727 15.7007C23.723 16.2689 24.3857 17.0764 24.7771 18.0213C25.1684 18.9661 25.2708 20.0057 25.0713 21.0088C24.8718 22.0118 24.3793 22.9331 23.6562 23.6562C22.9331 24.3794 22.0117 24.8718 21.0087 25.0714C20.0057 25.2709 18.966 25.1685 18.0212 24.7771C17.0764 24.3858 16.2688 23.723 15.7007 22.8727C15.1325 22.0224 14.8292 21.0227 14.8292 20C14.8312 18.6292 15.3766 17.3152 16.3459 16.3459C17.3152 15.3767 18.6292 14.8313 20 14.8293Z"
                    fill="black"
                    style="fill:black;fill-opacity:1;"
                />
                <path
                    d="M19.9999 23.1112C20.6153 23.1112 21.2168 22.9288 21.7284 22.5869C22.2401 22.245 22.6389 21.7591 22.8743 21.1906C23.1098 20.6221 23.1714 19.9965 23.0514 19.393C22.9313 18.7895 22.635 18.2351 22.1999 17.8C21.7648 17.3649 21.2104 17.0686 20.6069 16.9485C20.0034 16.8285 19.3778 16.8901 18.8093 17.1256C18.2408 17.3611 17.7549 17.7598 17.413 18.2715C17.0711 18.7831 16.8887 19.3846 16.8887 20C16.8896 20.8249 17.2176 21.6157 17.8009 22.199C18.3842 22.7823 19.175 23.1103 19.9999 23.1112Z"
                    fill="black"
                    style="fill:black;fill-opacity:1;"
                />
            </svg>

        </a>
    }
}
