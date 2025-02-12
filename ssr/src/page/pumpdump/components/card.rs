pub(super) mod result;
pub(super) mod skeleton;
use ic_agent::identity::DelegatedIdentity;
pub use result::*;
pub use skeleton::*;
use std::rc::Rc;

use codee::string::JsonSerdeCodec;
use leptos::*;
use leptos_router::use_query;
use leptos_use::{use_websocket, UseWebSocketReturn};
use yral_pump_n_dump_common::ws::{websocket_connection_url, WsRequest};

use crate::{
    consts::PUMP_AND_DUMP_WORKER_URL,
    page::{
        icpump::ProcessedTokenListResponse,
        pumpdump::{
            CardQuery, GameState, PlayerDataRes, RunningGameCtx, RunningGameRes, ShowSelectedCard,
            ShowSelectedCardSignal, WsResponse,
        },
    },
    state::canisters::authenticated_canisters,
};

#[component]
pub fn GameCard(token: ProcessedTokenListResponse) -> impl IntoView {
    let show_selected_card: ShowSelectedCardSignal = expect_context();
    let card_query = use_query::<CardQuery>().get().ok();

    provide_context(token.clone());

    let game_state = create_rw_signal(None);
    create_effect(move |_| {
        match (
            show_selected_card.get(),
            card_query.as_ref().and_then(|c| c.details()),
        ) {
            (ShowSelectedCard(true), Some((root, result))) if root == token.root => {
                show_selected_card.set_untracked(ShowSelectedCard(false));
                game_state.set(Some(GameState::ResultDeclared(result)))
            }
            _ => game_state.set(Some(GameState::Playing)),
        }
    });

    let auth_cans = authenticated_canisters();
    let player_data: PlayerDataRes = expect_context();
    let running_game_res = auth_cans.derive_local(
        || (),
        move |cans, _| {
            let player_data = player_data.clone();
            let token = token.clone();
            async move {
                let mut ws_url = PUMP_AND_DUMP_WORKER_URL.clone();
                let scheme = {
                    #[cfg(not(feature = "local-lib"))]
                    {
                        "wss"
                    }

                    #[cfg(feature = "local-lib")]
                    {
                        "ws"
                    }
                };
                ws_url.set_scheme(scheme).expect("scheme to be valid");

                let cans = cans?;
                let id: DelegatedIdentity = cans.id.try_into()?;
                let websocket_url = websocket_connection_url(
                    ws_url,
                    &id,
                    token.token_owner.as_ref().unwrap().canister_id,
                    token.root,
                )
                .map_err(ServerFnError::new)?;

                let UseWebSocketReturn {
                    message,
                    send: sendfn,
                    ..
                } = use_websocket::<WsRequest, WsResponse, JsonSerdeCodec>(websocket_url.as_str());

                let ctx = RunningGameCtx::derive(
                    cans.user_canister,
                    player_data,
                    game_state,
                    message,
                    Rc::new(sendfn),
                    token,
                );

                Ok::<_, ServerFnError>(ctx)
            }
        },
    );
    provide_context::<RunningGameRes>(running_game_res);

    view! {
        <Suspense> // <-- This suspense somehow prevents infinite scroller from resetting scroll :|
            {move || game_state.get().map(move |game_state| view! {
                <div
                    style="perspective: 500px; transition: transform 0.4s; transform-style: preserve-3d;"
                    class="relative w-full min-h-[31rem] snap-start snap-always"
                >
                    {match game_state {
                        GameState::Playing => view! {
                            <PlayingCard/>
                        },
                        GameState::ResultDeclared(result) => view! {
                            <ResultDeclaredCard result/>
                        }
                    }}
                </div>
            })}
        </Suspense>
    }
}
