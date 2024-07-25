use leptos::*;
use leptos_icons::*;

use crate::{
    component::{back_btn::BackButton, img_to_png::ImgToPng, title::Title},
    state::canisters::auth_canisters_store,
    utils::web::FileWithUrl,
};

#[component]
fn TokenImgInput(logo_b64: RwSignal<Option<String>>) -> impl IntoView {
    let img_file = create_rw_signal(None::<FileWithUrl>);

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
            set_invalid_cnt: WriteSignal<u32>,
        ) -> impl IntoView {
            let error = create_rw_signal(initial_value.is_none());
            let show_error = create_rw_signal(false);
            if error.get_untracked() {
                set_invalid_cnt.update(|c| *c += 1);
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
                                    set_invalid_cnt.update(|c| *c -= 1);
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
                                set_invalid_cnt.update(|c| *c += 1);
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

input_component!(InputBox, input, {});
input_component!(InputArea, textarea, rows = 4);

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
    let (invalid_cnt, set_invalid_cnt) = create_signal(0);
    let logo_b64 = create_rw_signal(None::<String>);
    let create_disabled =
        create_memo(move |_| logo_b64.with(|l| l.is_none()) || invalid_cnt() != 0);

    view! {
        <div class="w-dvw min-h-dvh bg-black pt-4 flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback=fallback_url/>
                    <span class="font-bold justify-self-center">Create a Token</span>
                </div>
            </Title>
            <div class="flex flex-col w-full px-6 md:px-8 gap-6 md:gap-8">
                <div class="flex flex-row w-full justify-between items-center">
                    <InputBox heading="Token name" placeholder="Name" updater=|_v| {} validator=Some set_invalid_cnt />
                    <TokenImgInput logo_b64/>
                </div>
                <InputArea heading="Description" placeholder="Text" updater=|_v| {} validator=Some set_invalid_cnt />
                <InputBox heading="Token Symbol" placeholder="Text" updater=|_v| {} validator=Some set_invalid_cnt />
                <div class="w-full flex justify-center">
                    <button disabled=create_disabled class="text-white disabled:text-neutral-500 md:text-xl py-4 md:py-4 font-bold w-full md:w-1/2 lg:w-1/3 rounded-full bg-primary-600 disabled:bg-primary-500/30">Create</button>
                </div>
            </div>
        </div>
    }
}
