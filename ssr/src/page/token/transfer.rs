use crate::{
    component::{
        back_btn::BackButton, canisters_prov::WithAuthCans, spinner::FullScreenSpinner,
        title::Title,
    },
    page::token::non_yral_tokens::SUPPORTED_NON_YRAL_TOKENS_ROOT,
    state::canisters::{authenticated_canisters, Canisters, CanistersAuthWire},
    utils::{
        event_streaming::events::TokensTransferred,
        token::{get_ck_metadata, token_metadata_by_root, TokenBalance, TokenMetadata},
        web::{copy_to_clipboard, paste_from_clipboard},
    },
};
use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::use_event_listener;
use server_fn::codec::Cbor;
use yral_canisters_client::{
    sns_ledger::{Account, TransferArg},
    sns_root::ListSnsCanistersArg,
};

use super::{popups::TokenTransferPopup, TokenParams};

#[server(
    input = Cbor
)]
async fn transfer_token_to_user_principal(
    cans_wire: CanistersAuthWire,
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

    let destination_canister_principal = cans
        .get_individual_canister_by_user_principal(destination_principal)
        .await?;

    let is_non_yral_token = SUPPORTED_NON_YRAL_TOKENS_ROOT
        .iter()
        .any(|&token_root| token_root == root_canister.to_text());

    if destination_canister_principal.is_some() && !is_non_yral_token {
        let destination_canister = cans
            .individual_user(destination_canister_principal.unwrap())
            .await;
        let res = destination_canister.add_token(root_canister).await?;
        println!("add_token res: {:?}", res);
    }

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

async fn transfer_ck_token_to_user_principal(
    cans_wire: CanistersAuthWire,
    destination_principal: Principal,
    ledger_canister: Principal,
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
            <div class="flex flex-row items-center gap-1 w-full text-sm md:text-base">
                <Icon class="text-red-600" icon=icondata::AiInfoCircleOutlined />
                <span class="text-white/60">{move || err().unwrap()}</span>
            </div>
        </Show>
    }
}

#[component]
fn TokenTransferInner(
    cans: Canisters<true>,
    root: Option<Principal>,
    info: TokenMetadata,
    param: String
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
    let Some(balance) = info.balance else {
        return view! {
            <div>
                <Redirect path="/" />
            </div>
        };
    };
    let max_amt = if balance
        .map_balance_ref(|b| b > &info.fees)
        .unwrap_or_default()
    {
        balance
            .map_balance_ref(|b| b.clone() - info.fees.clone())
            .unwrap()
    } else {
        TokenBalance::new_cdao(0u32.into())
    };
    let max_amt_c = max_amt.clone();
    let set_max_amt = move || {
        let input = amount_ref()?;
        input.set_value(&max_amt.humanize_float());
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
        } else if amt.e8s == 0_u64 {
            amt_res.set(Err("Cannot send 0 tokens".to_string()));
        } else {
            amt_res.set(Ok(Some(amt)));
        }
    });

    let auth_cans_wire = authenticated_canisters();
    
    let send_action = create_action(move |&()| {
        let cans = cans.clone();
        let auth_cans_wire = auth_cans_wire.clone();
        let param = param.clone();
        async move {
            let destination = destination_res.get_untracked().unwrap().unwrap();

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

            let amt = amt_res.get_untracked().unwrap().unwrap();

            match root{
                Some(root) => {
                    let root_canister = cans.sns_root(root).await;
                    println!("{}", root);
                    let sns_cans = root_canister
                        .list_sns_canisters(ListSnsCanistersArg {})
                        .await
                        .unwrap();
                    let ledger_canister = sns_cans.ledger.unwrap();
                    log::debug!("ledger_canister: {:?}", ledger_canister);
        
                    transfer_token_to_user_principal(
                        auth_cans_wire.wait_untracked().await.unwrap(),
                        destination,
                        ledger_canister,
                        root,
                        amt.clone(),
                    )
                    .await?;
                },
                None => {
                    if &param == "ckbtc"{
                        let ledger_canister = Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap();
                        log::debug!("ledger_canister: {:?}", ledger_canister);
                        transfer_ck_token_to_user_principal(
                            auth_cans_wire.wait_untracked().await.unwrap(),
                            destination,
                            ledger_canister,
                            amt.clone(),
                        )
                        .await?;
                    }else if &param == "ckusdc"{
                        let ledger_canister = Principal::from_text("xevnm-gaaaa-aaaar-qafnq-cai").unwrap();
                        transfer_ck_token_to_user_principal(
                            auth_cans_wire.wait_untracked().await.unwrap(),
                            destination,
                            ledger_canister,
                            amt.clone(),
                        )
                        .await?;
                    }
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

    view! {
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
                            _ref=destination_ref
                            class="text-white bg-transparent w-full text-base md:text-lg placeholder-white/40 focus:outline-none"
                        />
                        <button on:click=move |_| paste_destination.dispatch(())>
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
                        _ref=amount_ref
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
                    on:click=move |_| send_action.dispatch(())
                    disabled=move || !valid()
                    class="flex flex-row justify-center text-white md:text-lg w-full md:w-1/2 rounded-full p-3 bg-primary-600 disabled:opacity-50"
                >
                    Send
                </button>
            </div>
            <TokenTransferPopup token_name=info.symbol transfer_action=send_action />
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
                let token_root = Principal::from_text(params.token_root.clone());
                let meta = if &params.token_root == "ckbtc" {
                    // Map the AgentError to ServerFnError to ensure type compatibility
                    get_ck_metadata(
                        &cans,
                        Some(user_principal),
                        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai").unwrap(),
                        Principal::from_text("n5wcd-faaaa-aaaar-qaaea-cai").unwrap(),
                    )
                    .await
                    .map_err(|e| ServerFnError::new(e.to_string()))? // Map AgentError to ServerFnError
                } else if &params.token_root == "ckusdc" {
                    get_ck_metadata(
                        &cans,
                        Some(user_principal),
                        Principal::from_text("xevnm-gaaaa-aaaar-qafnq-cai").unwrap(),
                        Principal::from_text("xrs4b-hiaaa-aaaar-qafoa-cai").unwrap(),
                    )
                    .await
                    .map_err(|e| ServerFnError::new(e.to_string()))? // Map AgentError to ServerFnError
                }else{
                    token_metadata_by_root(&cans, Some(user_principal), token_root.clone().unwrap()).await?
                };
                    
                Ok(meta.map(|m| (m, token_root.ok(), params.token_root)))
            }
        })
    };

    view! {
        <WithAuthCans
            fallback=FullScreenSpinner
            with=token_metadata_fetch
            children=|(cans, res)| {
                match res {
                    Err(e) => {
                        println!("Error: {:?}", e);
                        view! { <Redirect path=format!("/error?err={e}") /> }
                    },
                    Ok(None) => view! { <Redirect path="/" /> },
                    Ok(Some((info, root, param))) => view! { <TokenTransferInner cans info root param/> },
                }
            }
        />
    }
}
