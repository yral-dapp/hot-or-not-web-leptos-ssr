#[cfg(feature = "ssr")]
mod server_impl;

use crate::{
    component::{back_btn::BackButton, img_to_png::ImgToPng, title::Title},
    state::canisters::{auth_canisters_store, authenticated_canisters, CanistersAuthWire},
    utils::web::FileWithUrl,
};
use leptos::*;
use std::env;

use server_fn::codec::Cbor;
use sns_validation::{humanize::parse_tokens, pbs::nns_pb::Tokens};
use sns_validation::{
    humanize::{
        format_duration, format_percentage, format_tokens, parse_duration, parse_percentage,
    },
    pbs::sns_pb::SnsInitPayload,
};

use super::{popups::TokenCreationPopup, sns_form::SnsFormState};

use icp_ledger::AccountIdentifier;

#[server]
async fn is_server_available() -> Result<(bool, AccountIdentifier), ServerFnError> {
    server_impl::is_server_available().await
}

#[server(
    input = Cbor
)]
async fn deploy_cdao_canisters(
    cans_wire: CanistersAuthWire,
    create_sns: SnsInitPayload,
) -> Result<(), ServerFnError> {
    server_impl::deploy_cdao_canisters(cans_wire, create_sns).await
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

    let file_input_ref: NodeRef<html::Input> = create_node_ref::<leptos::html::Input>();

    let on_edit_click = move |_| {
        // Trigger the file input click
        if let Some(input) = file_input_ref.get() {
            input.click();
            // input.click();
        }
    };

    let img_url = Signal::derive(move || img_file.with(|f| f.as_ref().map(|f| f.url.to_string())));

    let border_class = move || match img_url.with(|u| u.is_none()) {
        true => "relative w-20 h-20 rounded-full border-2 border-white/20".to_string(),
        _ => "relative w-20 h-20 rounded-full border-2 border-primary-600".to_string(),
    };

    view! {
        <div class="flex flex-col space-y-4  rounded-lg text-white">

            <div class="flex items-center space-x-4">
                <div class= border_class  >


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
                <Show when = move || img_url.with(|u| u.is_some()) fallback=|| view! {  <div></div> }>
                    <img
                    class="absolute top-0 object-conver h-full w-full rounded-full"
                    src=move || img_url().unwrap()
                    />
                    <div class="absolute bottom-0 right-0 p-1 rounded-full bg-white ">
                    <button on:click=on_edit_click class="w-4 h-4 flex items-center justify-center rounded-full bg-white" >
                     <img src="/img/edit.svg" class="bg-white w-4 h-4 rounded-full" />
                    </button>
                    </div>
                </Show>



                </div>


            </div>
         </div>
                 <ImgToPng img_file=img_file output_b64=logo_b64/>
    }
}

macro_rules! input_component {
    ($name:ident, $input_element:ident, $attrs:expr) => {
        #[component]
        fn $name<T: 'static, U: Fn(T) + 'static, V: Fn(String) -> Option<T> + 'static>(
            #[prop(into)] heading: String,
            #[prop(into)] placeholder: String,
            #[prop(optional)] initial_value: Option<String>,
            #[prop(optional, into)] input_type: Option<String>,
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
                     <$input_element value={initial_value.unwrap_or_default()} on:input=move |ev| {
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

                    <span class="text-red-500 font-semibold">
                        <Show when=move || show_error() && error()>
                                "Invalid "
                        </Show>
                    </span>
                </div>
            }
        }
    }
}

fn non_empty_string_validator(s: String) -> Option<String> {
    (!s.is_empty()).then_some(s)
}

fn non_empty_string_validator_for_u64(s: String) -> Option<u64> {
    if s.is_empty() {
        return None;
    }
    s.parse().ok()
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

impl CreateTokenCtx {
    pub fn reset() {
        let ctx: Self = expect_context();

        ctx.file.set(None);
        ctx.form_state.set(SnsFormState::default());
        ctx.invalid_cnt.set(0);
    }
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

    let set_token_name = move |name: String| {
        ctx.form_state.update(|f| f.name = Some(name));
    };
    let set_token_symbol = move |symbol: String| {
        ctx.form_state.update(|f| f.symbol = Some(symbol));
    };
    let set_token_desc = move |desc: String| {
        ctx.form_state.update(|f| f.description = Some(desc));
    };
    let set_total_distribution = move |total: u64| {
        ctx.form_state.update(|f| {
            (*f).try_update_total_distribution_tokens(
                parse_tokens(&format!("{} tokens", total)).unwrap(),
            );
        });
    };

    let cans_wire_res = authenticated_canisters();

    let create_action = create_action(move |&()| {
        let cans_wire_res = cans_wire_res.clone();
        async move {
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|e| e.to_string())?;
            let cans = cans_wire
                .clone()
                .canisters()
                .map_err(|_| "Unable to authenticate".to_string())?;

            let sns_form = ctx.form_state.get_untracked();
            let sns_config = sns_form.try_into_config(&cans)?;

            let create_sns = sns_config.try_convert_to_executed_sns_init()?;
            let server_available = is_server_available().await.map_err(|e| e.to_string())?;
            log::debug!(
                "Server details: {}, {}",
                server_available.0,
                server_available.1
            );
            if !server_available.0 {
                return Err("Server is not available".to_string());
            }

            deploy_cdao_canisters(cans_wire, create_sns)
                .await
                .map_err(|e| e.to_string())
        }
    });
    let creating = create_action.pending();

    let create_disabled = create_memo(move |_| {
        creating()
            || auth_cans.with(|c| c.is_none())
            || ctx.form_state.with(|f| f.logo_b64.is_none())
            || ctx.form_state.with(|f: &SnsFormState| f.name.is_none())
            || ctx
                .form_state
                .with(|f: &SnsFormState| f.description.is_none())
            || ctx.form_state.with(|f| f.symbol.is_none())
            || ctx.invalid_cnt.get() != 0
            || ctx.file.with(|f| f.is_none())
    });

    // let create_act_value = create_action.value();
    // let create_act_res = Signal::derive(move || {
    //     if creating() {
    //         return None;
    //     }
    //     create_act_value()
    // });

    view! {
                <div class="w-dvw min-h-dvh bg-black pt-4 flex flex-col gap-4" style="padding-bottom:6rem" >
                    <Title justify_center=false>
                        <div class="flex justify-between w-full" >
                            <BackButton fallback=fallback_url/>
                            <span class="font-bold justify-self-center">Create Meme Token </span>
                            <a href="/token/create/faq"><img src="/img/info.svg" /></a>
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
                                initial_value=ctx.form_state.with_untracked(|f| f.name.clone()).unwrap_or_default()
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
                            initial_value=ctx.form_state.with_untracked(|f| f.description.clone()).unwrap_or_default()

                        />
                        <InputBox
                            heading="Token Symbol"
                            placeholder="Eg. DODGE"
                            updater=set_token_symbol
                            validator=non_empty_string_validator
                            initial_value=ctx.form_state.with_untracked(|f| f.symbol.clone()).unwrap_or_default()

                        />
                        <InputBox
                            heading="Distribution"
                            placeholder="Distribution Tokens"
                            input_type="number"
                            updater=set_total_distribution
                            // initial_value="100000000".into()
                            initial_value=(ctx.form_state.with_untracked(|f| f.total_distrubution().e8s.unwrap_or_else(|| 1000000 * 10e8 as u64) / 10e8 as u64)).to_string()
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
                        img_url=Signal::derive(move || ctx.file.with(|f| f.clone()).unwrap().url.to_string())
                        token_name=Signal::derive(move || {
                            ctx.form_state.with(|f| f.name.clone()).unwrap_or_default()
                        })
    />
                </div>
            }
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
    let fstate = ctx.form_state;

    let validate_tokens = |value: String| parse_tokens(&value).ok();
    let validate_tokens_e8s = |value: String| parse_token_e8s(&value).ok();
    let (transaction_fee, set_transaction_fee) = slice!(fstate.transaction_fee);
    let (rejection_fee, set_rejection_fee) = slice!(fstate.proposals.rejection_fee);

    let validate_duration = |value: String| parse_duration(&value).ok();
    let (initial_voting_period, set_initial_voting_period) =
        slice!(fstate.proposals.initial_voting_period);
    let (max_wait_deadline_extension, set_max_wait_deadline_extension) =
        slice!(fstate.proposals.maximum_wait_for_quiet_deadline_extension);
    let (min_creation_stake, set_min_creation_stake) =
        slice!(fstate.neurons.minimum_creation_stake);
    let (min_dissolve_delay, set_min_dissolve_delay) = slice!(fstate.voting.minimum_dissolve_delay);
    let (age, set_age) = slice!(fstate.voting.maximum_voting_power_bonuses.age.duration);

    let validate_percentage = |value: String| parse_percentage(&value).ok();
    let (age_bonus, set_age_bonus) = slice!(fstate.voting.maximum_voting_power_bonuses.age.bonus);
    let (min_participants, set_min_participants) = slice!(fstate.swap.minimum_participants);

    let optional_tokens_validator = |value: String| {
        if value.is_empty() {
            return Some(None);
        }
        Some(Some(parse_tokens(&value).ok()?))
    };
    let (min_direct_participants_icp, set_min_direct_participants_icp) =
        slice!(fstate.swap.minimum_direct_participation_icp);
    let (max_direct_participants_icp, set_max_direct_participants_icp) =
        slice!(fstate.swap.maximum_direct_participation_icp);
    let (min_participants_icp, set_min_participants_icp) =
        slice!(fstate.swap.minimum_participant_icp);
    let (max_participants_icp, set_max_participants_icp) =
        slice!(fstate.swap.maximum_participant_icp);

    // let set_restricted_country = move |value: String| {
    //     ctx.form_state.update(|f| {
    //         f.sns_form_setting.restricted_country = Some(value);
    //     });
    // };

    let reset_settings = move |_| {
        ctx.form_state.update(|f| f.reset_advanced_settings());
    };

    view! {
         <div class="w-dvw min-h-dvh bg-black pt-4 flex flex-col gap-4 p-4" style="padding-bottom:5rem;" >
                   <Title justify_center=false >
                    <div class="flex justify-between w-full" style="background: black" >
                        <BackButton fallback=fallback_url/>
                        <span class="font-bold justify-self-center">Settings</span>
                        <a href="/token/create/faq"><img src="/img/info.svg" /></a>
                    </div>
                    </Title>
                <label class="flex flex-cols-2 cursor-pointer px-1">
                <span class="flex-1 text-sm font-medium text-gray-400 dark:text-gray-500">Do you want to raise ICP?</span>
                <div>
                    <span class="text-sm font-medium text-gray-400 dark:text-gray-500">Coming Soon!</span>
                //   <input type="checkbox" value="" class="sr-only peer" checked disabled />
                    // <div class="relative w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-checked:bg-gray-600">
                    //     <div class="absolute top-0.5 left-0.5 bg-white border border-gray-300 rounded-full h-5 w-5 transition-transform peer-checked:translate-x-5 dark:border-gray-600"/>
                    // </div>
                </div>
                 // <div class="relative w-11 h-6 bg-gray-200 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-gray-600"></div>
                </label>

                <InputBox
                        heading="Transaction Fee (e8s)"
                        input_type="number"
                        placeholder="100"
                        updater=set_transaction_fee
                        validator=validate_tokens_e8s
                        initial_value=transaction_fee.get_untracked().e8s.unwrap_or(1).to_string()
                 />
                 <InputBox
                        heading="Rejection Fee (Token)"
                        placeholder="1 Token"
                        updater=set_rejection_fee
                        validator=validate_tokens
                        initial_value=format_tokens(&rejection_fee.get_untracked())
                 />
                 <InputBox
                        heading="Initial Voting Period (days)"
                        placeholder="4 days"
                        updater=set_initial_voting_period
                        validator=validate_duration
                        initial_value=format_duration(&initial_voting_period.get_untracked())
                 />
                 <InputBox
                        heading="Maximum wait for quiet deadline extention (days)"
                        placeholder="1 day"
                        updater=set_max_wait_deadline_extension
                        validator=validate_duration
                        initial_value=format_duration(&max_wait_deadline_extension.get_untracked())

                 />
                 <InputBox
                        heading="Minimum creation stake (token)"
                        placeholder="1 token"
                        updater=set_min_creation_stake
                        validator=validate_tokens
                        initial_value=format_tokens(&min_creation_stake.get_untracked())

                 />
                 <InputBox
                        heading="Minimum dissolve delay (months)"
                        placeholder="1 month"
                        updater=set_min_dissolve_delay
                        validator=validate_duration
                        initial_value=format_duration(&min_dissolve_delay.get_untracked())

                 />
                 <InputBox
                        heading="Age (duration in years)"
                        placeholder="4 years"
                        updater=set_age
                        validator=validate_duration
                        initial_value=format_duration(&age.get_untracked())

                 />
                 <InputBox
                        heading="Age (bonus %)"
                        placeholder="25%"
                        updater=set_age_bonus
                        validator=validate_percentage
                        initial_value=format_percentage(&age_bonus.get_untracked())

                 />
                 <InputBox
                        heading="Minimum participants"
                        placeholder="57"
                        input_type="number"
                        updater=set_min_participants
                        validator=non_empty_string_validator_for_u64
                        initial_value=min_participants.get_untracked().to_string()
                 />
                 <InputBox
                        heading="Minimum direct participant icp"
                        placeholder="100,000 tokens"
                        updater=set_min_direct_participants_icp
                        validator=optional_tokens_validator
                        initial_value=min_direct_participants_icp.with_untracked(|p| p.as_ref().map(format_tokens)).unwrap_or_default()
                 />
                 <InputBox
                        heading="Maximum direct participant icp"
                        placeholder="1000000 tokens"
                        updater=set_max_direct_participants_icp
                        validator=optional_tokens_validator
                        initial_value=max_direct_participants_icp.with_untracked(|p| p.as_ref().map(format_tokens)).unwrap_or_default()
                 />
                 <InputBox
                        heading="Minimum participant icp"
                        placeholder="10 tokens"
                        updater=set_min_participants_icp
                        validator=validate_tokens
                        initial_value=format_tokens(&min_participants_icp.get_untracked())
                 />
                 <InputBox
                        heading="Maximum participant icp"
                        placeholder="10,000 tokens"
                        updater=set_max_participants_icp
                        validator=validate_tokens
                        initial_value=format_tokens(&max_participants_icp.get_untracked())
                 />
                //  <InputBox
                //         heading="Restricted Country"
                //         placeholder="Antarctica"
                //         updater=set_restricted_country
                //         validator=non_empty_string_validator
                //  />
                 <button on:click=reset_settings class="w-full flex justify-center underline text-sm text-white my-4 " >Reset to default</button>
            </div>

    }
}
