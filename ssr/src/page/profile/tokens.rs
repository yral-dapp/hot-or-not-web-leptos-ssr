use candid::Principal;
use leptos::*;

use crate::{
    component::{
        bullet_loader::BulletLoader, canisters_prov::AuthCansProvider,
        claim_tokens::ClaimTokensOrRedirectError, token_confetti_symbol::TokenConfettiSymbol,
    },
    page::wallet::tokens::TokenTile,
    state::canisters::unauth_canisters,
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

    view! {
        <ClaimTokensOrRedirectError token_root=token.root/>
        <Suspense fallback=TokenViewFallback>
            {move || {
                token_info()
                    .and_then(|info| info.ok())
                    .map(|info| {
                        view! {
                            <TokenTile token_root=token.root.to_text() user_principal=user_principal.to_text() token_meta_data=info.clone() />
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
                {header_text} <br/> <span class="text-white">Meme Coin</span>
            </span>
            <TokenConfettiSymbol class="w-2/3 md:w-1/2 lg:w-1/3 mx-8"/>
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

    view! {
        <div class="flex flex-col w-full items-center gap-4">
            <Suspense fallback=|| {
                view! {
                    <div class="w-full flex justify-center items-center py-9">
                        <BulletLoader/>
                    </div>
                }
            }>
                {move || {
                    token_list()
                        .map(|tokens| tokens.unwrap_or_default())
                        .map(|tokens| {
                            let empty = tokens.is_empty();
                            view! {
                                {tokens
                                    .into_iter()
                                    .map(|token| view! { <TokenView user_principal token/> })
                                    .collect_view()}

                                <AuthCansProvider fallback=BulletLoader let:canisters>

                                    {
                                        let is_native_profile = canisters.user_principal()
                                            == user_principal;
                                        view! {
                                            <Show when=move || { empty }>
                                                <CreateYourToken header_text=if is_native_profile {
                                                    "Create your own"
                                                } else {
                                                    "They have not created any"
                                                }/>

                                            </Show>
                                            <Show when=move || { is_native_profile }>
                                                <a
                                                    href="/token/create"
                                                    class="text-xl bg-primary-600 py-4 w-2/3 md:w-1/2 lg:w-1/3 rounded-full text-center text-white"
                                                >
                                                    Create
                                                </a>
                                            </Show>
                                        }
                                    }

                                </AuthCansProvider>
                            }
                        })
                }}

            </Suspense>
        </div>
    }
}
