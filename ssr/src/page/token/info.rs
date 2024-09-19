use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::use_window;

use crate::{
    component::{back_btn::BackButton, spinner::FullScreenSpinner, title::Title},
    page::{token::TokenInfoParams, wallet::share_token_popup::ShareProfilePopup},
    state::canisters::unauth_canisters,
    utils::token::{token_metadata_by_root, TokenMetadata},
};

#[component]
fn TokenField(#[prop(into)] label: String, #[prop(into)] value: String) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-1 w-full">
            <span class="text-white text-sm md:text-base">{label}</span>
            <p class="bg-white/5 text-base md:text-lg text-white/50 px-2 py-4 rounded-xl w-full">
                {value}
            </p>
        </div>
    }
}

#[component]
fn TokenDetails(meta: TokenMetadata) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full gap-6 p-4 rounded-xl bg-white/5">
            <TokenField label="Description" value=meta.description />
            <TokenField label="Symbol" value=meta.symbol />
        </div>
    }
}

#[component]
fn TokenInfoInner(meta: TokenMetadata, principal: String, root: String) -> impl IntoView {
    let meta_c = meta.clone();
    let detail_toggle = create_rw_signal(false);
    let view_detail_icon = Signal::derive(move || {
        if detail_toggle() {
            icondata::AiUpOutlined
        } else {
            icondata::AiDownOutlined
        }
    });

    let base_url = || {
        use_window()
            .as_ref()
            .and_then(|w| w.location().origin().ok())
    };

    let share_link = base_url()
        .map(|b| format!("{b}/token/info/{root}/{principal}"))
        .unwrap_or_default();

    let share_action = create_action(move |&()| async move { Ok(()) });

    // let share_profile_url = move || {
    //     // let url = base_url()
    //     //     .map(|b| format!("{b}/token/info/{root}/{principal}"))
    //     //     .unwrap_or_default();
    //     // share_url(&url);
    // };

    let message = format!(
        "Hey! Check out the token: {} I created on YRAL 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        meta.symbol,  share_link.clone()
    );
    let share_profile_url = move || {
        // let url = base_url()
        //     .map(|b| format!("{b}/profile/{}?tab=tokens", username_or_principal))
        //     .unwrap_or_default();
        // share_url(&url);

        share_action.dispatch(());
    };

    view! {
        <div class="w-dvw min-h-dvh bg-neutral-800 flex flex-col gap-4">
            <Title justify_center=false>
                <div class="grid grid-cols-3 justify-start w-full">
                    <BackButton fallback="/wallet" />
                    <span class="font-bold justify-self-center">Token details</span>
                </div>
            </Title>
            <div class="flex flex-col w-full px-8 md:px-10 items-center gap-8">
                <div class="flex flex-col justify-self-start w-full gap-6 md:gap-8 items-center">
                    <div class="flex flex-col gap-4 w-full bg-white/5 p-4 drop-shadow-lg rounded-xl">
                        <div class="flex flex-row justify-between items-center">
                            <div class="flex flex-row gap-2 items-center">
                                <img
                                    class="object-cover h-14 w-14 md:w-18 md:h-18 rounded-full"
                                    src=meta.logo_b64
                                />
                                <span class="text-base md:text-lg font-semibold text-white">
                                    {meta.name}
                                </span>
                            </div>
                            <button
                                on:click= move|_| share_profile_url()
                                class="text-white text-center p-1 text-lg md:text-xl bg-primary-600 rounded-full"
                            >
                            <Icon icon=icondata::AiShareAltOutlined/>

                            </button>
                            <ShareProfilePopup
                                sharing_action=share_action
                                share_link
                                message

                            />
                        </div>
                        <div class="flex flex-row justify-between border-b p-1 border-white items-center">
                            <span class="text-xs md:text-sm text-green-500">Balance</span>
                            <span class="text-lg md:text-xl text-white">
                                <span class="font-bold">
                                    {format!("{} ", meta.balance.humanize())}
                                </span>
                                {meta.symbol}
                            </span>
                        </div>
                    <button
                            on:click=move |_| detail_toggle.update(|t| *t = !*t)
                            class="w-full bg-transparent p-1 flex flex-row justify-center items-center gap-2 text-white"
                        >
                            <span class="text-xs md:text-sm">View details</span>
                            <div class="p-1 bg-white/15 rounded-full">
                                <Icon class="text-xs md:text-sm text-white" icon=view_detail_icon />
                            </div>
                        </button>
                    </div>
                    <Show when=detail_toggle>
                        <TokenDetails meta=meta_c.clone() />
                    </Show>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn TokenInfo() -> impl IntoView {
    let params = use_params::<TokenInfoParams>();

    let token_metadata_fetch = create_resource(params, move |params| async move {
        let Ok(params) = params else {
            return Ok::<_, ServerFnError>(None);
        };
        let principal = params.user_principal.to_text().clone();
        let root = params.token_root.to_text().clone();

        let cans = unauth_canisters();
        let meta = token_metadata_by_root(&cans, params.user_principal, params.token_root).await?;
        Ok(Some((meta, principal, root)))
    });

    view! {
        <Suspense fallback=FullScreenSpinner>
            {move || {
                token_metadata_fetch()
                    .and_then(|info| info.ok())
                    .map(|info| {
                        match info {
                            Some((Some(metadata), principal, root)) => view! { <TokenInfoInner meta=metadata principal=principal root=root /> },
                            _ => view! { <Redirect path="/" /> },
                        }
                    })
            }}
        </Suspense>
    }
}
