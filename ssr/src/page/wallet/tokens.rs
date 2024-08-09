use candid::Principal;
use ic_agent::AgentError;

use crate::{
    canister::individual_user_template::Result13,
    component::{
        back_btn::BackButton,
        bullet_loader::BulletLoader,
        canisters_prov::AuthCansProvider,
        infinite_scroller::{CursoredDataProvider, InfiniteScroller, KeyedData, PageEntry},
        title::Title,
    },
    state::canisters::{unauth_canisters, Canisters},
    utils::{
        profile::propic_from_principal,
        token::{token_metadata_by_root, TokenMetadata},
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
        let user = self.0.authenticated_user().await?;
        let tokens = user
            .get_token_roots_of_this_user_with_pagination_cursor(start as u64, end as u64)
            .await?;
        let tokens = match tokens {
            Result13::Ok(v) => v,
            Result13::Err(_) => vec![],
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
    user_canister: Principal,
    token_root: Principal,
) -> TokenMetadata {
    let metadata = token_metadata_by_root(&cans, user_canister, token_root)
        .await
        .ok()
        .flatten();
    metadata.unwrap_or_else(|| TokenMetadata {
        logo_b64: propic_from_principal(token_root),
        name: "<ERROR>".to_string(),
        description: "Unknown".to_string(),
        symbol: "??".to_string(),
        balance: 0u32.into(),
    })
}

#[component]
fn FallbackToken() -> impl IntoView {
    view! {
        <div class="w-full items-center h-20 rounded-xl border-2 border-neutral-700 bg-white/15 animate-pulse"/>
    }
}

#[component]
pub fn TokenView(
    user_canister: Principal,
    token_root: Principal,
    #[prop(optional)] _ref: NodeRef<html::A>,
) -> impl IntoView {
    let cans = unauth_canisters();
    let info = create_resource(
        || (),
        move |_| token_metadata_or_fallback(cans.clone(), user_canister, token_root),
    );

    view! {
        <Suspense fallback=FallbackToken>
        {move || info.map(|info| view! {
            <a href=format!("/token/{token_root}") _ref=_ref class="grid grid-cols-2 grid-rows-1 w-full items-center p-4 rounded-xl border-2 border-neutral-700 bg-white/15">
                <div class="flex flex-row gap-2 items-center justify-self-start">
                    <img class="w-12 h-12 rounded-full" src=info.logo_b64.clone()/>
                    <span class="text-white truncate">{info.name.clone()}</span>
                </div>
                <div class="flex flex-row gap-2 items-center justify-self-end text-base text-white">
                    <span class="truncate">{format!("{} {}", info.balance, info.symbol)}</span>
                    <div class="flex items-center justify-center w-8 h-8 bg-white/15 rounded-full">
                        <Icon icon=icondata::BsSend/>
                    </div>
                </div>
            </a>
        })}
        </Suspense>
    }
}

#[component]
fn TokenList(canisters: Canisters<true>) -> impl IntoView {
    let user_canister = canisters.user_canister();
    let provider = TokenRootList(canisters);

    view! {
        <div class="flex flex-col w-full gap-2 items-center">
            <InfiniteScroller
                provider
                fetch_count=10
                children=move |token_root, _ref| {
                    view! { <TokenView user_canister token_root _ref=_ref.unwrap_or_default()/> }
                }
            />
        </div>
    }
}

#[component]
pub fn Tokens() -> impl IntoView {
    view! {
        <div class="felx items-center flex-col w-dvw min-h-dvh gap-6 bg-black pt-4 px-4 pb-12">
            <Title justify_center=false>
                <div class="flex flex-row justify-between">
                    <BackButton fallback="/wallet".to_string()/>
                    <span class="text-xl text-white font-bold">Tokens</span>
                    <div></div>
                </div>
            </Title>
            <AuthCansProvider fallback=BulletLoader let:canisters>
                <TokenList canisters/>
            </AuthCansProvider>
        </div>
    }
}
