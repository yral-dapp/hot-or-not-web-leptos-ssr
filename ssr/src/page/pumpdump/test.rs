use std::rc::Rc;

use candid::Principal;
use codee::string::JsonSerdeCodec;
use leptos::{html::Input, *};
use leptos_router::use_params_map;
use leptos_use::{use_websocket, UseWebSocketReturn};
use yral_canisters_common::{
    utils::token::{RootType, TokenOwner},
    Canisters,
};
use yral_pump_n_dump_common::{
    ws::{websocket_connection_url, WsMessage, WsRequest},
    GameDirection,
};

use crate::{
    consts::PUMP_AND_DUMP_WORKER_URL, page::pumpdump::WsResponse,
    state::canisters::authenticated_canisters, utils::token::icpump::IcpumpTokenInfo,
};

use super::{GameRunningData, PlayerData};

#[derive(Debug, Clone)]
struct TestData {
    owner: TokenOwner,
    root: Principal,
    user_canister: Principal,
    player_data: PlayerData,
    running_data: GameRunningData,
}

type TestDataSignal = RwSignal<Option<TestData>>;

// based on https://leptos-use.rs/network/use_websocket.html#usage-with-provide_context
#[derive(Clone)]
struct WsCtx {
    message: Signal<Option<WsResponse>>,
    sendfn: Rc<dyn Fn(&WsRequest)>,
}

impl WsCtx {
    #[inline]
    fn send(&self, msg: &WsRequest) {
        (self.sendfn)(msg)
    }
}

#[component]
fn WebsocketLogs(websocket: WsCtx, round: u64) -> impl IntoView {
    let received = create_rw_signal(Vec::new());
    let sent = create_rw_signal(Vec::new());
    let message = websocket.message;
    let pump_socket = websocket.clone();
    let send_pump = move || {
        let message = WsRequest {
            request_id: uuid::Uuid::new_v4(),
            msg: WsMessage::Bet {
                direction: GameDirection::Pump,
                round,
            },
        };
        pump_socket.send(&message);
        sent.update(move |d| d.push(serde_json::to_string_pretty(&message).unwrap()));
    };
    let send_dump = move || {
        let message = WsRequest {
            request_id: uuid::Uuid::new_v4(),
            msg: WsMessage::Bet {
                direction: GameDirection::Dump,
                round,
            },
        };
        websocket.send(&message);
        sent.update(move |d| d.push(serde_json::to_string_pretty(&message).unwrap()));
    };
    create_effect(move |_| {
        if let Some(message) = message.get() {
            received.update(move |d| d.push(serde_json::to_string_pretty(&message).unwrap()));
        }
    });

    view! {
        <div>
            <div>
                <button class="p-3 border" on:click=move |_| send_pump()>PUMP</button>
                <button class="p-3 border" on:click=move |_| send_dump()>DUMP</button>
            </div>
            <div class="flex">
                <div class="flex-1">
                    <h1>Sent</h1>
                    <For each=move || sent.get().into_iter().rev() key=|item| item.clone() let:item>
                        <pre>
                            {item}
                        </pre>
                    </For>
                </div>
                <div class="flex-1">
                    <h1>Received</h1>
                    <For each=move || received.get().into_iter().rev() key=|item| item.clone() let:item>
                        <pre>
                            {item}
                        </pre>
                    </For>
                </div>
            </div>
        </div>
    }
}

#[component]
fn PresentDetails(#[prop(into)] data: TestData) -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 gap-4">
            <fieldset class="border">
                <legend>Player Data</legend>
                <pre class="whitespace-pre-line">
                    game count: {move || data.player_data.games_count}
                    balance: {move || data.player_data.wallet_balance}
                </pre>
            </fieldset>
            <fieldset class="border">
                <legend>Game Running Data</legend>
                <pre class="whitespace-pre-line">
                    pumps: {move || data.running_data.pumps}
                    dumps: {move || data.running_data.dumps}
                    player count {move || data.running_data.player_count}
                </pre>
            </fieldset>
            <fieldset class="border">
                <legend>Token Detail</legend>
                <pre class="whitespace-pre-line">
                    token root: {move || data.root.to_string()}
                    token owner (cansister): {move || data.owner.canister_id.to_string()}
                    user (canister): {move || data.user_canister.to_string()}
                </pre>
            </fieldset>
        </div>
    }
}

#[component]
pub fn PndTest() -> impl IntoView {
    let params = use_params_map().get();
    let token_root = params
        .get("token_root")
        .expect("token_root to be in path params");
    let token_root = Principal::from_text(token_root).expect("token root to be valid");

    let data: TestDataSignal = create_rw_signal(None);
    let round = create_rw_signal(0u64);

    let cans_wire = authenticated_canisters();
    let fetch_test_data = create_action(move |&()| {
        let cans_wire = cans_wire.clone();
        async move {
            let cans_wire = cans_wire
                .wait_untracked()
                .await
                .expect("cans_wire to be there");
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .expect("get auth canisters from the wire");

            let meta = cans
                .token_metadata_by_root_type(
                    &IcpumpTokenInfo,
                    Some(cans.user_canister()),
                    RootType::Other(token_root),
                )
                .await
                .inspect_err(|err| log::error!("metadata request failed{err}"))
                .expect("couldn't get the token metadata")
                .expect("token root to exist");

            data.set(Some(TestData {
                owner: meta.token_owner.clone().unwrap(),
                root: token_root,
                user_canister: cans.user_canister(),
                player_data: PlayerData::load(cans.user_canister()).await.unwrap(),
                running_data: GameRunningData::load(
                    meta.token_owner.unwrap().canister_id,
                    token_root,
                    cans.user_canister(),
                )
                .await
                .unwrap(),
            }));
        }
    });

    let websocket = create_rw_signal(None);
    let cans_wire = authenticated_canisters();
    let create_websocket_connection = create_action(move |&()| {
        let cans_wire = cans_wire.clone();
        async move {
            let mut ws_url = PUMP_AND_DUMP_WORKER_URL.clone();
            ws_url.set_scheme("ws").expect("schema to valid");
            let cans_wire = cans_wire
                .wait_untracked()
                .await
                .expect("cans_wire to be there");
            let cans = Canisters::from_wire(cans_wire.clone(), expect_context())
                .expect("get auth canisters from the wire");

            let meta = cans
                .token_metadata_by_root_type(
                    &IcpumpTokenInfo,
                    Some(cans.user_canister()),
                    RootType::Other(token_root),
                )
                .await
                .inspect_err(|err| log::error!("metadata request failed: {err}"))
                .expect("couldn't get the token metadata")
                .expect("token root to exist");

            let websocket_url = websocket_connection_url(
                ws_url,
                cans.identity(),
                meta.token_owner.unwrap().canister_id,
                token_root,
            )
            .expect("websocket connection to go through");

            let UseWebSocketReturn {
                message,
                send: sendfn,
                ..
            } = use_websocket::<WsRequest, WsResponse, JsonSerdeCodec>(websocket_url.as_str());

            // erase type, because sendfn is not send/sync
            let context = WsCtx {
                message,
                sendfn: Rc::new(sendfn),
            };

            websocket.set(Some(context));
        }
    });
    create_effect(move |_| {
        if websocket.get().is_none() {
            create_websocket_connection.dispatch(());
        }
    });

    create_effect(move |_| {
        if data.get().is_none() {
            fetch_test_data.dispatch(());
        }
    });

    let input_ref = create_node_ref::<Input>();
    let change_round = move |_: leptos::ev::Event| {
        if let Some(input) = input_ref.get() {
            let value = input.value().parse().unwrap_or_default();
            round.set(value);
        }
    };

    view! {
        <Show when=move || data.get().is_some()>
            <PresentDetails data=data.get().unwrap() />
        </Show>
        <input ref=input_ref type="number" on:input=change_round placeholder="round" />
        <Show when=move || websocket.get().is_some()>
            <WebsocketLogs round=round.get() websocket=websocket.get().unwrap() />
        </Show>
    }
}
