use crate::{
    canister::individual_user_template::Result22,
    component::{
        back_btn::BackButton, canisters_prov::WithAuthCans, spinner::FullScreenSpinner,
        title::Title,
    },
    state::canisters::Canisters,
    utils::{
        token::{token_metadata_by_root, TokenMetadata},
        web::{copy_to_clipboard, paste_from_clipboard},
    },
};
use candid::{Nat, Principal};
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::use_event_listener;

use super::TokenParams;

#[component]
fn FormError<V: 'static>(#[prop(into)] res: Signal<Result<V, String>>) -> impl IntoView {
    let err = Signal::derive(move || res.with(|r| r.as_ref().err().cloned()));

    view! {
        <Show when=move || res.with(|r| r.is_err())>
            <div class="flex flex-row items-center gap-1 w-full text-sm md:text-base">
                <Icon class="text-red-600" icon=icondata::AiInfoCircleOutlined/>
                <span class="text-white/60">{move || err().unwrap()}</span>
            </div>
        </Show>
    }
}

#[component]
fn TokenTransferInner(
    cans: Canisters<true>,
    root: Principal,
    info: TokenMetadata,
) -> impl IntoView {
    let source_addr = cans.user_principal();
    let copy_source = move || {
        let _ = copy_to_clipboard(&source_addr.to_string());
    };

    let destination_ref = create_node_ref::<html::Input>();
    let paste_destination = create_action(move |&()| async move {
        let input = destination_ref()?;
        let principal = paste_from_clipboard().await?;
        input.set_value(&principal);
        Some(())
    });

    let destination_res = create_rw_signal(Ok::<_, String>(None::<Principal>));
    _ = use_event_listener(destination_ref, ev::input, move |_| {
        let Some(input) = destination_ref() else {
            return;
        };
        let principal_raw = input.value();
        let principal_res =
            Principal::from_text(principal_raw).map_err(|_| "Invalid principal".to_string());
        destination_res.set(principal_res.map(Some));
    });

    let amount_ref = create_node_ref::<html::Input>();
    let max_amt = if info.balance < info.fees {
        0u32.into()
    } else {
        info.balance.clone() - info.fees.clone()
    };
    let max_amt_c = max_amt.clone();
    let set_max_amt = move || {
        let input = amount_ref()?;
        input.set_value(&max_amt.to_string());
        Some(())
    };

    let amt_res = create_rw_signal(Ok::<_, String>(None::<Nat>));
    _ = use_event_listener(amount_ref, ev::input, move |_| {
        let Some(input) = amount_ref() else {
            return;
        };
        let amt_raw = input.value();
        let Ok(amt) = Nat::parse(amt_raw.as_bytes()) else {
            amt_res.set(Err("Invalid amount".to_string()));
            return;
        };
        if amt > max_amt_c {
            amt_res.set(Err(
                "Sorry, there are not enough funds in this account".to_string()
            ));
        } else {
            amt_res.set(Ok(Some(amt)));
        }
    });

    let send_action = create_action(move |&()| {
        let cans = cans.clone();
        async move {
            let destination = destination_res.get_untracked().unwrap().unwrap();
            let amt = amt_res.get_untracked().unwrap().unwrap();

            let user = cans.authenticated_user().await;
            let res = user
                .transfer_token_to_user_canister(root, destination, None, amt)
                .await
                .map_err(|e| e.to_string())?;
            if let Result22::Err(e) = res {
                return Err(format!("{e:?}"));
            }

            Ok(())
        }
    });

    let valid = move || {
        amt_res.with(|r| matches!(r, Ok(Some(_))))
            && destination_res.with(|r| matches!(r, Ok(Some(_))))
    };

    view! {
        <div class="w-dvw min-h-dvh bg-neutral-800 flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback="/wallet"/>
                    <span class="font-bold justify-self-center">Send {info.name}</span>
                </div>
            </Title>
            <div class="flex flex-col w-full gap-4 md:gap-6 items-center p-4">
                <div class="flex flex-col w-full gap-2 items-center">
                    <div class="flex flex-row justify-between w-full text-sm md:text-base text-white">
                        <span>Source:</span>
                        <span>{format!("{} {}", info.balance, info.symbol)}</span>
                    </div>
                    <div class="flex flex-row gap-2 w-full items-center">
                        <p class="text-sm md:text-md text-white/80">{source_addr.to_string()}</p>
                        <button on:click=move |_| copy_source()>
                            <Icon class="text-white text-lg md:text-xl" icon=icondata::FaCopyRegular/>
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
                        <input _ref=destination_ref class="text-white bg-transparent w-full text-base md:text-lg placeholder-white/40 focus:outline-none"/>
                        <button on:click=move |_| paste_destination.dispatch(())>
                            <Icon class="text-neutral-600 text-lg md:text-xl" icon=icondata::BsClipboard/>
                        </button>
                    </div>
                    <FormError res=destination_res />
                </div>
                <div class="flex flex-col w-full gap-1">
                    <div class="flex flex-row justify-between w-full text-sm md:text-base text-white">
                        <span>Amount</span>
                        <button class="flex flex-row gap-1 items-center" on:click=move |_| _ = set_max_amt()><Icon icon=icondata::AiEnterOutlined/> " Max"</button>
                    </div>
                    <input
                        _ref=amount_ref
                        class=("border-white/15", move || amt_res.with(|r| r.is_ok()))
                        class=("border-red", move || amt_res.with(|r| r.is_err()))
                        class="w-full p-3 bg-white/5 rounded-lg border text-white placeholder-white/40 focus:outline-none text-base md:text-lg"
                    />
                    <FormError res=amt_res/>
                </div>
                <div class="flex flex-col text-sm md:text-base text-white/60 w-full">
                    <span>Transaction Fee (billed to source)</span>
                    <span>{format!("{} {}", info.fees, info.symbol)}</span>
                </div>
                <button on:click=move |_| send_action.dispatch(()) disabled=move || !valid() class="flex flex-row justify-center text-white md:text-lg w-full md:w-1/2 rounded-full p-3 bg-primary-600 disabled:opacity-50">
                    Send
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn TokenTransfer() -> impl IntoView {
    let params = use_params::<TokenParams>();

    let token_metadata_fetch = move |cans: Canisters<true>| {
        create_resource(params, move |params| {
            let cans = cans.clone();
            async move {
                let Ok(params) = params else {
                    return Ok::<_, ServerFnError>(None);
                };
                let user = cans.user_canister();
                let meta = token_metadata_by_root(&cans, user, params.token_root).await?;
                Ok(meta.map(|m| (m, params.token_root)))
            }
        })
    };

    view! {
        <WithAuthCans fallback=FullScreenSpinner with=token_metadata_fetch children=|(cans, res)| {
            match res {
                Err(e) => view! { <Redirect path=format!("/error?err={e}") /> },
                Ok(None) => view! { <Redirect path="/" /> },
                Ok(Some((info, root))) => view! { <TokenTransferInner cans info root /> },
            }
        }/>
    }
}
