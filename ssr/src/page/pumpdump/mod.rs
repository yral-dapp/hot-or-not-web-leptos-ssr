use codee::string::FromToStringCodec;
use leptos::{
    component, create_action, create_effect, create_rw_signal, expect_context, html::Div, logging,
    provide_context, view, For, IntoView, NodeRef, Show, SignalGet, SignalGetUntracked, SignalSet,
    SignalUpdate, SignalUpdateUntracked,
};
use leptos_router::use_query;
use leptos_use::{use_cookie, use_infinite_scroll_with_options, UseInfiniteScrollOptions};
use yral_canisters_common::{utils::token::RootType, Canisters};

use crate::{
    page::icpump::{process_token_list_item, ProcessedTokenListResponse},
    state::canisters::authenticated_canisters,
    utils::token::icpump::{get_paginated_token_list_with_limit, IcpumpTokenInfo, TokenListItem},
};

pub mod profile;
pub use profile::*;
pub mod withdrawal;
pub use withdrawal::*;
pub mod test;
pub use test::*;

pub(super) mod model;
use model::*;

mod context;
use context::*;

pub(super) mod components;
use components::header::Header;
use components::{card::GameCard, onboarding::OnboardingPopup};

async fn load_selected_card(
    cans: &Canisters<true>,
    card_query: CardQuery,
) -> Result<ProcessedTokenListResponse, String> {
    let CardQuery { root, .. } = card_query;

    let meta = cans
        .token_metadata_by_root_type(
            &IcpumpTokenInfo,
            Some(cans.user_principal()),
            RootType::Other(root),
        )
        .await
        .map_err(|err| format!("Couldn't load token's meta info: {err}"))?
        .ok_or("The token doesn't exist".to_string())?;

    // REFACTOR: create struct with only the information that's actually needed
    Ok(ProcessedTokenListResponse {
        token_details: TokenListItem {
            user_id: "notneeded".into(),
            name: meta.name.clone(),
            token_name: meta.name,
            token_symbol: meta.symbol,
            logo: meta.logo_b64,
            description: meta.description,
            created_at: "notneeded".into(),
            formatted_created_at: "notneeded".into(),
            link: "notneeded".into(),
            is_nsfw: meta.is_nsfw,
        },
        root,
        token_owner: meta.token_owner,
        is_airdrop_claimed: false, // not needed
    })
}

#[component]
pub fn PumpNDump() -> impl IntoView {
    let card_query = use_query::<CardQuery>().get().ok();
    let s: ShowSelectedCardSignal = create_rw_signal(ShowSelectedCard(
        card_query.as_ref().is_some_and(|q| q.is_valid()),
    ));
    provide_context(s);

    let show_selected_card = create_rw_signal(card_query);

    let player_games_count_and_balance = create_rw_signal(None::<PlayerData>);
    let cans_wire_res = authenticated_canisters();
    // i wonder if we remove this excessive cloning somehow
    let cans_wire_res_for_tokens = cans_wire_res.clone();
    let cans_wire_res_for_game_data = cans_wire_res.clone();
    let identity = create_rw_signal::<Option<Canisters<true>>>(None);
    provide_context(identity);
    let fetch_identity = create_action(move |&()| {
        let cans_wire_res = cans_wire_res.clone();
        async move {
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|e| e.to_string())?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|_| "Unable to authenticate".to_string())?;

            identity.update(|i| *i = Some(cans));

            Ok::<_, String>(())
        }
    });
    create_effect(move |_| {
        if identity.get_untracked().is_none() {
            fetch_identity.dispatch(());
        }
    });

    let fetch_user_principal = create_action(move |&()| {
        let cans_wire_res = cans_wire_res_for_game_data.clone();
        async move {
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|e| e.to_string())?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|_| "Unable to authenticate".to_string())?;

            let data = PlayerData::load(cans.user_canister())
                .await
                .map_err(|_| "Couldn't load player data".to_string())?;

            player_games_count_and_balance.set(Some(data));

            Ok::<(), String>(())
        }
    });

    create_effect(move |_| {
        if player_games_count_and_balance.get_untracked().is_none() {
            fetch_user_principal.dispatch(());
        }
    });

    provide_context(player_games_count_and_balance);

    let (should_show, set_should_show) = use_cookie::<bool, FromToStringCodec>("show_onboarding");
    let show_onboarding = ShowOnboarding(should_show, set_should_show);
    provide_context(show_onboarding);

    let tokens = create_rw_signal(Vec::<ProcessedTokenListResponse>::new());
    let page = create_rw_signal(0u32);
    let should_load_more = create_rw_signal(true);
    let fetch_more_tokens = create_action(move |&page: &u32| {
        let cans_wire_res = cans_wire_res_for_tokens.clone();
        async move {
            // since we are starting a load job, no more load jobs should be start
            should_load_more.set(false);
            let cans_wire = cans_wire_res
                .wait_untracked()
                .await
                .map_err(|_| "Couldn't get cans_wire")?;
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .map_err(|_| "Unable to authenticate".to_string())?;

            let selected_card = show_selected_card.get().take();
            show_selected_card.update(|item| *item = None);

            let selected_card = match selected_card {
                Some(q) => load_selected_card(&cans, q)
                    .await
                    .inspect_err(|err| {
                        logging::error!("Couldn't load selected card: {err}");
                    })
                    .ok(),
                None => Default::default(),
            };

            let limit = 5;

            let more_tokens = get_paginated_token_list_with_limit(page, limit)
                .await
                .expect("TODO: handle error");
            let had_tokens = !more_tokens.is_empty();

            logging::log!("more {more_tokens:?}");

            let mut processed_token = match selected_card {
                Some(t) => vec![t],
                None => Default::default(),
            };

            #[cfg(any(feature = "local-bin", feature = "local-lib"))]
            processed_token.extend_from_slice(
                &process_token_list_item(more_tokens, cans.user_canister()).await,
            );

            #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
            processed_token.extend_from_slice(
                &process_token_list_item(more_tokens, cans.user_principal()).await,
            );

            logging::log!("processed {processed_token:?}");
            // ignore tokens with no owners
            processed_token.retain(|item| item.token_owner.is_some());

            tokens.update(|tokens| {
                tokens.extend_from_slice(&processed_token);
            });

            if had_tokens {
                // since there were tokens loaded
                // assume we have more tokens to load
                // so, allow token loading
                should_load_more.set(true)
            }

            Ok::<_, String>(())
        }
    });
    let scroll_container = NodeRef::<Div>::new();
    let _ = use_infinite_scroll_with_options(
        scroll_container,
        move |_| async move {
            if !should_load_more.get() {
                return;
            }
            page.update_untracked(|v| {
                *v += 1;
            });

            fetch_more_tokens.dispatch(page.get_untracked());
        },
        // start loading early, throttled at 3s per load
        UseInfiniteScrollOptions::default()
            .distance(400f64)
            .interval(2000f64),
    );
    view! {
        <div class="h-screen w-screen block text-white bg-black">
            <div class="max-w-md flex flex-col relative w-full mx-auto items-center h-full px-4 py-4">
                <Header />
                <div node_ref=scroll_container class="size-full overflow-scroll flex flex-col gap-4 snap-mandatory snap-y pb-[50vh]">
                    <For each=move || tokens.get() key=|item| item.token_details.token_name.clone() let:token>
                        <GameCard token />
                    </For>
                </div>
            </div>
            <Show when=move || show_onboarding.should_show()>
                <OnboardingPopup />
            </Show>
        </div>
    }
}
