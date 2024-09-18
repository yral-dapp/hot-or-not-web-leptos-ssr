pub mod tokens;
pub mod transactions;
use crate::page::token::popups::ShareProfilePopup;
mod txn;
use crate::{
    component::{
        bullet_loader::BulletLoader,
        canisters_prov::AuthCansProvider,
        connect::ConnectLogin,
        infinite_scroller::{CursoredDataProvider, KeyedData},
    },
    state::{auth::account_connected_reader, canisters::authenticated_canisters},
    try_or_redirect_opt,
    utils::profile::ProfileDetails,
};
use candid::Principal;
use leptos::*;
use leptos_icons::*;
// use std::time::Duration;
use tokens::{TokenRootList, TokenView};
use txn::{provider::get_history_provider, TxnView};

#[component]
fn ProfileGreeter(details: ProfileDetails) -> impl IntoView {
    // let display_name = details.display_name_or_fallback();

    // let (_icon, set_icon) = create_signal(icondata::BsCopy);
    //
    // let copy_to_clipboard = move || {
    //     let display_name_clone = display_name.clone();
    //     let _ = leptos::window()
    //         .navigator()
    //         .clipboard()
    //         .write_text(&display_name_clone);
    //     set_icon(icondata::BsCheckSquare);
    //     leptos::set_timeout(
    //         move || {
    //             set_icon(icondata::BsCopy);
    //         },
    //         Duration::from_millis(2000),
    //     );
    // };
    let share_action = create_action(move |&()| async move { Ok(()) });

    view! {
    <div class="flex flex-col">
        // <span class="text-white/50 text-md">Welcome!</span>
     <div class="flex overflow-hidden justify-center items-center px-10 mx-1 space-x-2 rounded-xl border-2 border-neutral-700 h-[2.5rem] items md:h-[5rem]" >
        <span class="text-lg text-white md:text-xl truncate">
            // TEMP: Workaround for hydration bug until leptos 0.7
            // class=("md:w-5/12", move || !is_connected())
            {details.display_name_or_fallback()}
         </span>
       <button
                        on:click=move |_| share_action.dispatch(())>
                <Icon class="w-6 h-6 text-white cursor-pointer" icon=icondata::ChShare />
            </button>
        </div>
            {move ||
                            view! {
                                <ShareProfilePopup
                                    sharing_action=share_action

                                />

                        } }

    </div>
    }
}

#[component]
fn FallbackGreeter() -> impl IntoView {
    view! {
        <div class="flex flex-col">
            <span class="text-white/50 text-md">Welcome!</span>
            <div class="py-2 w-3/4 rounded-full animate-pulse bg-white/40"></div>
        </div>
        <div class="justify-self-end w-16 rounded-full animate-pulse aspect-square overflow-clip bg-white/40"></div>
    }
}

const RECENT_TXN_CNT: usize = 10;

#[component]
fn BalanceFallback() -> impl IntoView {
    view! { <div class="py-3 mt-1 w-1/4 rounded-full animate-pulse bg-white/30"></div> }
}

#[component]
fn TokensFetch() -> impl IntoView {
    let auth_cans = authenticated_canisters();
    let tokens_fetch = auth_cans.derive(
        || (),
        |cans_wire, _| async move {
            let cans = cans_wire?.canisters()?;
            let user_principal = cans.user_principal();
            let tokens_prov = TokenRootList(cans);
            let tokens = tokens_prov.get_by_cursor(0, 5).await?;
            Ok::<_, ServerFnError>((user_principal, tokens.data))
        },
    );

    view! {
        <Suspense fallback=BulletLoader>
            {move || {
                tokens_fetch()
                    .map(|tokens_res| {
                        let tokens = tokens_res.as_ref().map(|t| t.1.clone()).unwrap_or_default();
                        let user_principal = tokens_res.as_ref().map(|t| t.0).unwrap_or(Principal::anonymous());
                        view! {
                            <For
                                each=move || tokens.clone()
                                key=|inf| inf.key()
                                let:token_root
                            >
                                <TokenView
                                    user_principal
                                    token_root
                                />
                            </For>
                        }
                    })
            }}
        </Suspense>
    }
}

#[component]
pub fn Wallet() -> impl IntoView {
    let (is_connected, _) = account_connected_reader();

    let auth_cans = authenticated_canisters();
    let balance_fetch = auth_cans.derive(
        || (),
        |cans_wire, _| async move {
            let cans = cans_wire?.canisters()?;
            let user = cans.authenticated_user().await;

            let bal = user.get_utility_token_balance().await?;
            Ok::<_, ServerFnError>(bal.to_string())
        },
    );
    let history_fetch = auth_cans.derive(
        || (),
        |cans_wire, _| async move {
            let cans = cans_wire?.canisters()?;
            let history_prov = get_history_provider(cans);
            let page = history_prov.get_by_cursor(0, RECENT_TXN_CNT).await?;

            Ok::<_, ServerFnError>(page.data)
        },
    );

    view! {
        <div>
            <div class="flex flex-col w-dvw min-h-dvh bg-black gap-4 px-4 pt-4 pb-12">
                <div class="grid grid-cols-2 grid-rows-1 items-center w-full">
                    <AuthCansProvider fallback=FallbackGreeter let:cans>
                        <ProfileGreeter details=cans.profile_details()/>
                    </AuthCansProvider>
                </div>
                <div class="flex flex-col items-center mt-6 w-full text-white">
                    <span class="uppercase lg:text-lg text-md">Your Coyns Balance</span>
                    <Suspense fallback=BalanceFallback>
                        {move || {
                            let balance = try_or_redirect_opt!(balance_fetch() ?);
                            Some(view! { <div class="text-xl lg:text-2xl">{balance}</div> })
                        }}

                    </Suspense>
                </div>
                <Show when=move || !is_connected()>
                    <div class="flex flex-col items-center py-5 w-full">
                        <div class="flex flex-row items-center w-9/12 md:w-5/12">
                            <ConnectLogin
                                login_text="Login to claim your COYNs"
                                cta_location="wallet"
                            />
                        </div>
                    </div>
                </Show>
                <div class="flex flex-col gap-2 w-full">
                    <div class="flex flex-row justify-between items-end w-full">
                        <span class="text-sm text-white md:text-md">My Tokens</span>
                        <a href="/tokens" class="md:text-lg text-white/50 text-md">
                            See All
                        </a>
                    </div>
                    <div class="flex flex-col gap-2 items-center">
                        <TokensFetch/>
                    </div>
                </div>
                <div class="flex flex-col gap-2 w-full">
                    <div class="flex flex-row justify-between items-end w-full">
                        <span class="text-sm text-white md:text-md">Recent Transactions</span>
                        <a href="/transactions" class="md:text-lg text-white/50 text-md">
                            See All
                        </a>
                    </div>
                    <div class="flex flex-col divide-y divide-white/10">
                        <Suspense fallback=BulletLoader>
                            {move || {
                                history_fetch()
                                    .map(|history| {
                                        view! {
                                            <For
                                                each=move || history.clone().unwrap_or_default()
                                                key=|inf| inf.key()
                                                let:info
                                            >
                                                <TxnView info/>
                                            </For>
                                        }
                                    })
                            }}

                        </Suspense>
                    </div>
                </div>
            </div>
        </div>
    }
}
