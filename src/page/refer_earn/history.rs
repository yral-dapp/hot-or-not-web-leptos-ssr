use leptos::*;

use crate::component::bullet_loader::BulletLoader;
use crate::{
    state::canisters::{authenticated_canisters, Canisters},
    try_or_redirect_opt,
    utils::{profile::propic_from_principal, timestamp::get_day_month},
};
use history_provider::*;

#[component]
fn HistoryItem(detail: HistoryDetails) -> impl IntoView {
    view! {
        <div class="px-2 grid grid-cols-4 grid-rows-1 items-center gap-2 w-full">
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
                {detail.amount} Coins
            </span>
        </div>
    }
}

#[component]
fn AuthenticatedHistory(canisters: Canisters<true>) -> impl IntoView {
    let data = create_rw_signal(Vec::<HistoryDetails>::new());
    let cursor = create_rw_signal(0);
    let loading = create_rw_signal(true);
    let end = create_rw_signal(false);

    let fetch_more_action = create_action(move |&()| {
        let canisters = canisters.clone();
        let history_prov = get_history_provider(canisters);
        loading.set(true);
        async move {
            let cursor_v = cursor.get_untracked();
            let history = match get_history(&history_prov, cursor_v).await {
                Ok(h) => h,
                Err(e) => {
                    loading.set(false);
                    end.set(true);
                    log::warn!("failed to fetch more history, err: {e}");
                    return;
                }
            };
            batch(|| {
                data.update(|d| d.extend(history.details));
                cursor.set(history.cursor);
                loading.set(false);
                end.set(history.list_end);
            });
        }
    });
    fetch_more_action.dispatch(());

    view! {
        <div class="flex flex-col justify-center w-full md:w-10/12 lg:w-8/12 gap-4 pb-16">
            <For each=data key=|d| d.referee let:detail>
                <HistoryItem detail/>
            </For>
            <Show when=move || !end() && !loading()>
                <button
                    on:click=move |_| fetch_more_action.dispatch(())
                    class="text-white text-xl underline"
                >
                    Show more..
                </button>
            </Show>
            <Show when=loading>
                <BulletLoader/>
            </Show>
        </div>
    }
}

#[component]
pub fn HistoryView() -> impl IntoView {
    let canisters = authenticated_canisters();

    view! {
        <Suspense fallback=BulletLoader>
            {move || {
                canisters()
                    .and_then(|canisters| {
                        let canisters = try_or_redirect_opt!(canisters)?;
                        Some(view! { <AuthenticatedHistory canisters=canisters/> })
                    })
                    .unwrap_or_else(|| view! { <BulletLoader/> })
            }}

        </Suspense>
    }
}

mod history_provider {
    use candid::Principal;
    use ic_agent::AgentError;

    use crate::state::canisters::Canisters;

    #[derive(Clone, Copy)]
    pub struct HistoryDetails {
        pub epoch_secs: u64,
        pub referee: Principal,
        pub amount: u64,
    }

    #[derive(Default)]
    pub struct HistoryRes {
        pub details: Vec<HistoryDetails>,
        pub cursor: u64,
        pub list_end: bool,
    }

    pub trait HistoryProvider {
        async fn get_history(
            &self,
            from: u64,
            end: u64,
        ) -> Result<(Vec<HistoryDetails>, bool), AgentError>;
    }

    pub async fn get_history(
        prov: &impl HistoryProvider,
        from: u64,
    ) -> Result<HistoryRes, AgentError> {
        let (details, list_end) = prov.get_history(from, from + 10).await?;
        Ok(HistoryRes {
            details,
            cursor: from + 10,
            list_end,
        })
    }

    pub fn get_history_provider(canisters: Canisters<true>) -> impl HistoryProvider {
        #[cfg(feature = "mock-referral-history")]
        {
            _ = canisters;
            mock::MockHistoryProvider
        }
        #[cfg(not(feature = "mock-referral-history"))]
        {
            canisters
        }
    }

    #[cfg(not(feature = "mock-referral-history"))]
    impl HistoryProvider for Canisters<true> {
        async fn get_history(
            &self,
            from: u64,
            end: u64,
        ) -> Result<(Vec<HistoryDetails>, bool), AgentError> {
            use crate::canister::individual_user_template::{MintEvent, Result5, TokenEvent};
            use crate::utils::route::failure_redirect;
            let individual = self.authenticated_user();
            let history = individual
                .get_user_utility_token_transaction_history_with_pagination(from, end)
                .await?;
            let history = match history {
                Result5::Ok(history) => history,
                Result5::Err(_) => {
                    failure_redirect("failed to get posts");
                    return Ok((vec![], true));
                }
            };
            let list_end = history.len() < (end - from) as usize;
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
            Ok((details, list_end))
        }
    }

    #[cfg(feature = "mock-referral-history")]
    mod mock {
        use ic_agent::{identity::Secp256k1Identity, Identity};
        use k256::SecretKey;
        use rand_chacha::{
            rand_core::{RngCore, SeedableRng},
            ChaCha8Rng,
        };

        use crate::utils::current_epoch;

        use super::*;

        pub struct MockHistoryProvider;

        impl HistoryProvider for MockHistoryProvider {
            async fn get_history(
                &self,
                from: u64,
                end: u64,
            ) -> Result<(Vec<HistoryDetails>, bool), AgentError> {
                let mut rand_gen = ChaCha8Rng::seed_from_u64(current_epoch().as_nanos() as u64);
                Ok((
                    (from..end)
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
                        .collect(),
                    false,
                ))
            }
        }
    }
}
