use candid::Principal;
use ic_agent::AgentError;

use crate::component::infinite_scroller::KeyedCursoredDataProvider;
use crate::page::wallet::ShareButtonWithFallbackPopup;
use crate::utils::token::TokenBalanceOrClaiming;
use crate::{
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
        token::{token_metadata_by_root, TokenBalance, TokenMetadata},
    },
};
use leptos::*;
use yral_canisters_client::individual_user_template::{IndividualUserTemplate, Result14};

#[derive(Clone)]
pub struct TokenRootList(pub Canisters<true>);


impl KeyedData for Principal {
    type Key = Principal;

    fn key(&self) -> Self::Key {
        *self
    }
}
impl<'a> KeyedCursoredDataProvider<IndividualUserTemplate<'a>> for TokenRootList{
    async fn get_by_cursor_by_key(
            &self,
            start: usize,
            end: usize,
            user: IndividualUserTemplate<'a>,
        ) -> Result<PageEntry<Self::Data>, Self::Error> {
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
        balance: TokenBalanceOrClaiming::claiming(),
        fees: TokenBalance::new_cdao(0u32.into()),
        root: Principal::anonymous(),
    })
}

#[component]
pub fn TokenViewFallback() -> impl IntoView {
    view! {
        <div class="w-full items-center h-16 rounded-xl border-2 border-neutral-700 bg-white/15 animate-pulse"></div>
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

    view! {
        <Suspense fallback=TokenViewFallback>
            {move || {
                info.map(|info| {
                    view! {
                        <TokenTile user_principal=user_principal.to_text() token_meta_data=info.clone() />
                    }
                })
            }}

        </Suspense>
    }
}

#[component]
pub fn TokenTile(user_principal: String, token_meta_data: TokenMetadata) -> impl IntoView {
    let root = token_meta_data.root;
    let share_link = format!("/token/info/{root}/{user_principal}?airdrop_amt=100");
    let share_link_s = store_value(share_link);
    let share_message = format!(
        "Hey! Check out the token: {} I created on YRAL 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
        token_meta_data.symbol,
        share_link_s(),
    );
    let share_message_s = store_value(share_message);
    let info = token_meta_data;
    view! {
        <div
            class="flex  w-full items-center h-16 rounded-xl border-2 border-neutral-700 bg-white/15 gap-1"
        >
            <a
            href=format!("/token/info/{root}/{user_principal}?airdrop_amt=100")
            // _ref=_ref
            class="flex flex-1  p-y-4"
            >
                <div class="flex flex-2 items-center space-x-2 px-2">
                <img class="w-12 h-12 rounded-full" src=info.logo_b64.clone()/>
                <span class="text-white text-xs truncate">{info.name.clone()}</span>
                </div>
                <div class="flex flex-1 flex-col">
                    <span
                    class="flex flex-1  items-center justify-end text-xs text-white">
                    {info.balance.humanize_float_truncate_to_dp(2)}
                    </span>
                    <span
                    class="flex flex-1  items-center justify-end text-xs text-white truncate">
                    {info.symbol.clone()}
                    </span>
                </div>

            </a>
            <div>
                <ShareButtonWithFallbackPopup
                    share_link=share_link_s()
                    message=share_message_s()
                    style="w-12 h-12".into()
                />
            </div>

        </div>
    }
}

#[component]
fn TokenList(canisters: Canisters<true>) -> impl IntoView {
    // let user_canister = canisters.user_canister();
    let user_principal = canisters.user_principal();
    let provider: TokenRootList = TokenRootList(canisters);

    view! {
        <div class="flex flex-col w-full gap-2 items-center">
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
        <div class="flex items-center flex-col w-dvw min-h-dvh gap-6 bg-black pt-4 px-4 pb-12">
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
