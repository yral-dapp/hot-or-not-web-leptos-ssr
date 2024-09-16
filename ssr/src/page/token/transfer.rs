use crate::{
    canister::{
        sns_ledger::{Account, TransferArg},
        sns_root::ListSnsCanistersArg,
    },
    component::{
        back_btn::BackButton, canisters_prov::WithAuthCans, spinner::FullScreenSpinner,
        title::Title,
    },
    state::canisters::{authenticated_canisters, Canisters, CanistersAuthWire},
    utils::{
        token::{token_metadata_by_root, TokenBalance, TokenMetadata},
        web::{copy_to_clipboard, paste_from_clipboard},
    },
};
use candid::{Nat, Principal};
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::use_event_listener;
use server_fn::codec::Cbor;

use super::{popups::TokenTransferPopup, TokenParams};

#[server(
    input = Cbor
)]
async fn transfer_token_to_user_principal(
    cans_wire: CanistersAuthWire,
    destination_canister: Principal,
    destination_principal: Principal,
    ledger_canister: Principal,
    root_canister: Principal,
    amount: TokenBalance,
) -> Result<(), ServerFnError> {
    let cans = cans_wire.canisters().unwrap();
    // let user_id = user_id.to_owned();
    // let user_principal = user_id.sender()?;
    // let agent = cans.agent.get_agent().await;
    // let user_principal = agent.get_principal()?;
    // log::debug!("user_principal: {:?}", user_principal.to_string());
    let sns_ledger = cans.sns_ledger(ledger_canister).await;
    let res = sns_ledger
        .icrc_1_transfer(TransferArg {
            memo: Some(serde_bytes::ByteBuf::from(vec![0])),
            amount: amount.clone().into(),
            fee: None,
            from_subaccount: None,
            to: Account {
                owner: destination_principal,
                subaccount: None,
            },
            created_at_time: None,
        })
        .await
        .unwrap();
    log::debug!("transfer res: {:?}", res);
    let res = sns_ledger
        .icrc_1_transfer(TransferArg {
            memo: Some(serde_bytes::ByteBuf::from(vec![1])),
            amount: Nat::from(1_u64),
            fee: None,
            from_subaccount: None,
            to: Account {
                owner: destination_canister,
                subaccount: None,
            },
            created_at_time: None,
        })
        .await
        .unwrap();
    log::debug!("transfer res: {:?}", res);

    // let agent = Agent::builder()
    //     .with_url(AGENT_URL)
    //     .with_identity(user_id)
    //     .build()
    //     .unwrap();
    // agent.fetch_root_key().await.unwrap();

    // let transfer_args = types::Transaction {
    //     memo: Some(vec![0]),
    //     amount,
    //     fee: None,
    //     from_subaccount: None,
    //     to: types::Recipient {
    //         owner: destination_principal,
    //         subaccount: None,
    //     },
    //     created_at_time: None,
    // };
    // let res = agent
    //     .update(
    //         &ledger_canister,
    //         "icrc1_transfer",
    //     )
    //     .with_arg(Encode!(&transfer_args).unwrap())
    //     .call_and_wait()
    //     .await
    //     .unwrap();
    // let transfer_result: types::TransferResult = Decode!(&res, types::TransferResult).unwrap();
    // println!("transfer_result: {:?}", transfer_result);

    let destination_canister = cans.individual_user(destination_canister).await;
    let res = destination_canister.add_token(root_canister).await.unwrap();
    println!("add_token res: {:?}", res);

    // let res = agent
    //     .update(
    //         &destination_canister,
    //         "add_token",
    //     )
    //     .with_arg(candid::encode_one(root_canister).unwrap())
    //     .call_and_wait()
    //     .await
    //     .unwrap();
    // println!("add_token res: {:?}", res);

    Ok(())
}

#[component]
fn FormError<V: 'static>(#[prop(into)] res: Signal<Result<V, String>>) -> impl IntoView {
    let err = Signal::derive(move || res.with(|r| r.as_ref().err().cloned()));

    view! {
        <Show when=move || res.with(|r| r.is_err())>
            <div class="flex flex-row gap-1 items-center w-full text-sm md:text-base">
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
    let (edit_mode, set_edit_mode) = create_signal(false);
    let (editable_addr, set_editable_addr) = create_signal(source_addr.to_string());
    let copy_source = move || {
        let _ = copy_to_clipboard(&source_addr.to_string());
    };
    let toggle_edit_mode = move || {
        set_edit_mode.update(|edit| *edit = !*edit);
    };

    // Event to handle the change of the source address when edited
    let handle_edit = move |event: Event| {
        let input: HtmlInputElement = event.target_unchecked_into();
        editable_addr.set(input.value());
    }; // Save the edited address
    let save_edited_address = move || {
        set_edit_mode.set(false); // Turn off edit mode after saving
    };
    let destination_ref = create_node_ref::<html::Input>();
    let paste_destination = create_action(move |&()| async move {
        let input = destination_ref()?;
        let principal = paste_from_clipboard().await?;
        input.set_value(&principal);
        #[cfg(feature = "hydrate")]
        {
            use web_sys::InputEvent;
            _ = input.dispatch_event(&InputEvent::new("input").unwrap());
        }
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
        TokenBalance::new_cdao(0u32.into())
    } else {
        info.balance.clone() - info.fees.clone()
    };
    let max_amt_c = max_amt.clone();
    let set_max_amt = move || {
        let input = amount_ref()?;
        input.set_value(&max_amt.to_tokens());
        #[cfg(feature = "hydrate")]
        {
            use web_sys::InputEvent;
            _ = input.dispatch_event(&InputEvent::new("input").unwrap());
        }
        Some(())
    };

    let amt_res = create_rw_signal(Ok::<_, String>(None::<TokenBalance>));
    _ = use_event_listener(amount_ref, ev::input, move |_| {
        let Some(input) = amount_ref() else {
            return;
        };
        let amt_raw = input.value();
        let Ok(amt) = TokenBalance::parse_cdao(&amt_raw) else {
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

    let auth_cans_wire = authenticated_canisters();

    let send_action = create_action(move |&()| {
        let cans = cans.clone();
        let auth_cans_wire = auth_cans_wire.clone();
        async move {
            let destination = destination_res.get_untracked().unwrap().unwrap();
            let destination_canister = cans
                .get_individual_canister_by_user_principal(destination)
                .await
                .unwrap()
                .unwrap();
            // let amt = amt_res.get_untracked().unwrap().unwrap();

            // let user = cans.authenticated_user().await;
            // let res = user
            //     .transfer_token_to_user_canister(root, destination, None, amt)
            //     .await
            //     .map_err(|e| e.to_string())?;
            // if let Result22::Err(e) = res {
            //     return Err(format!("{e:?}"));
            // }

            // Ok(())

            let root_canister = cans.sns_root(root).await;
            let sns_cans = root_canister
                .list_sns_canisters(ListSnsCanistersArg {})
                .await
                .unwrap();
            let ledger_canister = sns_cans.ledger.unwrap();
            log::debug!("ledger_canister: {:?}", ledger_canister);
            let amt = amt_res.get_untracked().unwrap().unwrap();

            transfer_token_to_user_principal(
                auth_cans_wire.wait_untracked().await.unwrap(),
                destination_canister,
                destination,
                ledger_canister,
                root,
                amt.clone(),
            )
            .await?;

            Ok::<_, ServerFnError>(amt)
        }
    });
    let sending = send_action.pending();

    let valid = move || {
        amt_res.with(|r| matches!(r, Ok(Some(_))))
            && destination_res.with(|r| matches!(r, Ok(Some(_))))
            && !sending()
    };

    view! {
        <div class="flex flex-col gap-4 w-dvw min-h-dvh bg-neutral-800">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback="/wallet"/>
                    <span class="justify-self-center font-bold">Send {info.name}</span>
                </div>
            </Title>
            <div class="flex flex-col gap-4 items-center p-4 w-full md:gap-6">
                <div class="flex flex-col gap-2 items-center w-full">
                    <div class="flex flex-row justify-between w-full text-sm text-white md:text-base">
                        <span>Source:</span>
                        <span>{format!("{} {}", info.balance.humanize(), info.symbol)}</span>
                    </div>
                    <div class="flex flex-row gap-2 items-center w-full">
                       {move || if edit_mode() {
                            // If in edit mode, show an input field to edit the address
                            view! {
                                <input
                                    class="p-2 w-full text-white bg-transparent rounded-lg border border-gray-400"
                                    value={editable_addr()}
                                    on:input=handle_edit
                                />
                                <button on:click=move |_| save_edited_address()>
                                    <Icon
                                        class="text-lg text-white md:text-xl"
                                        icon=icondata::AiEnterOutlined
                                    />
                                </button>
                            }
                        } else {
                            // If not in edit mode, show the address as text and the edit icon
                            view! {
                                <p class="text-sm text-white/80 md:text-md">{editable_addr()}</p>
                                <button on:click=move |_| copy_source()>
                                    <Icon
                                        class="text-lg text-white md:text-xl"
                                        icon=icondata::FaCopyRegular
                                    />
                                </button>
                                <button on:click=move |_| toggle_edit_mode()>
                                    <Icon
                                        class="text-lg text-white md:text-xl"
                                        icon=icondata::FiEdit2
                                    />
                                </button>
                            }
                        }}
                 </div>
                </div>
                <div class="flex flex-col gap-1 w-full">
                    <span class="text-sm text-white md:text-base">Destination</span>
                    <div
                        class=("border-white/15", move || destination_res.with(|r| r.is_ok()))
                        class=("border-red", move || destination_res.with(|r| r.is_err()))
                        class="flex flex-row gap-2 justify-between p-3 w-full rounded-lg border bg-white/5"
                    >
                        <input
                            _ref=destination_ref
                            class="w-full text-base text-white bg-transparent md:text-lg focus:outline-none placeholder-white/40"
                        />
                        <button on:click=move |_| paste_destination.dispatch(())>
                            <Icon
                                class="text-lg md:text-xl text-neutral-600"
                                icon=icondata::BsClipboard
                            />
                        </button>
                    </div>
                    <FormError res=destination_res/>
                </div>
                <div class="flex flex-col gap-1 w-full">
                    <div class="flex flex-row justify-between w-full text-sm text-white md:text-base">
                        <span>Amount</span>
                        <button
                            class="flex flex-row gap-1 items-center"
                            on:click=move |_| _ = set_max_amt()
                        >
                            <Icon icon=icondata::AiEnterOutlined/>
                            " Max"
                        </button>
                    </div>
                    <input
                        _ref=amount_ref
                        class=("border-white/15", move || amt_res.with(|r| r.is_ok()))
                        class=("border-red", move || amt_res.with(|r| r.is_err()))
                        class="p-3 w-full text-base text-white rounded-lg border md:text-lg focus:outline-none bg-white/5 placeholder-white/40"
                    />
                    <FormError res=amt_res/>
                </div>
                <div class="flex flex-col w-full text-sm md:text-base text-white/60">
                    <span>Transaction Fee (billed to source)</span>
                    <span>{format!("{} {}", info.fees.humanize_float(), info.symbol)}</span>
                </div>
                <button
                    on:click=move |_| send_action.dispatch(())
                    disabled=move || !valid()
                    class="flex flex-row justify-center p-3 w-full text-white rounded-full md:w-1/2 md:text-lg disabled:opacity-50 bg-primary-600"
                >
                    Send
                </button>
            </div>
            <TokenTransferPopup token_name=info.symbol transfer_action=send_action/>
        </div>
    }
}

#[component]
pub fn TokenTransfer() -> impl IntoView {
    let params = use_params::<TokenParams>();

    let token_metadata_fetch = move |cans: Canisters<true>| {
        create_resource(params, move |params| {
            let cans = cans.clone();
            let user_principal = cans.user_principal();
            async move {
                let Ok(params) = params else {
                    return Ok::<_, ServerFnError>(None);
                };
                // let user = cans.user_canister();
                let meta = token_metadata_by_root(&cans, user_principal, params.token_root).await?;
                Ok(meta.map(|m| (m, params.token_root)))
            }
        })
    };

    view! {
        <WithAuthCans
            fallback=FullScreenSpinner
            with=token_metadata_fetch
            children=|(cans, res)| {
                match res {
                    Err(e) => view! { <Redirect path=format!("/error?err={e}")/> },
                    Ok(None) => view! { <Redirect path="/"/> },
                    Ok(Some((info, root))) => view! { <TokenTransferInner cans info root/> },
                }
            }
        />
    }
}
