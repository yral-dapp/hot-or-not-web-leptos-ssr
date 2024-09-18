use crate::page::token::popups::ShareProfilePopup;
use candid::Principal;
use ic_agent::AgentError;

use crate::{
    canister::individual_user_template::Result14,
    component::{
        back_btn::BackButton,
        bullet_loader::BulletLoader,
        canisters_prov::AuthCansProvider,
        claim_tokens::ClaimTokensOrRedirectError,
        infinite_scroller::{CursoredDataProvider, InfiniteScroller, KeyedData, PageEntry},
        title::Title,
    },
    state::canisters::{unauth_canisters, Canisters},
    utils::{
        profile::propic_from_principal,
        token::{token_metadata_by_root, TokenBalance, TokenMetadata},
    },
};
use leptos::*;
use leptos_icons::*;

#[derive(Clone)]
pub struct TokenRootList(pub Canisters<true>);

impl KeyedData for Principal {
    type Key = Principal;

    fn key(&self) -> Self::Key {
        *self
    }
}

impl CursoredDataProvider for TokenRootList {
    type Data = Principal;
    type Error = AgentError;

    async fn get_by_cursor(
        &self,
        start: usize,
        end: usize,
    ) -> Result<PageEntry<Self::Data>, Self::Error> {
        let user = self.0.authenticated_user().await;
        let tokens = user
            .get_token_roots_of_this_user_with_pagination_cursor(start as u64, end as u64)
            .await?;
        let tokens = match tokens {
            Result14::Ok(v) => v,
            Result14::Err(_) => vec![],
        };
        let list_end = tokens.len() < (end - start);
        Ok(PageEntry {
            data: tokens,
            end: list_end,
        })
    }
}

async fn token_metadata_or_fallback(
    cans: Canisters<false>,
    user_principal: Principal,
    token_root: Principal,
) -> TokenMetadata {
    let metadata = token_metadata_by_root(&cans, user_principal, token_root)
        .await
        .ok()
        .flatten();
    metadata.unwrap_or_else(|| TokenMetadata {
        logo_b64: propic_from_principal(token_root),
        name: "<ERROR>".to_string(),
        description: "Unknown".to_string(),
        symbol: "??".to_string(),
        balance: TokenBalance::new_cdao(0u32.into()),
        fees: TokenBalance::new_cdao(0u32.into()),
    })
}

#[component]
fn FallbackToken() -> impl IntoView {
    view! {
        <div class="items-center w-full h-20 rounded-xl border-2 animate-pulse border-neutral-700 bg-white/15"></div>
    }
}

#[component]
pub fn TokenView(
    user_principal: Principal,
    token_root: Principal,
    #[prop(optional)] _ref: NodeRef<html::A>,
) -> impl IntoView {
    let cans = unauth_canisters();

    let info = create_resource(
        || (),
        move |_| token_metadata_or_fallback(cans.clone(), user_principal, token_root),
    );

    let share_action = create_action(move |&()| async move { Ok(()) });
    // let creating = create_action.pending();
    //
    // let create_disabled = create_memo(move |_| {
    //     creating()
    //         || auth_cans.with(|c| c.is_none())
    //         || ctx.form_state.with(|f| f.logo_b64.is_none())
    //         || ctx.form_state.with(|f: &SnsFormState| f.name.is_none())
    //         || ctx
    //             .form_state
    //             .with(|f: &SnsFormState| f.description.is_none())
    //         || ctx.form_state.with(|f| f.symbol.is_none())
    //         || ctx.invalid_cnt.get() != 0
    // });

    view! {
        <ClaimTokensOrRedirectError token_root/>
        <Suspense fallback=FallbackToken>
            {move || {
                info.map(|info| {
                    view! {
                        <a
                            href=format!("/token/info/{token_root}")
                            _ref=_ref
                            class="grid grid-cols-2 grid-rows-1 items-center p-4 w-full rounded-xl border-2 border-neutral-700 bg-white/15"
                        >
                            <div class="flex flex-row gap-2 justify-self-start items-center">
                                <img class="w-12 h-12 rounded-full" src=info.logo_b64.clone()/>
                                <span class="text-white truncate">{info.name.clone()}</span>
                            </div>
                            <div class="flex flex-row gap-2 justify-self-end items-center text-base text-white">
                                <span class="truncate">
                                    {format!("{} {}", info.balance.humanize(), info.symbol)}
                                </span>
                        <button
                        on:click=move |_| share_action.dispatch(())>
                                <div class="flex justify-center items-center w-8 h-8 rounded-full bg-white/15">
                                    <Icon icon=icondata::ChShare/>
                                </div>
                    </button>
                            </div>
                                           {move ||
                            view! {
                                <ShareProfilePopup
                                    sharing_action=share_action

                                />

                        } }
                    </a>
                    }
                })
            }}

        </Suspense>
    }
}

#[component]
fn TokenList(canisters: Canisters<true>) -> impl IntoView {
    // let user_canister = canisters.user_canister();
    let user_principal = canisters.user_principal();
    let provider: TokenRootList = TokenRootList(canisters);

    view! {
        <div class="flex flex-col gap-2 items-center w-full">
            <InfiniteScroller
                provider
                fetch_count=10
                children=move |token_root, _ref| {
                    view! { <TokenView user_principal token_root _ref=_ref.unwrap_or_default()/> }
                }
            />

        </div>
    }
}

#[component]
pub fn Tokens() -> impl IntoView {
    view! {
        <div class="flex-col gap-6 items-center px-4 pt-4 pb-12 bg-black felx w-dvw min-h-dvh">
            <Title justify_center=false>
                <div class="flex flex-row justify-between">
                    <BackButton fallback="/wallet".to_string()/>
                    <span class="text-xl font-bold text-white">Tokens</span>
                    <div></div>
                </div>
            </Title>
            <AuthCansProvider fallback=BulletLoader let:canisters>
                <TokenList canisters/>
            </AuthCansProvider>
        </div>
    }
}
