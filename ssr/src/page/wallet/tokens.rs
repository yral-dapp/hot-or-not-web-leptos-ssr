use candid::Principal;
use ic_agent::AgentError;
use leptos_use::use_window;

use crate::page::wallet::SharePopup;
use crate::utils::web::{check_share_support, share_url};
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

    view! {
        <ClaimTokensOrRedirectError token_root />
        <Suspense fallback=FallbackToken>
            {move || {

                info.map(|info| {
                    let base_url = || {
                        use_window().as_ref().and_then(|w| w.location().origin().ok())
                    };
                    let username_or_principal = user_principal.to_text().clone();
                    let principal = user_principal.to_text().clone();
                    let share_link = base_url()
                        .map(|b| format!("{b}/profile/{}?tab=tokens", &username_or_principal))
                        .unwrap_or_default();
                    let message = format!(
                        "Hey! Check out my YRAL profile 👇 {}. I just minted my own token—come see and create yours! 🚀 #YRAL #TokenMinter",
                        share_link.clone(),
                    );
                    let share_action = create_action(move |&()| async move { Ok(()) });
                    let link = share_link.clone();
                    let share_profile_url = move || {
                        let has_share_support = check_share_support();
                        match has_share_support {
                            Some(_) => {
                                share_url(&link);
                            }
                            None => {
                                share_action.dispatch(());
                            }
                        };
                    };
                    view! {
                        <div class="relative w-full grid grid-cols-[1fr,auto] items-center p-4 rounded-xl border-2 border-neutral-700 bg-white/15">
                            <a
                                href=format!("/token/info/{token_root}/{principal}")
                                class="flex items-center w-full"
                            >
                                <div class="flex flex-row flex-grow gap-2 items-center">
                                    <img class="w-12 h-12 rounded-full" src=info.logo_b64.clone() />
                                    <span class="text-white truncate">{info.name.clone()}</span>
                                </div>
                                <div
                                    class="flex flex-row gap-2 items-center ml-auto text-base text-white"
                                    style="padding-right:33px; "
                                >
                                    <span class="truncate">
                                        {format!("{} {}", info.balance.humanize(), info.symbol)}
                                    </span>
                                </div>
                            </a>

                            <button
                                on:click=move |event| {
                                    event.prevent_default();
                                    event.stop_propagation();
                                    share_profile_url();
                                }
                                class="absolute right-4 top-1/2 p-1 text-lg text-center text-white rounded-full transform -translate-y-1/2 md:text-xl bg-primary-600"
                            >
                                <Icon icon=icondata::AiShareAltOutlined />
                            </button>

                            <ShareProfilePopup sharing_action=share_action share_link message />
                        </div>
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
                    view! { <TokenView user_principal token_root _ref=_ref.unwrap_or_default() /> }
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
                    <BackButton fallback="/wallet".to_string() />
                    <span class="text-xl font-bold text-white">Tokens</span>
                    <div></div>
                </div>
            </Title>
            <AuthCansProvider fallback=BulletLoader let:canisters>
                <TokenList canisters />
            </AuthCansProvider>
        </div>
    }
}
