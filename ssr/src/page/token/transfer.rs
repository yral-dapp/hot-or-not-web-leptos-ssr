use crate::page::token::RootType;
use crate::utils::token::icpump::IcpumpTokenInfo;
use crate::utils::{send_wrap, MockPartialEq};
use crate::{
    component::{back_btn::BackButton, spinner::FullScreenSpinner, title::Title},
    state::canisters::authenticated_canisters,
    utils::{
        event_streaming::events::TokensTransferred,
        web::{copy_to_clipboard, paste_from_clipboard},
    },
};
use candid::Principal;
use leptos::either::Either;
use leptos::{ev, html, prelude::*};
use leptos_icons::*;
use leptos_router::{components::*, hooks::use_params};
use leptos_use::use_event_listener;
use server_fn::codec::Json;
use yral_canisters_client::sns_root::ListSnsCanistersArg;
use yral_canisters_common::utils::token::balance::TokenBalance;
use yral_canisters_common::utils::token::TokenMetadata;
use yral_canisters_common::{Canisters, CanistersAuthWire};

use super::{popups::TokenTransferPopup, TokenParams};

#[server(
    input = Json
)]
async fn transfer_token_to_user_principal(
    cans_wire: CanistersAuthWire,
    destination_principal: Principal,
    ledger_canister: Principal,
    root_canister: Principal,
    amount: TokenBalance,
) -> Result<(), ServerFnError> {
    let cans = Canisters::from_wire(cans_wire, expect_context())?;
    cans.transfer_token_to_user_principal(
        destination_principal,
        ledger_canister,
        root_canister,
        amount,
    )
    .await?;

    Ok(())
}

async fn transfer_ck_token_to_user_principal(
    cans_wire: CanistersAuthWire,
    destination_principal: Principal,
    ledger_canister: Principal,
    amount: TokenBalance,
) -> Result<(), ServerFnError> {
    let cans = Canisters::from_wire(cans_wire, expect_context())?;
    cans.transfer_ck_token_to_user_principal(destination_principal, ledger_canister, amount)
        .await?;

    Ok(())
}

#[component]
fn FormError<V: 'static + Send + Sync>(
    #[prop(into)] res: Signal<Result<V, String>>,
) -> impl IntoView {
    let err = Signal::derive(move || res.with(|r| r.as_ref().err().cloned()));

    view! {
        <Show when=move || res.with(|r| r.is_err())>
            <div class="flex flex-row items-center gap-1 w-full text-sm md:text-base">
                <Icon class="text-red-600" icon=icondata::AiInfoCircleOutlined />
                <span class="text-white/60">{move || err().unwrap()}</span>
            </div>
        </Show>
    }
}

#[component]
fn TokenTransferInner(
    cans_wire: CanistersAuthWire,
    root: RootType,
    info: TokenMetadata,
) -> impl IntoView {
    let source_addr = cans_wire.profile_details.principal;
    let copy_source = move || {
        let _ = copy_to_clipboard(&source_addr.to_string());
    };

    let destination_ref = NodeRef::<html::Input>::new();
    // TODO: switch to Action::new_local once https://github.com/leptos-rs/leptos/pull/3310 is released
    let paste_destination: Action<_, _, LocalStorage> = Action::new_unsync(move |&()| async move {
        let input = destination_ref.get()?;
        let principal = paste_from_clipboard().await?;
        input.set_value(&principal);
        #[cfg(feature = "hydrate")]
        {
            use web_sys::InputEvent;
            _ = input.dispatch_event(&InputEvent::new("input").unwrap());
        }
        Some(())
    });

    let destination_res = RwSignal::new(Ok::<_, String>(None::<Principal>));
    _ = use_event_listener(destination_ref, ev::input, move |_| {
        let Some(input) = destination_ref.get() else {
            return;
        };
        let principal_raw = input.value();
        let principal_res =
            Principal::from_text(principal_raw).map_err(|_| "Invalid principal".to_string());
        destination_res.set(principal_res.map(Some));
    });

    let amount_ref = NodeRef::<html::Input>::new();
    let Some(balance) = info.balance else {
        return Either::Left(view! {
            <div>
                <Redirect path="/" />
            </div>
        });
    };

    let max_amt = if balance
        .map_balance_ref(|b| b > &info.fees)
        .unwrap_or_default()
    {
        balance
            .map_balance_ref(|b| b.clone() - info.fees.clone())
            .unwrap()
    } else {
        TokenBalance::new(0u32.into(), info.decimals)
    };
    let max_amt_c = max_amt.clone();
    let set_max_amt = move || {
        let input = amount_ref.get()?;
        input.set_value(&max_amt.humanize_float());
        #[cfg(feature = "hydrate")]
        {
            use web_sys::InputEvent;
            _ = input.dispatch_event(&InputEvent::new("input").unwrap());
        }
        Some(())
    };

    let amt_res = RwSignal::new(Ok::<_, String>(None::<TokenBalance>));
    _ = use_event_listener(amount_ref, ev::input, move |_| {
        let Some(input) = amount_ref.get() else {
            return;
        };
        let amt_raw = input.value();
        let Ok(amt) = TokenBalance::parse(&amt_raw, info.decimals) else {
            amt_res.set(Err("Invalid amount".to_string()));
            return;
        };
        if amt > max_amt_c {
            amt_res.set(Err(
                "Sorry, there are not enough funds in this account".to_string()
            ));
        } else if amt.e8s == 0_u64 {
            amt_res.set(Err("Cannot send 0 tokens".to_string()));
        } else {
            amt_res.set(Ok(Some(amt)));
        }
    });

    let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
        .expect("expected cans_wire to be valid");
    let send_action: Action<_, _, LocalStorage> = Action::new_unsync(move |&()| {
        let cans = cans.clone();
        let auth_cans_wire = cans_wire.clone();

        let root = root.clone();
        async move {
            let destination = destination_res.get_untracked().unwrap().unwrap();
            let amt = amt_res.get_untracked().unwrap().unwrap();

            match root {
                RootType::Other(root) => {
                    let root_canister = cans.sns_root(root).await;
                    println!("{}", root);
                    let sns_cans = root_canister
                        .list_sns_canisters(ListSnsCanistersArg {})
                        .await
                        .unwrap();
                    let ledger_canister = sns_cans.ledger.unwrap();
                    log::debug!("ledger_canister: {:?}", ledger_canister);

                    transfer_token_to_user_principal(
                        auth_cans_wire,
                        destination,
                        ledger_canister,
                        root,
                        amt.clone(),
                    )
                    .await?;
                }
                RootType::BTC { ledger, .. } => {
                    transfer_ck_token_to_user_principal(
                        auth_cans_wire,
                        destination,
                        ledger,
                        amt.clone(),
                    )
                    .await?;
                }
                RootType::USDC { ledger, .. } => {
                    transfer_ck_token_to_user_principal(
                        auth_cans_wire,
                        destination,
                        ledger,
                        amt.clone(),
                    )
                    .await?;
                }
            }
            TokensTransferred.send_event(amt.e8s.to_string(), destination, cans.clone());

            Ok::<_, ServerFnError>(amt)
        }
    });
    let sending = send_action.pending();

    let valid = move || {
        amt_res.with(|r| matches!(r, Ok(Some(_))))
            && destination_res.with(|r| matches!(r, Ok(Some(_))))
            && !sending()
    };

    Either::Right(view! {
        <div class="w-dvw min-h-dvh bg-neutral-800 flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback="/wallet" />
                    <span class="font-bold justify-self-center">Send {info.name}</span>
                </div>
            </Title>
            <div class="flex flex-col w-full gap-4 md:gap-6 items-center p-4">
                <div class="flex flex-col w-full gap-2 items-center">
                    <div class="flex flex-row justify-between w-full text-sm md:text-base text-white">
                        <span>Source:</span>
                        <span>{format!("{} {}", balance.humanize_float(), info.symbol)}</span>
                    </div>
                    <div class="flex flex-row gap-2 w-full items-center">
                        <p class="text-sm md:text-md text-white/80">{source_addr.to_string()}</p>
                        <button on:click=move |_| copy_source()>
                            <Icon
                                class="text-white text-lg md:text-xl"
                                icon=icondata::FaCopyRegular
                            />
                        </button>
                    </div>
                </div>
                <div class="flex flex-col w-full gap-1">
                    <span class="text-white text-sm md:text-base">Destination</span>
                    <div
                        class=("border-white/15", move || destination_res.with(|r| r.is_ok()))
                        class=("border-red", move || destination_res.with(|r| r.is_err()))
                        class="flex flex-row gap-2 w-full justify-between p-3 bg-white/5 rounded-lg border"
                    >
                        <input
                            node_ref=destination_ref
                            class="text-white bg-transparent w-full text-base md:text-lg placeholder-white/40 focus:outline-none"
                        />
                        <button on:click=move |_| { paste_destination.dispatch(()); }>
                            <Icon
                                class="text-neutral-600 text-lg md:text-xl"
                                icon=icondata::BsClipboard
                            />
                        </button>
                    </div>
                    <FormError res=destination_res />
                </div>
                <div class="flex flex-col w-full gap-1">
                    <div class="flex flex-row justify-between w-full text-sm md:text-base text-white">
                        <span>Amount</span>
                        <button
                            class="flex flex-row gap-1 items-center"
                            on:click=move |_| _ = set_max_amt()
                        >
                            <Icon icon=icondata::AiEnterOutlined />
                            " Max"
                        </button>
                    </div>
                    <input
                        node_ref=amount_ref
                        class=("border-white/15", move || amt_res.with(|r| r.is_ok()))
                        class=("border-red", move || amt_res.with(|r| r.is_err()))
                        class="w-full p-3 bg-white/5 rounded-lg border text-white placeholder-white/40 focus:outline-none text-base md:text-lg"
                    />
                    <FormError res=amt_res />
                </div>
                <div class="flex flex-col text-sm md:text-base text-white/60 w-full">
                    <span>Transaction Fee (billed to source)</span>
                    <span>{format!("{} {}", info.fees.humanize_float(), info.symbol)}</span>
                </div>
                <button
                    on:click=move |_| { send_action.dispatch(()); }
                    disabled=move || !valid()
                    class="flex flex-row justify-center text-white md:text-lg w-full md:w-1/2 rounded-full p-3 bg-primary-600 disabled:opacity-50"
                >
                    Send
                </button>
            </div>
            <TokenTransferPopup token_name=info.symbol transfer_action=send_action />
        </div>
    })
}

#[component]
pub fn TokenTransfer() -> impl IntoView {
    let params = use_params::<TokenParams>();

    let auth_cans = authenticated_canisters();
    let token_metadata_fetch = Resource::new(
        move || {
            auth_cans.track();
            (MockPartialEq(()), params.get())
        },
        move |(_, params)| async move {
            let Ok(params) = params else {
                return Ok::<_, ServerFnError>(None);
            };
            let cans_wire = auth_cans.await?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())?;

            let meta = send_wrap(cans.token_metadata_by_root_type(
                &IcpumpTokenInfo,
                Some(cans.user_principal()),
                params.token_root.clone(),
            ))
            .await
            .ok()
            .flatten();

            Ok(meta.map(|m| (cans_wire, m, params.token_root)))
        },
    );

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || Suspend::new(async move {
                let res = token_metadata_fetch.await;
                match res {
                    Err(e) => {
                        Either::Left(view! {
                            <Redirect path=format!("/error?err={e}") />
                        })
                    },
                    Ok(None) => Either::Left(view! { <Redirect path="/" /> }),
                    Ok(Some((cans_wire, info, root))) => Either::Right(view! {
                        <TokenTransferInner cans_wire info root/>
                    })
                }
            })}
        </Suspense>
    }
}
