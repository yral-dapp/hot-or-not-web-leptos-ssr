use leptos::*;
use leptos_icons::*;

use crate::{
    component::{back_btn::BackButton, img_to_png::ImgToPng, title::Title},
    state::canisters::auth_canisters_store,
    utils::web::FileWithUrl,
};

use super::sns_form::SnsFormState;

#[component]
fn TokenImgInput() -> impl IntoView {
    let ctx = expect_context::<CreateTokenCtx>();
    let img_file = create_rw_signal(None::<FileWithUrl>);
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
                img_file.set(Some(FileWithUrl::new(file.into())));
            }
            Some(())
        })
    };
    let img_url = Signal::derive(move || img_file.with(|f| f.as_ref().map(|f| f.url.to_string())));

    view! {
        <div class="ml-2 md:ml-4 h-20 w-20 md:w-36 md:h-36 rounded-full">
            <label for="dropzone-logo" class="flex flex-col h-full w-full cursor-pointer">
                <Show when=move || img_url.with(|u| u.is_none()) fallback=move || view! {
                    <img class="object-contain h-full w-full rounded-full" src=move || img_url().unwrap() />
                }>
                    <div class="flex flex-col items-center justify-center h-full w-full bg-white/10 rounded-full border-2 border-dashed border-neutral-600 hover:bg-white/15">
                        <Icon icon=icondata::BiImageAddSolid class="text-center bg-white/30 text-4xl"/>
                    </div>
                </Show>
                <input
                    on:change=on_file_input
                    id="dropzone-logo"
                    type="file"
                    accept="image/*"
                    class="sr-only"
                />
            </label>
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
            updater: U,
            validator: V,
        ) -> impl IntoView {
            let ctx: CreateTokenCtx = expect_context();
            let error = create_rw_signal(initial_value.is_none());
            let show_error = create_rw_signal(false);
            if error.get_untracked() {
                ctx.invalid_cnt.update(|c| *c += 1);
            }

            view! {
                <div class="flex flex-col grow gap-y-2 text-sm md:text-base">
                    <Show when=move || show_error() && error() fallback=move || view! { <span class="text-white font-semibold">{heading.clone()}</span> }>
                        <span class="text-red-500 font-semibold">Invalid</span>
                    </Show>
                    <$input_element prop:value=initial_value.unwrap_or_default() on:input=move |ev| {
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
                        class="p-3 py-4 md:p-4 md:py-5 text-white outline-none bg-white/10 border-2 border-solid border-white/20 rounded-xl placeholder-neutral-600"
                        type="text"
                    />
                </div>
            }
        }
    }
}

fn non_empty_string_validator(s: String) -> Option<String> {
    (!s.is_empty()).then_some(s)
}

input_component!(InputBox, input, {});
input_component!(InputArea, textarea, rows = 4);

#[derive(Clone, Copy, Default)]
struct CreateTokenCtx {
    form_state: RwSignal<SnsFormState>,
    invalid_cnt: RwSignal<u32>,
}

#[component]
pub fn CreateToken() -> impl IntoView {
    let auth_cans = auth_canisters_store();
    let fallback_url = Signal::derive(move || {
        let Some(cans) = auth_cans() else {
            return "/menu".to_string();
        };
        let id = cans.profile_details().username_or_principal();
        format!("/your-profile/{id}")
    });
    let ctx = CreateTokenCtx::default();
    provide_context(ctx);

    let set_token_name = move |name: String| {
        ctx.form_state.update(|f| f.name = Some(name));
    };
    let set_token_symbol = move |symbol: String| {
        ctx.form_state.update(|f| f.symbol = Some(symbol));
    };
    let set_token_desc = move |desc: String| {
        ctx.form_state.update(|f| f.description = Some(desc));
    };

    let create_action = create_action(move |&()| async move {
        let cans = auth_cans
            .get_untracked()
            .expect("Create token called without auth canisters");
        let sns_form = ctx.form_state.get_untracked();
        let sns_config = sns_form.try_into_config(cans)?;

        let _create_sns = sns_config.try_convert_to_executed_sns_init()?;
        // TODO: send call to canister
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
        <div class="w-dvw min-h-dvh bg-black pt-4 flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback=fallback_url/>
                    <span class="font-bold justify-self-center">Create a Token</span>
                </div>
            </Title>
            <div class="flex flex-col w-full px-6 md:px-8 gap-6 md:gap-8">
                <Show when=move || create_act_res.with(|v| v.as_ref().map(|v| v.is_err()).unwrap_or_default())>
                    <div class="flex flex-col w-full items-center gap-2">
                        <span class="text-red-500 font-semibold text-center">Error creating token:</span>
                        <textarea
                            prop:value=create_act_res().unwrap().err().unwrap()
                            disabled
                            rows=3
                            class="bg-white/10 text-xs md:text-sm text-red-500/60 w-full md:w-2/3 resize-none p-2"
                        />
                    </div>
                </Show>
                <div class="flex flex-row w-full justify-between items-center">
                    <InputBox heading="Token name" placeholder="Name" updater=set_token_name validator=non_empty_string_validator />
                    <TokenImgInput/>
                </div>
                <InputArea heading="Description" placeholder="Text" updater=set_token_desc validator=non_empty_string_validator />
                <InputBox heading="Token Symbol" placeholder="Text" updater=set_token_symbol validator=non_empty_string_validator />
                <div class="w-full flex justify-center">
                    <button on:click=move |_| create_action.dispatch(()) disabled=create_disabled class="text-white disabled:text-neutral-500 md:text-xl py-4 md:py-4 font-bold w-full md:w-1/2 lg:w-1/3 rounded-full bg-primary-600 disabled:bg-primary-500/30">
                        {move || if creating() { "Creating..." } else { "Create" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
