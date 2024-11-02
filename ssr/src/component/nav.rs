use crate::{consts::USER_PRINCIPAL_STORE, state::{auth::account_connected_reader, canisters::auth_canisters_store}, utils::{event_streaming::events::BaseEvent, host::show_cdao_page}};

use super::nav_icons::*;
use candid::Principal;
use codee::string::FromToStringCodec;
use leptos::*;
use leptos::ev::MouseEvent;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::use_cookie;

#[component]
fn NavIcon<F: Fn(MouseEvent) + 'static>(
    on_click: F,
    idx: usize,
    #[prop(into)] href: MaybeSignal<String>,
    #[prop(into)] icon: icondata_core::Icon,
    #[prop(optional)] filled_icon: Option<icondata_core::Icon>,
    cur_selected: Memo<usize>,
) -> impl IntoView {
    view! {
        <a href=href class="flex justify-center items-center" on:click=on_click>
            <Show
                when=move || cur_selected() == idx
                fallback=move || {
                    view! {
                        <div class="py-5">
                            <Icon icon=icon class="text-2xl text-white md:text-3xl" />
                        </div>
                    }
                }
            >

                <div class="py-5 border-t-2 border-t-pink-500">
                    <Icon
                        icon=filled_icon.unwrap_or(icon)
                        class="text-2xl text-white md:text-3xl aspect-square"
                    />
                </div>
            </Show>
        </a>
    }
}

// #[component]
// fn TrophyIcon(idx: usize, cur_selected: Memo<usize>) -> impl IntoView {
//     view! {
//         <a href="/leaderboard" class="flex justify-center items-center">
//             <Show
//                 when=move || cur_selected() == idx
//                 fallback=move || {
//                     view! {
//                         <div class="py-5">
//                             <Icon icon=TrophySymbol class="text-2xl text-white md:text-3xl fill-none"/>
//                         </div>
//                     }
//                 }
//             >
//
//                 <div class="py-5 border-t-2 border-t-pink-500">
//                     <Icon
//                         icon=TrophySymbolFilled
//                         class="text-2xl text-white md:text-3xl fill-none aspect-square"
//                     />
//                 </div>
//             </Show>
//         </a>
//     }
// }

#[component]
fn UploadIcon(idx: usize, cur_selected: Memo<usize>) -> impl IntoView {
    view! {
        <a href="/upload" class="flex justify-center items-center text-white rounded-full">
            <Show
                when=move || cur_selected() == idx
                fallback=move || {
                    view! {
                        <Icon
                            icon=icondata::AiPlusOutlined
                            class="p-2 w-10 h-10 bg-transparent rounded-full border-2"
                        />
                    }
                }
            >

                <div class="border-t-2 border-transparent">
                    <Icon
                        icon=icondata::AiPlusOutlined
                        class="p-2 w-10 h-10 rounded-full bg-primary-600 aspect-square"
                    />
                    <div class="absolute bottom-0 w-10 bg-primary-600 blur-md"></div>
                </div>
            </Show>
        </a>
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let cur_location = use_location();
    let home_path = create_rw_signal("/".to_string());
    let (user_principal, _) = use_cookie::<Principal, FromToStringCodec>(USER_PRINCIPAL_STORE);
    let cur_selected = create_memo(move |_| {
        let path = cur_location.pathname.get();

        match path.as_str() {
            "/" => 0,
            // "/leaderboard" => 1,
            "/upload" => 2,
            "/transactions" => 3,
            "/menu" | "/leaderboard" => 4,
            "/board" => 0,
            s if s.starts_with("/hot-or-not") => {
                home_path.set(path);
                0
            }
            s if s.starts_with("/profile/") => match user_principal.get() {
                Some(user_principal) => {
                    if s.starts_with(&format!("/profile/{}", user_principal)) {
                        5
                    } else {
                        6 // having a number out of range to not highlight anything
                    }
                }
                None => 0,
            },
            s if s.starts_with("/wallet/") => match user_principal.get() {
                Some(user_principal) => {
                    if s.starts_with(&format!("/wallet/{}", user_principal)) {
                        3
                    } else {
                        6 // having a number out of range to not highlight anything
                    }
                }
                None => 0,
            },
            s if s.starts_with("/profile") => 5,
            s if s.starts_with("/wallet") => 3, // highlights during redirects
            s if s.starts_with("/token/info") => 3,
            s if s.starts_with("/token/create") => 2,
            s if s.starts_with("/icpump-ai") => 5,
            _ => 4,
        }
    });

    let show_cdao_icon = show_cdao_page();

    let (logged_in, _) = account_connected_reader();
    let canister_store = auth_canisters_store();

    let home_click = move |_| {
        BaseEvent.send_event("navigation_home".to_string(), logged_in, canister_store);
    };
    let wallet_click = move |_| {
        BaseEvent.send_event("navigation_wallet".to_string(), logged_in, canister_store);
    };
    let icpumpai_click = move |_| {
        BaseEvent.send_event("navigation_ICPumpAI".to_string(), logged_in, canister_store);
    };
    let menu_click = move |_| {
        BaseEvent.send_event("navigation_menu".to_string(), logged_in, canister_store);
    };

    view! {
    <Suspense>
        <div class="flex fixed bottom-0 left-0 z-50 flex-row justify-between items-center px-6 w-full bg-black/80">
            <NavIcon
                on_click=home_click
                idx=0
                href=home_path
                icon=HomeSymbol
                filled_icon=HomeSymbolFilled
                cur_selected=cur_selected
            />
            <NavIcon
                on_click=wallet_click
                idx=3
                href="/wallet"
                icon=WalletSymbol
                filled_icon=WalletSymbolFilled
                cur_selected=cur_selected
            />
            <UploadIcon idx=2 cur_selected />

            {
                move || {
                    if show_cdao_icon {
                        view! {
                            <NavIcon
                                on_click=icpumpai_click
                                idx=5
                                href="/icpump-ai"
                                icon=ICPumpAiIcon
                                cur_selected=cur_selected
                            />
                        }
                    } else {
                        view! {
                            <NavIcon
                                on_click=|_|()
                                idx=5
                                href="/profile/tokens"
                                icon=ProfileIcon
                                filled_icon=ProfileIconFilled
                                cur_selected=cur_selected
                            />
                        }
                    }
                }
            }

            <NavIcon on_click=menu_click idx=4 href="/menu" icon=MenuSymbol cur_selected=cur_selected />
        </div>

    </Suspense>
    }
}
