use std::rc::Rc;

use crate::{
    consts::USER_PRINCIPAL_STORE,
    utils::host::{show_cdao_page, show_pnd_page},
};

use super::nav_icons::*;
use candid::Principal;
use codee::string::FromToStringCodec;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::use_cookie;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum SiteHost {
    #[default]
    Yral,
    Pumpdump,
    Icpump,
}

impl SiteHost {
    /// Figures out what site are we are hosting from
    fn decide() -> Self {
        if show_cdao_page() {
            Self::Icpump
        } else if show_pnd_page() {
            Self::Pumpdump
        } else {
            Self::default()
        }
    }
}

#[derive(Clone)]
struct NavItem {
    render_data: NavItemRenderData,
    matcher: Rc<dyn Fn() -> bool>,
}

impl NavItem {
    fn is_selected(&self) -> bool {
        (self.matcher)()
    }

    fn is_upload(&self) -> bool {
        matches!(self.render_data, NavItemRenderData::Upload)
    }
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
            matcher: Rc::new(move || matches!(path.get().as_str(), "/")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: TokenSymbol,
                filled_icon: Some(TokenSymbolFilled),
                href: "/board".into(),
            },
            matcher: Rc::new(move || matches!(path.get().as_str(), "/board")),
        },
        NavItem {
            render_data: NavItemRenderData::Upload,
            matcher: Rc::new(move || matches!(path.get().as_str(), "/token/create")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: WalletSymbol,
                filled_icon: Some(WalletSymbolFilled),
                href: "/wallet".into(),
            },
            matcher: Rc::new(move || {
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
            matcher: Rc::new(move || matches!(path.get().as_str(), "/menu")),
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
            matcher: Rc::new(move || matches!(path.get().as_str(), "/")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: WalletSymbol,
                filled_icon: Some(WalletSymbolFilled),
                href: "/wallet".into(),
            },
            matcher: Rc::new(move || {
                // is selected only if the user is viewing their own wallet
                let Some(user_principal) = user_principal.get() else {
                    return false;
                };
                path.get().starts_with(&format!("/wallet/{user_principal}"))
            }),
        },
        NavItem {
            render_data: NavItemRenderData::Upload,
            matcher: Rc::new(move || matches!(path.get().as_str(), "/token/create")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: ProfileIcon,
                filled_icon: Some(ProfileIconFilled),
                href: "/profile/token".into(),
            },
            matcher: Rc::new(move || {
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
            matcher: Rc::new(move || matches!(path.get().as_str(), "/menu")),
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
            matcher: Rc::new(move || matches!(path.get().as_str(), "/")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: WalletSymbol,
                filled_icon: Some(WalletSymbolFilled),
                href: "/wallet".into(),
            },
            matcher: Rc::new(move || {
                // is selected only if the user is viewing their own wallet
                let Some(user_principal) = user_principal.get() else {
                    return false;
                };
                path.get().starts_with(&format!("/wallet/{user_principal}"))
            }),
        },
        NavItem {
            render_data: NavItemRenderData::Upload,
            matcher: Rc::new(move || matches!(path.get().as_str(), "/token/create")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: ProfileIcon,
                filled_icon: Some(ProfileIconFilled),
                href: "/icpump-ai".into(),
            },
            matcher: Rc::new(move || matches!(path.get().as_str(), "/icpump-ai")),
        },
        NavItem {
            render_data: NavItemRenderData::Icon {
                icon: MenuSymbol,
                filled_icon: None,
                href: "/menu".into(),
            },
            matcher: Rc::new(move || matches!(path.get().as_str(), "/menu")),
        },
    ]
}

fn get_nav_items() -> Vec<NavItem> {
    match SiteHost::decide() {
        SiteHost::Yral => yral_nav_items(),
        SiteHost::Pumpdump => pnd_nav_items(),
        SiteHost::Icpump => icpump_nav_items(),
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let items = get_nav_items();

    let (items, _) = create_signal(items);
    view! {
        <Suspense>
            <div class="flex fixed bottom-0 left-0 z-50 flex-row justify-between items-center px-6 w-full bg-black/80">
                <For each=move || (0..items.get().len()) key=|item| *item let:item>
                    <Show
                        when=move || items.get()[item].is_upload()
                        fallback=move || {
                            let item = &items.get()[item];
                            let cur_selected = item.is_selected();
                            let NavItemRenderData::Icon { href, icon, filled_icon } = item.render_data.clone() else {
                                unreachable!("as of now, there's no other type available")
                            };
                            view! {
                                <NavIcon href icon filled_icon cur_selected />
                            }
                        }
                    >
                        <UploadIcon cur_selected=items.get().get(item).unwrap().is_selected() />
                    </Show>
                </For>
            </div>
        </Suspense>
    }
}

#[component]
fn NavIcon(
    #[prop(into)] href: MaybeSignal<String>,
    #[prop(into)] icon: icondata_core::Icon,
    #[prop(into)] filled_icon: Option<icondata_core::Icon>,
    #[prop(into)] cur_selected: bool,
) -> impl IntoView {
    view! {
        <a href=href class="flex justify-center items-center">
            <Show
                when=move || cur_selected
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
fn UploadIcon(#[prop(into)] cur_selected: bool) -> impl IntoView {
    view! {
        <a href="/upload" class="flex justify-center items-center text-white rounded-full">
            <Show
                when=move || cur_selected
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
