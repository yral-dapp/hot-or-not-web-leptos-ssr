use std::{env, str::FromStr};

use crate::{
    canister::individual_user_template::Result7,
    component::{
        back_btn::{go_back_or_fallback, BackButton},
        img_to_png::ImgToPng,
        title::Title,
    },
    page::token::{sns_form::SnsFormSettings, types},
    state::canisters::auth_canisters_store,
    utils::web::FileWithUrl,
};
use leptos::*;
use leptos_router::*;

use sns_validation::pbs::nns_pb::Tokens;

use super::{popups::TokenCreationPopup, sns_form::SnsFormState};

use candid::{Decode, Encode, Nat, Principal};
use ic_agent::Identity;
use ic_agent::{identity::BasicIdentity, Agent};
use ic_base_types::PrincipalId;
use icp_ledger::Subaccount;

use crate::canister::sns_swap::{
    NewSaleTicketRequest, NewSaleTicketResponse, RefreshBuyerTokensRequest,
    RefreshBuyerTokensResponse,
};
use crate::consts::{AGENT_URL, ICP_LEDGER_CANISTER_ID};

#[server]
async fn participate_in_swap(swap_canister: Principal) -> Result<(), ServerFnError> {
    let admin_id_pem: String =
        env::var("BACKEND_ADMIN_IDENTITY").expect("`BACKEND_ADMIN_IDENTITY` is required!");
    let admin_id_pem_by = admin_id_pem.as_bytes();
    let admin_id =
        BasicIdentity::from_pem(admin_id_pem_by).expect("Invalid `BACKEND_ADMIN_IDENTITY`");
    let admin_principal = admin_id.sender().unwrap();

    let agent = Agent::builder()
        .with_url(AGENT_URL)
        .with_identity(admin_id)
        .build()
        .unwrap();
    agent.fetch_root_key().await.unwrap();

    // new_sale_ticket
    let new_sale_ticket_request = NewSaleTicketRequest {
        amount_icp_e8s: 100_000,
        subaccount: None,
    };
    let res = agent
        .update(&swap_canister, "new_sale_ticket")
        .with_arg(Encode!(&new_sale_ticket_request).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    let new_sale_ticket_response: NewSaleTicketResponse =
        Decode!(&res, NewSaleTicketResponse).unwrap();
    println!("new_sale_ticket_response: {:?}", new_sale_ticket_response);

    // transfer icp
    let subaccount = Subaccount::from(&PrincipalId(admin_principal));
    let transfer_args = types::Transaction {
        memo: Some(vec![0]),
        amount: Nat::from(1000000 as u64),
        fee: Some(Nat::from(0 as u64)),
        from_subaccount: None,
        to: types::Recipient {
            owner: swap_canister,
            subaccount: Some(subaccount.to_vec()),
        },
        created_at_time: None,
    };
    let res = agent
        .update(
            &Principal::from_str(ICP_LEDGER_CANISTER_ID).unwrap(),
            "icrc1_transfer",
        )
        .with_arg(Encode!(&transfer_args).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    let transfer_result: types::TransferResult = Decode!(&res, types::TransferResult).unwrap();
    println!("transfer_result: {:?}", transfer_result);

    // refresh_buyer_tokens
    let refresh_buyer_tokens_request = RefreshBuyerTokensRequest {
        buyer: admin_principal.to_string(),
        confirmation_text: None,
    };
    let res = agent
        .update(&swap_canister, "refresh_buyer_tokens")
        .with_arg(Encode!(&refresh_buyer_tokens_request).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    let refresh_buyer_tokens_response: RefreshBuyerTokensResponse =
        Decode!(&res, RefreshBuyerTokensResponse).unwrap();
    println!(
        "refresh_buyer_tokens_response: {:?}",
        refresh_buyer_tokens_response
    );

    Ok(())
}

#[component]
fn TokenImage() -> impl IntoView {
    let ctx = expect_context::<CreateTokenCtx>();
    let img_file = ctx.file;

    // let img_file = create_rw_signal(None::<FileWithUrl>);
    let logo_b64 = create_write_slice(ctx.form_state, |f, v| {
        f.logo_b64 = v;
    });

    let on_file_input = move |ev: ev::Event| {
        _ = ev.target().and_then(|_target| {
            #[cfg(feature = "hydrate")]
            {
                use wasm_bindgen::JsCast;
                use web_sys::HtmlInputElement;

                let input = _target.dyn_ref::<HtmlInputElement>()?;
                let file = input.files()?.get(0)?;
                img_file.set(Some(FileWithUrl::new(file.clone().into())));
                ctx.file.set(Some(FileWithUrl::new(file.into())));
            }
            Some(())
        })
    };

    let file_input_ref = create_node_ref::<leptos::html::Input>();

    let on_edit_click = move |_| {
        // Trigger the file input click
        if let Some(input) = file_input_ref.get() {
            img_file.set(None);
            ctx.file.set(None);
            input.set_value("");
            input.click();
            // input.click();
        }
    };

    let img_url = Signal::derive(move || img_file.with(|f| f.as_ref().map(|f| f.url.to_string())));

    let border_class = move || match img_url.with(|u| u.is_none()) {
        true => format!("relative w-20 h-20 rounded-full border-2 border-white/20"),
        _ => format!("relative w-20 h-20 rounded-full border-2 border-primary-600"),
    };

    view! {
        <div class="flex flex-col space-y-4  rounded-lg text-white">

            <div class="flex items-center space-x-4">
                <div class= move || border_class()  >
            <Show
                    when=move || img_url.with(|u| u.is_none())
                    fallback=move || {
                        view! {
                            <img
                                class="object-conver h-full w-full rounded-full"
                                src=move || img_url().unwrap()
                            />
                             <div class="absolute bottom-0 right-0 p-1 rounded-full bg-white ">
                           <button on:click=on_edit_click class="w-4 h-4 flex items-center justify-center rounded-full bg-white" >
                                 <img src="/img/edit.svg" class="bg-white w-4 h-4 rounded-full" />
                           </button>
                             </div>
                        }
                    }
                >

              <div class="flex items-center justify-center w-full h-full rounded-full">
                        <span class="text-xs text-center text-gray-400 font-medium">"Add custom logo"</span>
                    </div>

                    <input type="file"
                        node_ref=file_input_ref
                        on:change=on_file_input
                        id="dropzone-logo"
                        accept="image/*"
                    class="absolute inset-0 w-full h-full opacity-0 cursor-pointer" />
                    <div class="absolute bottom-0 right-0 p-1 rounded-full bg-white ">
                        <img src="/img/upload.svg" class="bg-white" />
                    </div>
                </Show>

                </div>
               /*  <div class="flex-1">
                    <InputBox
                        heading="Token name"
                        placeholder="Add a name to your crypto currency"
                        updater=set_token_name
                        validator=non_empty_string_validator
                    />

                </div> */
            </div>
         </div>
                 <ImgToPng img_file=img_file output_b64=logo_b64/>

    }
}

// #[component]
// fn TokenImgInput() -> impl IntoView {
//     let ctx = expect_context::<CreateTokenCtx>();
//     let img_file = ctx.file;
//     let logo_b64 = create_write_slice(ctx.form_state, |f, v| {
//         f.logo_b64 = v;
//     });

//     let on_file_input = move |ev: ev::Event| {
//         _ = ev.target().and_then(|_target| {
//             #[cfg(feature = "hydrate")]
//             {
//                 use wasm_bindgen::JsCast;
//                 use web_sys::HtmlInputElement;

//                 let input = _target.dyn_ref::<HtmlInputElement>()?;
//                 let file = input.files()?.get(0)?;
//                 img_file.set(Some(FileWithUrl::new(file.clone().into())));
//                 ctx.file.set(Some(FileWithUrl::new(file.into())));
//             }
//             Some(())
//         })
//     };
//     let img_url = Signal::derive(move || img_file.with(|f| f.as_ref().map(|f| f.url.to_string())));

//     view! {
//         <div class="ml-2 md:ml-4 h-20 w-20 md:w-36 md:h-36 rounded-full">
//             <label for="dropzone-logo" class="flex flex-col h-full w-full cursor-pointer">
//                 <Show
//                     when=move || img_url.with(|u| u.is_none())
//                     fallback=move || {
//                         view! {
//                             <img
//                                 class="object-contain h-full w-full rounded-full"
//                                 src=move || img_url().unwrap()
//                             />
//                         }
//                     }
//                 >

//                     <div class="flex flex-col items-center justify-center h-full w-full bg-white/10 rounded-full border-2 border-dashed border-neutral-600 hover:bg-white/15">
//                         <Icon
//                             icon=icondata::BiImageAddSolid
//                             class="text-center bg-white/30 text-4xl"
//                         />
//                     </div>
//                 </Show>
//                 <input
//                     on:change=on_file_input
//                     id="dropzone-logo"
//                     type="file"
//                     accept="image/*"
//                     class="sr-only"
//                 />
//             </label>
//         </div>
//         <ImgToPng img_file=img_file output_b64=logo_b64/>
//     }
// }

macro_rules! input_component {
    ($name:ident, $input_element:ident, $attrs:expr) => {
        #[component]
        fn $name<T: 'static, U: Fn(T) + 'static, V: Fn(String) -> Option<T> + 'static>(
            #[prop(into)] heading: String,
            #[prop(into)] placeholder: String,
            #[prop(optional)] initial_value: Option<String>,
            #[prop(optional)] input_type: Option<String>,
            updater: U,
            validator: V,
        ) -> impl IntoView {
            let ctx: CreateTokenCtx = expect_context();
            let error = create_rw_signal(initial_value.is_none());
            let show_error = create_rw_signal(false);
            if error.get_untracked() {
                ctx.invalid_cnt.update(|c| *c += 1);
            }

            let input_class =move ||  match show_error() && error() {
                false => format!("w-full p-3  md:p-4 md:py-5 text-white outline-none bg-white/10 border-2 border-solid border-white/20 text-xs  rounded-xl placeholder-neutral-600"),
                _ =>  format!("w-full p-3  md:p-4 md:py-5 text-white outline-none bg-white/10 border-2 border-solid border-red-500 text-xs  rounded-xl placeholder-neutral-600")
            };
            view! {
                <div class="flex flex-col grow gap-y-1 text-sm md:text-base">
                     <span class="text-white font-semibold">{heading.clone()}</span>
                     <$input_element prop:value={initial_value.unwrap_or_default()} on:input=move |ev| {
                        let value = event_target_value(&ev);
                        match validator(value) {
                            Some(v) => {
                                if error.get_untracked() {
                                    ctx.invalid_cnt.update(|c| *c -= 1);
                                }
                                error.set(false);
                                updater(v);
                            },
                            None => {
                                show_error.set(true);
                                if error.get_untracked() {
                                    return;
                                }
                                error.set(true);
                                ctx.invalid_cnt.update(|c| *c += 1);
                                }
                            }
                        }
                        $attrs
                        placeholder=placeholder
                        class=move || input_class()
                        type=input_type.unwrap_or_else(|| "text".into() )
                         />

                <Show when=move || show_error() && error() fallback=move || view!{
                                            <span class="text-red-500 font-semibold">  </span>
                } >
                        <span class="text-red-500 font-semibold">Invalid </span>
                    </Show>

                </div>
            }
        }
    }
}

fn non_empty_string_validator(s: String) -> Option<String> {
    (!s.is_empty()).then_some(s)
}

fn non_empty_string_validator_for_u64(s: String) -> Option<String> {
    if !s.is_empty() && s.parse::<u64>().is_ok() {
        Some(s)
    } else {
        None
    }
}

input_component!(InputBox, input, {});
input_component!(InputArea, textarea, rows = 4);
input_component!(InputField, textarea, rows = 1);

#[derive(Clone, Copy, Default)]
pub struct CreateTokenCtx {
    form_state: RwSignal<SnsFormState>,
    invalid_cnt: RwSignal<u32>,
    file: RwSignal<Option<FileWithUrl>>,
}

fn parse_token_e8s(s: &str) -> Result<Tokens, String> {
    let e8s: u64 = s
        .replace('_', "")
        .parse::<u64>()
        .map_err(|err| err.to_string())?;

    Ok(Tokens { e8s: Some(e8s) })
}

#[component]
pub fn CreateToken() -> impl IntoView {
    let auth_cans = auth_canisters_store();
    let fallback_url = Signal::derive(move || {
        let Some(cans) = auth_cans() else {
            return "/menu".to_string();
        };
        let id = cans.profile_details().username_or_principal();
        format!("/your-profile/{id}?tab=tokens")
    });
    let ctx: CreateTokenCtx = expect_context();
    // use_context().unwrap_or_else(|| {
    //     let ctx = CreateTokenCtx::default();
    //     provide_context(ctx);
    //     ctx
    //  });

    let set_token_name = move |name: String| {
        ctx.form_state.update(|f| f.name = Some(name));
    };
    let set_token_symbol = move |symbol: String| {
        ctx.form_state.update(|f| f.symbol = Some(symbol));
    };
    let set_token_desc = move |desc: String| {
        ctx.form_state.update(|f| f.description = Some(desc));
    };
    /*
        let set_transaction_fee = move |fee: String| {
            ctx.form_state
                .update(|f| f.transaction_fee = parse_token_e8s(&fee).unwrap());
        };
    */
    let set_total_distribution = move |total: String| {
        ctx.form_state.update(|f| {
            (*f).try_update_total_distribution_tokens(parse_token_e8s(&total).unwrap())
        });
    };

    let create_action = create_action(move |&()| async move {
        let cans = auth_cans
            .get_untracked()
            .expect("Create token called without auth canisters");
        let sns_form = ctx.form_state.get_untracked();
        let sns_config = sns_form.try_into_config(&cans)?;

        let create_sns = sns_config.try_convert_to_executed_sns_init()?;
        let res = cans
            .deploy_cdao_sns(create_sns)
            .await
            .map_err(|e| e.to_string())?;
        match res {
            Result7::Ok(c) => {
                log::debug!("deployed canister {}", c.governance);
                let participated = participate_in_swap(c.swap).await;
                if let Err(e) = participated {
                    return Err(format!("{e:?}"));
                } else {
                    log::debug!("participated in swap");
                }
            }
            Result7::Err(e) => {
                return Err(format!("{e:?}"));
            }
        };

        Ok::<_, String>(())
    });
    let creating = create_action.pending();

    let create_disabled = create_memo(move |_| {
        creating()
            || auth_cans.with(|c| c.is_none())
            || ctx.form_state.with(|f| f.logo_b64.is_none())
            || ctx.invalid_cnt.get() != 0
    });

    let create_act_value = create_action.value();
    let create_act_res = Signal::derive(move || {
        if creating() {
            return None;
        }
        create_act_value()
    });

    view! {
                <div class="w-dvw min-h-dvh bg-black pt-4 flex flex-col gap-4" style="padding-bottom:6rem" >
                    <Title justify_center=false>
                        <div class="flex justify-between w-full" >
                            <BackButton fallback=fallback_url/>
                            <span class="font-bold justify-self-center">Create Meme Token </span>
                            <button on:click=move |_|{navigate_token_faq() }  > <img src="/img/info.svg"/ > </button>
                        </div>
                    </Title>
                    <div class="flex flex-col w-full px-6 md:px-8 gap-2 md:gap-8">
                       /*  <Show when=move || {
                            create_act_res.with(|v| v.as_ref().map(|v| v.is_err()).unwrap_or_default())
                        }>
                            <div class="flex flex-col w-full items-center gap-2">
                                <span class="text-red-500 font-semibold text-center">
                                    Error creating token:
                                </span>
                                <textarea
                                    prop:value=create_act_res().unwrap().err().unwrap()
                                    disabled
                                    rows=3
                                    class="bg-white/10 text-xs md:text-sm text-red-500/60 w-full md:w-2/3 resize-none p-2"
                                ></textarea>
                            </div>
                        </Show>
                        */
                        <div class="flex flex-row w-full gap-4  justify-between items-center">
                        <TokenImage/>
                            <InputBox
                                heading="Token name"
                                placeholder="Add a name to your crypto currency"
                                updater=set_token_name
                                validator=non_empty_string_validator
                                initial_value=(ctx.form_state.get_untracked()).name.unwrap_or_default()
                            />
                        </div>
        /*
                       <div class="flex flex-row w-full justify-between items-center">
                            <TokenImgInput/>
                            <InputBox
                                heading="Token name"
                                placeholder="Add a name to your crypto currency"
                                updater=set_token_name
                                validator=non_empty_string_validator
                            />
                        </div>
          */

                        <InputArea
                            heading="Description"
                            placeholder="Fun & friendly internet currency inspired by the legendary Shiba Inu dog 'Kabosu'"
                            updater=set_token_desc
                            validator=non_empty_string_validator
                            initial_value=(ctx.form_state.get_untracked()).description.unwrap_or_default()

                        />
                        <InputBox
                            heading="Token Symbol"
                            placeholder="Eg. DODGE"
                            updater=set_token_symbol
                            validator=non_empty_string_validator
                            initial_value=(ctx.form_state.get_untracked()).symbol.unwrap_or_default()

                        />
          /*
                        <InputBox
                            heading="Transaction Fee"
                            placeholder="Fee"
                            input_type="number".into()
                            updater=set_transaction_fee
                            validator=non_empty_string_validator_for_u64
                        />
        */
                        <InputBox
                            heading="Distribution"
                            placeholder="Distribution Tokens"
                            input_type="number".into()
                            updater=set_total_distribution
                            // initial_value="100000000".into()
                            initial_value=(ctx.form_state.get_untracked()).total_distrubution().e8s.unwrap_or_else(||100000000).to_string()
                            validator=non_empty_string_validator_for_u64
                        />

                        <div class="w-full flex justify-center">
                            <button
                                on:click=move |_| create_action.dispatch(())
                                disabled=create_disabled
                                class="text-white disabled:text-neutral-500 md:text-xl py-4 md:py-4 font-bold w-full md:w-1/2 lg:w-1/3 rounded-full bg-primary-600 disabled:bg-primary-500/30"
                            >
                                {move || if creating() { "Creating..." } else { "Create" }}
                            </button>
                        </div>

                        <div class="w-full flex justify-center underline text-sm text-white my-4 " >
                        <a href="/token/create/settings"  >  View advanced settings </a>

                        //  <button on:click=move |_|{navigate_token_settings() }  >  View advanced settings </button>
                         </div>
                    </div>
                    <TokenCreationPopup
                        creation_action=create_action
                        token_name=Signal::derive(move || {
                            ctx.form_state.with(|f| f.name.clone()).unwrap_or_default()
                        })
    />
                </div>
            }
}

// fn navigate_token_settings() {
//     let navigate = use_navigate();
//     navigate("/token/create/settings", Default::default());
// }

fn navigate_token_faq() {
    let navigate = use_navigate();
    navigate("/token/create/faq", Default::default());
}

fn clear_form(_form_ref: &NodeRef<html::Form>) {
    // #[cfg(feature = "hydrate")] {
    // use web_sys::window;
    //     if let Some(win) = window() {
    //         _ = win.location().reload_with_forceget(false);
    //     }
    // }
    go_back_or_fallback("/token/crate");
    // navigate_token_settings();
}

#[component]
pub fn CreateTokenSettings() -> impl IntoView {
    let auth_cans = auth_canisters_store();
    let fallback_url = Signal::derive(move || {
        let Some(cans) = auth_cans() else {
            return "/token/create".to_string();
        };
        let id = cans.profile_details().username_or_principal();
        format!("/your-profile/{id}?tab=tokens")
    });
    let ctx: CreateTokenCtx = use_context().unwrap_or_else(|| {
        let ctx = CreateTokenCtx::default();
        provide_context(ctx);
        ctx
    });

    // let save_action = create_action(move |&()| async move {
    //     let cans = auth_cans
    //         .get_untracked()
    //         .expect("Create token called without auth canisters");
    //     let sns_form = ctx.form_state.get_untracked();
    //     let sns_config = sns_form.try_into_config(&cans)?;

    //     Ok::<_, String>(())
    // });
    // let saving = save_action.pending();

    // let save_disabled = create_memo(move |_| {
    //     saving()
    //         || auth_cans.with(|c| c.is_none())
    //         || ctx.form_state.with(|f| f.logo_b64.is_none())
    //         || ctx.invalid_cnt.get() != 0
    // });

    let set_sns_proposal_link = move |value: String| {
        ctx.form_state
            .update(|f| f.sns_form_setting.sns_proposal_link = Some(value));
    };

    let set_nns_proposal_link = move |value: String| {
        ctx.form_state
            .update(|f| f.sns_form_setting.nns_proposal_link = Some(value));
    };
    let set_dapp_canister_id = move |value: String| {
        ctx.form_state
            .update(|f| f.sns_form_setting.dapp_canister_id = Some(value));
    };
    let set_transaction_fee = move |value: String| {
        ctx.form_state
            .update(|f| f.transaction_fee = parse_token_e8s(&value).unwrap());
    };
    let set_rejection_fee = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.rejection_fee = parse_token_e8s(&value).ok();
        });
    };
    let set_initial_voting_period_in_days = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.initial_voting_period_in_days = value.parse::<u64>().ok();
        });
    };
    let set_max_wait_deadline_extention = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.max_wait_deadline_extention = value.parse::<u64>().ok();
        });
    };
    let set_min_creation_stake = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.min_creation_stake = value.parse::<u64>().ok();
        });
    };

    let set_min_dissolve_delay = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.min_dissolve_delay = value.parse::<u64>().ok();
        });
    };
    let set_age_in_years = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.age_duration_in_years = value.parse::<u64>().ok();
        });
    };

    let set_age_bonus = move |value: String| {
        ctx.form_state.update(|f| {
            if let Ok(value) = value.parse::<u64>() {
                if value <= 100 {
                    f.sns_form_setting.age_bonus = Some(value);
                }
            }
        });
    };

    let set_min_participants = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.min_participants = value.parse::<u64>().ok();
        });
    };

    let set_min_direct_participants_icp = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.min_direct_participants_icp = value.parse::<u64>().ok();
        });
    };

    let set_max_direct_participants_icp = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.max_direct_participants_icp = value.parse::<u64>().ok();
        });
    };

    let set_min_participants_icp = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.min_participants_icp = value.parse::<u64>().ok();
        });
    };

    let set_max_participants_icp = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.max_participants_icp = value.parse::<u64>().ok();
        });
    };

    let set_restricted_country = move |value: String| {
        ctx.form_state.update(|f| {
            f.sns_form_setting.restricted_country = Some(value);
        });
    };

    let form_ref = create_node_ref::<html::Form>();

    let reset_settings = move |_| {
        ctx.form_state
            .update(|f| f.sns_form_setting = SnsFormSettings::default());
        clear_form(&form_ref);
    };

    view! {
         <div class="w-dvw min-h-dvh bg-black pt-4 flex flex-col gap-4 p-4" style="padding-bottom:5rem;" >
                   <Title justify_center=false >
                    <div class="flex justify-between w-full" style="background: black" >
                        <BackButton fallback=fallback_url/>
                        <span class="font-bold justify-self-center">Settings</span>
                        <button on:click=move |_|{navigate_token_faq() }  > <img src="/img/info.svg"/ > </button>
                    </div>
                    </Title>
                <label class="flex flex-cols-2 cursor-pointer px-1">
                <span class="flex-1 text-sm font-medium text-gray-400 dark:text-gray-500">Do you want to raise ICP?</span>
                <div>
                  <input type="checkbox" value="" class="sr-only peer" checked disabled />
                    <div class="relative w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-checked:bg-gray-600">
                    <div class="absolute top-0.5 left-0.5 bg-white border border-gray-300 rounded-full h-5 w-5 transition-transform peer-checked:translate-x-5 dark:border-gray-600"></div>
                    </div>
                </div>
                 // <div class="relative w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-gray-600"></div>
                </label>

                <form node_ref=form_ref >

                <InputField
                        heading="SNS proposal link"
                        placeholder="https://your-proposal-link.com"
                        updater=set_sns_proposal_link
                        validator=non_empty_string_validator
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.sns_proposal_link.unwrap_or_default()
                />
                <InputField
                        heading="NNS proposal link"
                        placeholder="https://your-proposal-link.com"
                        updater=set_nns_proposal_link
                        validator=non_empty_string_validator
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.nns_proposal_link.unwrap_or_default()

                />
                <InputField
                        heading="Dapp Canister ID"
                        placeholder="#8539434643"
                        updater=set_dapp_canister_id
                        validator=non_empty_string_validator
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.dapp_canister_id.unwrap_or_default()

                />
                <InputBox
                        heading="Transaction Fee (e8s)"
                        placeholder="Fee"
                        input_type="number".into()
                        updater=set_transaction_fee
                        validator=non_empty_string_validator_for_u64
                        initial_value=(ctx.form_state.get_untracked()).transaction_fee.e8s.unwrap_or(1).to_string()

                 />
                 <InputBox
                        heading="Rejection Fee (Token)"
                        placeholder="1 Token"
                        input_type="number".into()
                        updater=set_rejection_fee
                        validator=non_empty_string_validator_for_u64
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.rejection_fee.unwrap_or_default().e8s.unwrap_or(1).to_string()

                 />
                 <InputBox
                        heading="Initial Voting Period (days)"
                        placeholder="4 days"
                        input_type="number".into()
                        updater=set_initial_voting_period_in_days
                        validator=non_empty_string_validator_for_u64
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.initial_voting_period_in_days.unwrap_or(4).to_string()

                 />
                 <InputBox
                        heading="Maximum wait for quiet deadline extention (days)"
                        placeholder="1 day"
                        input_type="number".into()
                        updater=set_max_wait_deadline_extention
                        validator=non_empty_string_validator_for_u64
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.max_wait_deadline_extention.unwrap_or(1).to_string()

                 />
                 <InputBox
                        heading="Minimum creation stake (token)"
                        placeholder="1 token"
                        input_type="number".into()
                        updater=set_min_creation_stake
                        validator=non_empty_string_validator_for_u64
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.min_creation_stake.unwrap_or(1).to_string()

                 />
                 <InputBox
                        heading="Minimum dissolve delay (months)"
                        placeholder="1 month"
                        input_type="number".into()
                        updater=set_min_dissolve_delay
                        validator=non_empty_string_validator_for_u64
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.min_dissolve_delay.unwrap_or(1).to_string()

                 />
                 <InputBox
                        heading="Age (duration in years)"
                        placeholder="4 years"
                        input_type="number".into()
                        updater=set_age_in_years
                        validator=non_empty_string_validator_for_u64
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.age_duration_in_years.unwrap_or(4).to_string()

                 />
                 <InputBox
                        heading="Age (bonus %)"
                        placeholder="25%"
                        input_type="number".into()
                        updater=set_age_bonus
                        validator=non_empty_string_validator_for_u64
                        initial_value=(ctx.form_state.get_untracked()).sns_form_setting.age_bonus.unwrap_or(25).to_string()

                 />
                 <InputBox
                        heading="Minimum participants"
                        placeholder="57"
                        input_type="number".into()
                        updater=set_min_participants
                        validator=non_empty_string_validator_for_u64
                 />
                 <InputBox
                        heading="Minimum direct participant icp"
                        placeholder="100,000 tokens"
                        input_type="number".into()
                        updater=set_min_direct_participants_icp
                        validator=non_empty_string_validator_for_u64
                 />
                 <InputBox
                        heading="Maximum direct participant icp"
                        placeholder="1000,000 tokens"
                        input_type="number".into()
                        updater=set_max_direct_participants_icp
                        validator=non_empty_string_validator_for_u64
                 />
                 <InputBox
                        heading="Minimum participant icp"
                        placeholder="10 tokens"
                        input_type="number".into()
                        updater=set_min_participants_icp
                        validator=non_empty_string_validator_for_u64
                 />
                 <InputBox
                        heading="Maximum participant icp"
                        placeholder="10,000 tokens"
                        input_type="number".into()
                        updater=set_max_participants_icp
                        validator=non_empty_string_validator_for_u64
                 />
                 <InputBox
                        heading="Restricted Country"
                        placeholder="Antarctica"
                        updater=set_restricted_country
                        validator=non_empty_string_validator
                 />
                 </form>
                //  <div class="w-full flex justify-center">
                //             <button
                //                 // on:click=move |_| create_action.dispatch(())
                //                 disabled=save_disabled
                //                 class="text-white disabled:text-neutral-500 md:text-xl py-4 md:py-4 font-bold w-full md:w-1/2 lg:w-1/3 rounded-full bg-primary-600 disabled:bg-primary-500/30"
                //             >
                //             Save
                //             </button>
                //  </div>
                 <button on:click=reset_settings class="w-full flex justify-center underline text-sm text-white my-4 " >Reset to default</button>
            </div>

    }
}
