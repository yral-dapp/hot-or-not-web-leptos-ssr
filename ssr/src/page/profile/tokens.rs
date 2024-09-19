use candid::Principal;
use leptos::*;
use leptos_icons::*;

use crate::{
    component::{
        bullet_loader::BulletLoader, claim_tokens::ClaimTokensOrRedirectError,
        token_confetti_symbol::TokenConfettiSymbol,
    },
    state::canisters::{authenticated_canisters, unauth_canisters},
    utils::token::{get_token_metadata, TokenCans},
};

#[component]
fn TokenViewFallback() -> impl IntoView {
    view! {
        <div class="w-full h-20 rounded-xl border-2 border-neutral-700 bg-white/15 animate-pulse"></div>
    }
}

#[component]
fn TokenView(user_principal: Principal, token: TokenCans) -> impl IntoView {
    let token_info = create_resource(
        || (),
        move |_| async move {
            let cans = unauth_canisters();
            let metadata =
                get_token_metadata(&cans, user_principal, token.governance, token.ledger).await?;

            Ok::<_, ServerFnError>(metadata)
        },
    );
    let token_link = move || format!("/token/info/{}/{}", token.root, user_principal);

    view! {
        <ClaimTokensOrRedirectError token_root=token.root />
        <Suspense fallback=TokenViewFallback>
            {move || {
                token_info()
                    .and_then(|info| info.ok())
                    .map(|info| {
                        view! {
                            <a
                                href=token_link()
                                class="w-full grid grid-cols-2 p-4 rounded-xl border-2 items-center border-neutral-700 bg-white/15"
                            >
                                <div class="flex flex-row gap-2 items-center justify-self-start">
                                    <img class="w-12 h-12 rounded-full" src=info.logo_b64 />
                                    <span class="text-white truncate">{info.name}</span>
                                </div>
                                <div class="flex flex-col gap-2 justify-self-end text-sm">
                                    <span class="text-white truncate">
                                        {format!("{} {}", info.balance.humanize(), info.symbol)}
                                    </span>
                                    <div class="flex flex-row gap-1 items-center">
                                        <span class="text-white">Details</span>
                                        <div class="flex items-center justify-center w-4 h-4 bg-white/15 rounded-full">
                                            <Icon class="text-white" icon=icondata::AiRightOutlined />
                                        </div>
                                    </div>
                                </div>
                            </a>
                        }
                    })
            }}

        </Suspense>
    }
}

#[component]
fn CreateYourToken(header_text: &'static str) -> impl IntoView {
    view! {
        <div class="w-full flex flex-col items-center gap-4">
            <span class="text-2xl text-primary-600 text-center">
                { header_text } <br /> <span class="text-white">Meme Coin</span>
            </span>
            <TokenConfettiSymbol class="w-2/3 md:w-1/2 lg:w-1/3 mx-8" />
        </div>
    }
}

#[component]
pub fn ProfileTokens(user_canister: Principal, user_principal: Principal) -> impl IntoView {
    let token_list = create_resource(
        || (),
        move |_| async move {
            let cans = unauth_canisters();
            let user = cans.individual_user(user_canister).await;
            let tokens: Vec<_> = user
                .deployed_cdao_canisters()
                .await?
                .into_iter()
                .map(|cans| TokenCans {
                    governance: cans.governance,
                    ledger: cans.ledger,
                    root: cans.root,
                })
                .collect();
            Ok::<_, ServerFnError>(tokens)
        },
    );

    let cans_res = authenticated_canisters();

    view! {
        <div class="flex flex-col w-full items-center gap-4">
            <Suspense fallback=|| {
                view! {
                    <div class="w-full flex justify-center items-center py-9">
                        <BulletLoader />
                    </div>
                }
            }>
                {move || {
                    token_list()
                        .map(|tokens| tokens.unwrap_or_default())
                        .map(|tokens| {
                            let empty = tokens.is_empty();
                            let canisters = (cans_res.0)().unwrap().ok().unwrap().canisters().ok().unwrap();
                            let is_native_profile = canisters.profile_details().principal == user_principal;
                            view! {
                                {tokens
                                    .into_iter()
                                    .map(|token| view! { <TokenView user_principal token /> })
                                    .collect_view()}
                                <Show when=move || { empty }>
                                    <CreateYourToken header_text={
                                        if is_native_profile {
                                            "Create your own"
                                        } else {
                                            "They have not created any"
                                        }
                                    }
                                    />
                                </Show>
                                <Show when=move || is_native_profile>
                                    <a
                                        href="/token/create"
                                        class="text-xl bg-primary-600 py-4 w-2/3 md:w-1/2 lg:w-1/3 rounded-full text-center text-white"
                                    >
                                        Create
                                    </a>
                                </Show>
                            }
                        })
                }}

            </Suspense>
        </div>
    }
}
