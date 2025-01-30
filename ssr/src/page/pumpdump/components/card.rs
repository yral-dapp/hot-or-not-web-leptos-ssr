pub(super) mod result;
pub use result::*;
use std::rc::Rc;

use codee::string::JsonSerdeCodec;
use leptos::{
    component, create_action, create_effect, create_rw_signal, expect_context, logging,
    provide_context, view, IntoView, Show, SignalGet, SignalGetUntracked, SignalSetUntracked,
    SignalUpdate, Suspense,
};
use leptos_router::use_query;
use leptos_use::{use_websocket, UseWebSocketReturn};
use yral_pump_n_dump_common::{
    ws::{websocket_connection_url, GameResult as RawGameResult, WsError, WsRequest, WsResp},
    GameDirection,
};

use crate::{
    consts::PUMP_AND_DUMP_WORKER_URL,
    page::{
        icpump::ProcessedTokenListResponse,
        pumpdump::{
            convert_e8s_to_gdolr, CardQuery, CurrentRound, GameResult, GameRunningData, GameState,
            IdentitySignal, LoadRunningDataAction, PlayerDataSignal, ShowSelectedCard,
            ShowSelectedCardSignal, WebsocketContext, WsResponse,
        },
    },
};

fn compute_game_result(running_data: GameRunningData, raw_result: RawGameResult) -> GameResult {
    if running_data.pumps == running_data.dumps {
        return GameResult::Win { amount: 0 };
    }

    let (user_direction, user_bet_count) = if running_data.pumps > running_data.dumps {
        (GameDirection::Pump, running_data.pumps)
    } else {
        (GameDirection::Dump, running_data.dumps)
    };

    // TODO: impl eq on GameDirection in yral-common
    let m = |direction: GameDirection| match direction {
        GameDirection::Pump => 0,
        GameDirection::Dump => 1,
    };
    if m(user_direction) != m(raw_result.direction) {
        GameResult::Loss {
            amount: running_data.pumps as u128 + running_data.dumps as u128,
        }
    } else {
        let amount = (user_bet_count * raw_result.reward_pool) / raw_result.bet_count;
        let amount = convert_e8s_to_gdolr(amount);
        GameResult::Win { amount }
    }
}

#[component]
pub fn GameCard(#[prop()] token: ProcessedTokenListResponse) -> impl IntoView {
    let show_selected_card: ShowSelectedCardSignal = expect_context();
    let card_query = use_query::<CardQuery>().get().ok();
    let owner_canister_id = token.token_owner.as_ref().unwrap().canister_id;
    let token_root = token.root;

    let websocket = create_rw_signal(None::<WebsocketContext>);
    provide_context(websocket);
    let current_round = create_rw_signal(None::<CurrentRound>);
    provide_context(current_round);
    let running_data = create_rw_signal(None::<GameRunningData>);
    provide_context(running_data);
    let game_state = create_rw_signal(None::<GameState>);
    provide_context(game_state);
    provide_context(token);

    let player_data: PlayerDataSignal = expect_context();

    let identity: IdentitySignal = expect_context();
    let load_running_data: LoadRunningDataAction =
        create_action(move |&(user_canister, reload_game_state)| async move {
            let data = GameRunningData::load(owner_canister_id, token_root, user_canister)
                .await
                .inspect_err(|err| {
                    logging::error!("couldn't load running data: {err}");
                })
                .ok();

            if data.is_some() && reload_game_state {
                game_state.update(|s| *s = Some(GameState::Playing));
            }

            running_data.update(|d| *d = data);
        });
    provide_context(load_running_data);
    // start websocket connection
    create_effect(move |_| {
        let ident = identity.get();
        if let Some(value) = &ident {
            let mut ws_url = PUMP_AND_DUMP_WORKER_URL.clone();
            #[cfg(not(any(feature = "local-bin", feature = "local-lib")))]
            ws_url.set_scheme("wss").expect("schema to valid");

            #[cfg(any(feature = "local-bin", feature = "local-lib"))]
            ws_url.set_scheme("ws").expect("schema to valid");

            let websocket_url =
                websocket_connection_url(ws_url, value.identity(), owner_canister_id, token_root)
                    .map_err(|err| format!("Coulnd't create ws connection url: {err}"))?;

            let UseWebSocketReturn {
                message,
                send: sendfn,
                ..
            } = use_websocket::<WsRequest, WsResponse, JsonSerdeCodec>(websocket_url.as_str());

            // erase type, because sendfn is not send/sync
            let context = WebsocketContext::new(
                message,
                Rc::new(move |message| {
                    sendfn(message);
                }),
            );

            websocket.update(|ws| *ws = Some(context));
        }
        Ok::<(), String>(())
    });

    create_effect(move |_| {
        if let Some(websocket) = websocket.get() {
            if let Some(message) = websocket.message.get() {
                match message.response {
                    WsResp::WelcomeEvent {
                        round,
                        pool,
                        player_count,
                        user_bets,
                    } => {
                        current_round.set_untracked(Some(CurrentRound(round)));
                        running_data.update(move |data| {
                            *data = Some(GameRunningData::new(
                                user_bets.pumps,
                                user_bets.dumps,
                                player_count,
                                Some(pool),
                            ))
                        });
                    }
                    WsResp::BetSuccesful { round } => {
                        logging::log!(
                            "ws: bet successful for round: {round}. (note: current round: {:?})",
                            current_round.get_untracked()
                        );
                    }
                    WsResp::Error(err) => {
                        // TODO: handle this error
                        logging::error!("ws: received error: {err:?}");

                        if let WsError::BetFailure { direction, .. } = err {
                            running_data.update(move |data| {
                                if let Some(data) = data {
                                    match direction {
                                        GameDirection::Pump => data.pumps -= 1,
                                        GameDirection::Dump => data.dumps -= 1,
                                    }
                                }
                            });
                        }
                    }
                    WsResp::GameResultEvent(result) => {
                        logging::log!("ws: received result: winning direction = {}, bet_count = {}, reward_pool = {}", match result.direction {
                             GameDirection::Pump => "pump",
                             GameDirection::Dump => "dump",
                        }, result.bet_count, result.reward_pool);
                        let running_data = running_data
                            .get_untracked()
                            .expect("running data to exist if we have recieved results");
                        current_round.set_untracked(Some(CurrentRound(result.new_round)));
                        let result = compute_game_result(running_data, result);

                        game_state.update(|s| *s = Some(GameState::ResultDeclared(result)));
                        logging::log!(
                            "after game state update: {:?}",
                            current_round.get_untracked()
                        );
                        match result {
                            GameResult::Win { amount } => {
                                player_data.update(|data| {
                                    if let Some(data) = data {
                                        data.games_count += 1;
                                        data.wallet_balance += amount;
                                    }
                                });
                            }
                            GameResult::Loss { .. } => {
                                player_data.update(|data| {
                                    if let Some(data) = data {
                                        data.games_count += 1;
                                    }
                                });
                            }
                        }
                    }
                    WsResp::WinningPoolEvent { new_pool, round } => {
                        logging::log!("ws: received new winning pot: {new_pool} for round: {round}. (note, current round: {:?})", current_round.get_untracked());
                        running_data.update(|data| {
                            if let Some(data) = data {
                                data.winning_pot = Some(new_pool);
                            }
                        })
                    }
                }
            }
        }
    });

    let load_game_state = create_action(move |&()| {
        let card_query = card_query.clone();
        async move {
            logging::log!("{:?}", show_selected_card.get());
            let state = match (
                show_selected_card.get(),
                card_query.and_then(|c| c.details()),
            ) {
                (ShowSelectedCard(true), Some((root, result))) if root == token_root => {
                    show_selected_card.set_untracked(ShowSelectedCard(false));
                    Some(GameState::ResultDeclared(result))
                }
                _ => GameState::load(owner_canister_id, token_root)
                    .await
                    .inspect_err(|err| {
                        logging::error!("couldn't load game state: {err}");
                    })
                    .ok(),
            };

            game_state.update(|s| *s = state);

            Ok::<(), String>(())
        }
    });

    create_effect(move |_| {
        // might dispatch multiple times, need a way to ensure game state is
        // loaded only once
        if game_state.get().is_none() {
            logging::log!("loading game state");
            load_game_state.dispatch(());
        }
    });

    view! {
        <Suspense>
            {move || game_state.get().map(move |game_state| view! {
                <div
                    style="perspective: 500px; transition: transform 0.4s; transform-style: preserve-3d;"
                    class="relative w-full min-h-[31rem] snap-start snap-always"
                >
                    <Show
                        when=move || { matches!(game_state, GameState::Playing)}
                        fallback=move || view! { <ResultDeclaredCard game_state /> }
                    >
                        <PlayingCard />
                    </Show>
                </div>
            })}
        </Suspense>
    }
}
