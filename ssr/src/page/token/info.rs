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
            <span class="text-white text-sm md:text-base">{label}</span>
            <p class="bg-white/5 text-base md:text-lg text-white/50 px-2 py-4 rounded-xl w-full">{value}</p>
        </div>
    }
}

#[component]
fn TokenDetails(meta: TokenMetadata) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full gap-6 p-4 rounded-xl bg-white/5">
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
        <div class="w-dvw min-h-dvh bg-neutral-800 flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback="/wallet"/>
                    <span class="font-bold justify-self-center">Token details</span>
                </div>
            </Title>
            <div class="grid grid-rows-2 w-full px-8 md:px-10 items-center">
                <div class="flex flex-col justify-self-start w-full gap-6 md:gap-8 items-center">
                    <div class="flex flex-col gap-4 w-full bg-white/5 p-4 drop-shadow-lg rounded-xl">
                        <div class="flex flex-row justify-between items-center">
                            <div class="flex flex-row gap-2 items-center">
                                <img class="object-contain h-14 w-14 md:w-18 md:h-18 rounded-full" src=meta.logo_b64 />
                                <span class="text-base md:text-lg font-semibold text-white">{meta.name}</span>
                            </div>
                            <div class="p-1 bg-white/15 rounded-full">
                                <Icon class="text-sm md:text-base text-white" icon=icondata::BsArrowUpRight/>
                            </div>
                        </div>
                        <div class="flex flex-row justify-between border-b p-1 border-white items-center">
                            <span class="text-xs md:text-sm text-green-500">Balance</span>
                            <span class="text-lg md:text-xl text-white">
                                <span class="font-bold">{format!("{} ", meta.balance.humanize())}</span>
                                {meta.symbol}
                            </span>
                        </div>
                        <button
                            on:click=move |_| detail_toggle.update(|t| *t = !*t)
                            class="w-full bg-transparent p-1 flex flex-row justify-center items-center gap-2 text-white"
                        >
                            <span class="text-xs md:text-sm">View details</span>
                            <div class="p-1 bg-white/15 rounded-full">
                                <Icon class="text-xs md:text-sm text-white" icon=view_detail_icon/>
                            </div>
                        </button>
                    </div>
                    <Show when=detail_toggle>
                        <TokenDetails meta=meta_c.clone() />
                    </Show>
                </div>
                <a href=format!("/token/transfer/{root}") class="flex flex-row justify-self-end justify-center text-white md:text-lg w-full md:w-1/2 rounded-full p-3 bg-primary-600">
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
            Err(e) => view! { <Redirect path=format!("/error?err={e}") /> },
            Ok(None) => view! { <Redirect path="/" /> },
            Ok(Some((meta, root))) => view! { <TokenInfoInner root meta /> },
        }}
        </WithAuthCans>
    }
}
