use candid::Principal;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

use super::TokenParams;
use crate::{
    component::{
        back_btn::BackButton, canisters_prov::WithAuthCans, spinner::FullScreenSpinner,
        title::Title,
    },
    state::canisters::Canisters,
    utils::token::{token_metadata_by_root, TokenMetadata},
};

#[component]
fn TokenField(#[prop(into)] label: String, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 w-full">
            <span class="text-sm text-white md:text-base">{label}</span>
            <p class="py-4 px-2 w-full text-base rounded-xl md:text-lg bg-white/5 text-white/50">
                {value}
            </p>
        </div>
    }
}

#[component]
fn TokenDetails(meta: TokenMetadata) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-6 p-4 w-full rounded-xl bg-white/5">
            <TokenField label="Description" value=meta.description/>
            <TokenField label="Symbol" value=meta.symbol/>
        </div>
    }
}

#[component]
fn TokenInfoInner(root: Principal, meta: TokenMetadata) -> impl IntoView {
    let meta_c = meta.clone();
    let detail_toggle = create_rw_signal(false);
    let view_detail_icon = Signal::derive(move || {
        if detail_toggle() {
            icondata::AiUpOutlined
        } else {
            icondata::AiDownOutlined
        }
    });

    view! {
        <div class="flex flex-col gap-4 w-dvw min-h-dvh bg-neutral-800">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback="/wallet"/>
                    <span class="justify-self-center font-bold">Token details</span>
                </div>
            </Title>
            <div class="flex flex-col gap-8 items-center px-8 w-full md:px-10">
                <div class="flex flex-col gap-6 justify-self-start items-center w-full md:gap-8">
                    <div class="flex flex-col gap-4 p-4 w-full rounded-xl bg-white/5 drop-shadow-lg">
                        <div class="flex flex-row justify-between items-center">
                            <div class="flex flex-row gap-2 items-center">
                                <img
                                    class="object-cover w-14 h-14 rounded-full md:w-18 md:h-18"
                                    src=meta.logo_b64
                                />
                                <span class="text-base font-semibold text-white md:text-lg">
                                    {meta.name}
                                </span>
                            </div>
                            <div class="p-1 rounded-full bg-white/15">
                                <Icon
                                    class="text-sm text-white md:text-base"
                                    icon=icondata::ChShare
                                />
                            </div>
                        </div>
                        <div class="flex flex-row justify-between items-center p-1 border-b border-white">
                            <span class="text-xs text-green-500 md:text-sm">Balance</span>
                            <span class="text-lg text-white md:text-xl">
                                <span class="font-bold">
                                    {format!("{} ", meta.balance.humanize())}
                                </span>
                                {meta.symbol}
                            </span>
                        </div>
                        <button
                            on:click=move |_| detail_toggle.update(|t| *t = !*t)
                            class="flex flex-row gap-2 justify-center items-center p-1 w-full text-white bg-transparent"
                        >
                            <span class="text-xs md:text-sm">View details</span>
                            <div class="p-1 rounded-full bg-white/15">
                                <Icon class="text-xs text-white md:text-sm" icon=view_detail_icon/>
                            </div>
                        </button>
                    </div>
                    <Show when=detail_toggle>
                        <TokenDetails meta=meta_c.clone()/>
                    </Show>
                </div>
                <a
                    href=format!("/token/transfer/{root}")
                    class="flex flex-row justify-center justify-self-center p-3 w-full text-white rounded-full md:w-1/2 md:text-lg bg-primary-600"
                >
                    Send
                </a>
            </div>
        </div>
    }
}

#[component]
pub fn TokenInfo() -> impl IntoView {
    let params = use_params::<TokenParams>();

    let token_metadata_fetch = move |cans: Canisters<true>| {
        create_resource(params, move |params| {
            let cans = cans.clone();
            async move {
                let Ok(params) = params else {
                    return Ok::<_, ServerFnError>(None);
                };
                // let user = cans.user_canister();
                let user_principal = cans.user_principal();
                let meta = token_metadata_by_root(&cans, user_principal, params.token_root).await?;
                Ok(meta.map(|m| (m, params.token_root)))
            }
        })
    };

    view! {
        <WithAuthCans fallback=FullScreenSpinner with=token_metadata_fetch let:info>
            {match info.1 {
                Err(e) => view! { <Redirect path=format!("/error?err={e}")/> },
                Ok(None) => view! { <Redirect path="/"/> },
                Ok(Some((meta, root))) => view! { <TokenInfoInner root meta/> },
            }}

        </WithAuthCans>
    }
}
