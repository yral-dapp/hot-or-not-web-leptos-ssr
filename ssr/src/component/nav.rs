use crate::{consts::USER_PRINCIPAL_STORE, state::app_type::AppType};

use super::nav_icons::*;
use candid::Principal;
use codee::string::FromToStringCodec;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::use_cookie;

#[derive(Clone)]
struct NavItem {
    render_data: NavItemRenderData,
    cur_selected: Signal<bool>,
}

#[derive(Debug, Clone)]
enum NavItemRenderData {
    Icon {
        icon: icondata_core::Icon,
        filled_icon: Option<icondata_core::Icon>,
        href: MaybeSignal<String>,
    },
    Upload,
}

fn pnd_nav_items() -> Vec<NavItem> {
    let cur_location = use_location();
    let path = cur_location.pathname;
    let (user_principal, _) = use_cookie::<Principal, FromToStringCodec>(USER_PRINCIPAL_STORE);
    let home_path = create_rw_signal("/".to_string());
    vec![
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: HomeSymbol,
                filled_icon: Some(HomeSymbolFilled),
                href: home_path.into(),
            },
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: TokenSymbol,
                filled_icon: Some(TokenSymbolFilled),
                href: "/board".into(),
            },
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/board")),
        },
        NavItem {
            render_data: NavItemRenderData::Upload,
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/token/create")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: WalletSymbol,
                filled_icon: Some(WalletSymbolFilled),
                href: "/wallet".into(),
            },
            cur_selected: Signal::derive(move || {
                if path.get().starts_with("/pnd/withdraw") {
                    return true;
                }
                // is selected only if the user is viewing their own wallet
                let Some(user_principal) = user_principal.get() else {
                    return false;
                };
                path.get().starts_with(&format!("/wallet/{user_principal}"))
            }),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: MenuSymbol,
                filled_icon: None,
                href: "/menu".into(),
            },
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/menu")),
        },
    ]
}

fn yral_nav_items() -> Vec<NavItem> {
    let cur_location = use_location();
    let path = cur_location.pathname;
    let (user_principal, _) = use_cookie::<Principal, FromToStringCodec>(USER_PRINCIPAL_STORE);
    let home_path = create_rw_signal("/".to_string());
    vec![
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: HomeSymbol,
                filled_icon: Some(HomeSymbolFilled),
                href: home_path.into(),
            },
            cur_selected: Signal::derive(move || {
                matches!(path.get().as_str(), "/") || path.get().contains("/hot-or-not")
            }),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: WalletSymbol,
                filled_icon: Some(WalletSymbolFilled),
                href: "/wallet".into(),
            },
            cur_selected: Signal::derive(move || {
                // is selected only if the user is viewing their own wallet
                let Some(user_principal) = user_principal.get() else {
                    return false;
                };
                path.get().starts_with(&format!("/wallet/{user_principal}"))
            }),
        },
        NavItem {
            render_data: NavItemRenderData::Upload,
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/upload")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: ProfileIcon,
                filled_icon: Some(ProfileIconFilled),
                href: "/profile/token".into(),
            },
            cur_selected: Signal::derive(move || {
                // is selected only if the user is viewing their own profile
                let Some(user_principal) = user_principal.get() else {
                    return false;
                };
                path.get()
                    .starts_with(&format!("/profile/{user_principal}"))
            }),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: MenuSymbol,
                filled_icon: None,
                href: "/menu".into(),
            },
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/menu")),
        },
    ]
}

fn icpump_nav_items() -> Vec<NavItem> {
    let cur_location = use_location();
    let path = cur_location.pathname;
    let (user_principal, _) = use_cookie::<Principal, FromToStringCodec>(USER_PRINCIPAL_STORE);
    let home_path = create_rw_signal("/".to_string());
    vec![
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: HomeSymbol,
                filled_icon: Some(HomeSymbolFilled),
                href: home_path.into(),
            },
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/board")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: WalletSymbol,
                filled_icon: Some(WalletSymbolFilled),
                href: "/wallet".into(),
            },
            cur_selected: Signal::derive(move || {
                // is selected only if the user is viewing their own wallet
                let Some(user_principal) = user_principal.get() else {
                    return false;
                };
                path.get().starts_with(&format!("/wallet/{user_principal}"))
            }),
        },
        NavItem {
            render_data: NavItemRenderData::Upload,
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/token/create")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: ICPumpAiIcon,
                filled_icon: None,
                href: "/icpump-ai".into(),
            },
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/icpump-ai")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: MenuSymbol,
                filled_icon: None,
                href: "/menu".into(),
            },
            cur_selected: Signal::derive(move || matches!(path.get().as_str(), "/menu")),
        },
    ]
}

fn get_nav_items() -> Vec<NavItem> {
    match AppType::select() {
        AppType::YRAL | AppType::HotOrNot => yral_nav_items(),
        AppType::ICPump => icpump_nav_items(),
        AppType::Pumpdump => pnd_nav_items(),
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let items = get_nav_items();

    view! {
        <Suspense>
            <div class="flex fixed bottom-0 left-0 z-50 flex-row justify-between items-center px-6 md:px-44 w-full bg-black/80">
                {items.iter().map(|item| {
                    let cur_selected = item.cur_selected;
                    match item.render_data.clone() {
                        NavItemRenderData::Icon { icon, filled_icon, href } => view! {
                            <NavIcon href icon filled_icon cur_selected />
                        },
                        NavItemRenderData::Upload => view! {
                            <UploadIcon cur_selected />
                        },
                    }
                }).collect::<Vec<_>>()}
            </div>
        </Suspense>
    }
}

#[component]
fn NavIcon(
    #[prop(into)] href: MaybeSignal<String>,
    #[prop(into)] icon: icondata_core::Icon,
    #[prop(into)] filled_icon: Option<icondata_core::Icon>,
    #[prop(into)] cur_selected: Signal<bool>,
) -> impl IntoView {
    view! {
        <a href=href class="flex justify-center items-center">
            <Show
                when=move || cur_selected()
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

#[component]
fn UploadIcon(#[prop(into)] cur_selected: Signal<bool>) -> impl IntoView {
    view! {
        <a href="/upload" class="flex justify-center items-center text-white rounded-full">
            <Show
                when=move || cur_selected()
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
