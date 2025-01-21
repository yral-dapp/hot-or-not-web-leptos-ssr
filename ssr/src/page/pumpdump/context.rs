use std::rc::Rc;

use candid::Principal;
use leptos::{Action, RwSignal, Signal, SignalGet, SignalSet, WriteSignal};
use serde::{Deserialize, Serialize};
use yral_canisters_common::Canisters;
use yral_pump_n_dump_common::ws::{WsRequest, WsResp};

use super::model::{GameRunningData, PlayerData};

#[derive(Copy, Clone, Debug)]
pub(super) struct ShowOnboarding(
    pub(super) Signal<Option<bool>>,
    pub(super) WriteSignal<Option<bool>>,
);

impl ShowOnboarding {
    #[inline]
    pub(super) fn show(&self) {
        self.1.set(Some(true));
    }

    #[inline]
    pub(super) fn hide(&self) {
        self.1.set(Some(false));
    }

    #[inline]
    pub(super) fn should_show(&self) -> bool {
        self.0.get().unwrap_or(true)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ShowSelectedCard(pub(super) bool);

pub(super) type ShowSelectedCardSignal = RwSignal<ShowSelectedCard>;
pub(super) type GameRunningDataSignal = RwSignal<Option<GameRunningData>>;
pub(super) type PlayerDataSignal = RwSignal<Option<PlayerData>>;
pub(super) type LoadRunningDataAction = Action<(Principal, bool), ()>;
pub(super) type WebsocketContextSignal = RwSignal<Option<WebsocketContext>>;
pub(super) type IdentitySignal = RwSignal<Option<Canisters<true>>>;

pub(super) type Sendfn = Rc<dyn Fn(&WsRequest)>;

// TODO: use the WsResponse from pnd common crate
#[derive(Serialize, Deserialize, Clone)]
pub(super) struct WsResponse {
    pub(super) request_id: uuid::Uuid,
    pub(super) response: WsResp,
}

// based on https://leptos-use.rs/network/use_websocket.html#usage-with-provide_context
#[derive(Clone)]
pub(super) struct WebsocketContext {
    pub message: Signal<Option<WsResponse>>,
    sendfn: Sendfn, // use Arc to make it easily cloneable
}

impl WebsocketContext {
    pub(super) fn new(message: Signal<Option<WsResponse>>, send: Sendfn) -> Self {
        Self {
            message,
            sendfn: send,
        }
    }

    // create a method to avoid having to use parantheses around the field
    #[inline(always)]
    pub(super) fn send(&self, message: &WsRequest) {
        (self.sendfn)(message);
    }
}
