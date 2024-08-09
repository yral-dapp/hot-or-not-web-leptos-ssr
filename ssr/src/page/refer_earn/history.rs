use leptos::*;

use crate::component::bullet_loader::BulletLoader;
use crate::component::canisters_prov::AuthCansProvider;
use crate::component::infinite_scroller::InfiniteScroller;
use crate::{
    state::canisters::Canisters,
    utils::{profile::propic_from_principal, timestamp::get_day_month},
};
use history_provider::*;

#[component]
fn HistoryItem(detail: HistoryDetails, _ref: NodeRef<html::Div>) -> impl IntoView {
    view! {
        <div _ref=_ref class="px-2 grid grid-cols-4 grid-rows-1 items-center gap-2 w-full">
            <div class="flex flex-row col-span-3 items-center gap-4 justify-items-start">
                <img
                    class="aspect-square w-12 md:w-16 lg:w-24 rounded-full"
                    src=propic_from_principal(detail.referee)
                />
                <div class="grid grid-cols-1 grid-rows-2">
                    <span class="text-white text-lg truncate">{detail.referee.to_text()}</span>
                    <span class="text-white/50 text-sm md:text-md">
                        {get_day_month(detail.epoch_secs)}
                    </span>
                </div>
            </div>
            <span class="text-white text-md md:text-xl text-center justify-self-end">
                {detail.amount} Coyns
            </span>
        </div>
    }
}

#[component]
fn AuthenticatedHistory(canisters: Canisters<true>) -> impl IntoView {
    let provider = get_history_provider(canisters);
    view! {
        <div class="flex flex-col justify-center w-full md:w-10/12 lg:w-8/12 gap-4 pb-16">
            <InfiniteScroller
                provider
                fetch_count=10
                children=|detail, _ref| {
                    view! { <HistoryItem detail _ref=_ref.unwrap_or_default()/> }
                }
            />

        </div>
    }
}

#[component]
pub fn HistoryView() -> impl IntoView {
    view! {
        <AuthCansProvider fallback=BulletLoader let:canisters>
            <AuthenticatedHistory canisters/>
        </AuthCansProvider>
    }
}

mod history_provider {
    use candid::Principal;

    use crate::{
        component::infinite_scroller::{CursoredDataProvider, KeyedData, PageEntry},
        state::canisters::Canisters,
    };

    #[derive(Clone, Copy)]
    pub struct HistoryDetails {
        pub epoch_secs: u64,
        pub referee: Principal,
        pub amount: u64,
    }

    impl KeyedData for HistoryDetails {
        type Key = Principal;

        fn key(&self) -> Self::Key {
            self.referee
        }
    }

    pub fn get_history_provider(
        canisters: Canisters<true>,
    ) -> impl CursoredDataProvider<Data = HistoryDetails> + Clone {
        #[cfg(feature = "mock-referral-history")]
        {
            _ = canisters;
            mock::MockHistoryProvider
        }
        #[cfg(not(feature = "mock-referral-history"))]
        {
            canister::ReferralHistory(canisters)
        }
    }

    #[cfg(not(feature = "mock-referral-history"))]
    mod canister {
        use super::*;
        use ic_agent::AgentError;

        #[derive(Clone)]
        pub struct ReferralHistory(pub Canisters<true>);

        impl CursoredDataProvider for ReferralHistory {
            type Data = HistoryDetails;
            type Error = AgentError;

            async fn get_by_cursor(
                &self,
                from: usize,
                end: usize,
            ) -> Result<PageEntry<HistoryDetails>, AgentError> {
                use crate::canister::individual_user_template::{MintEvent, Result14, TokenEvent};
                use crate::utils::route::failure_redirect;
                let individual = self.0.authenticated_user().await?;
                let history = individual
                    .get_user_utility_token_transaction_history_with_pagination(
                        from as u64,
                        end as u64,
                    )
                    .await?;
                let history = match history {
                    Result14::Ok(history) => history,
                    Result14::Err(_) => {
                        failure_redirect("failed to get posts");
                        return Ok(PageEntry {
                            data: vec![],
                            end: true,
                        });
                    }
                };
                let list_end = history.len() < (end - from);
                let details = history
                    .into_iter()
                    .filter_map(|(_, ev)| {
                        let TokenEvent::Mint {
                            timestamp,
                            details:
                                MintEvent::Referral {
                                    referee_user_principal_id,
                                    ..
                                },
                            amount,
                        } = ev
                        else {
                            return None;
                        };
                        Some(HistoryDetails {
                            epoch_secs: timestamp.secs_since_epoch,
                            referee: referee_user_principal_id,
                            amount,
                        })
                    })
                    .collect();
                Ok(PageEntry {
                    data: details,
                    end: list_end,
                })
            }
        }
    }

    #[cfg(feature = "mock-referral-history")]
    mod mock {
        use std::convert::Infallible;

        use ic_agent::{identity::Secp256k1Identity, Identity};
        use k256::SecretKey;
        use rand_chacha::{
            rand_core::{RngCore, SeedableRng},
            ChaCha8Rng,
        };

        use crate::utils::current_epoch;

        use super::*;

        #[derive(Clone, Copy)]
        pub struct MockHistoryProvider;

        impl CursoredDataProvider for MockHistoryProvider {
            type Data = HistoryDetails;
            type Error = Infallible;

            async fn get_by_cursor(
                &self,
                from: usize,
                end: usize,
            ) -> Result<PageEntry<HistoryDetails>, Infallible> {
                let mut rand_gen = ChaCha8Rng::seed_from_u64(current_epoch().as_nanos() as u64);
                let data = (from..end)
                    .map(|_| {
                        let sk = SecretKey::random(&mut rand_gen);
                        let epoch_secs = rand_gen.next_u32() as u64;
                        let identity = Secp256k1Identity::from_private_key(sk);
                        let amount = rand_gen.next_u64() % 500;
                        HistoryDetails {
                            epoch_secs,
                            referee: identity.sender().unwrap(),
                            amount,
                        }
                    })
                    .collect();
                Ok(PageEntry { data, end: false })
            }
        }
    }
}
