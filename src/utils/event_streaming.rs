use std::env;

use gloo_utils::format::JsValueSerdeExt;
use leptos::*;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

use crate::consts::{GTAG_MEASUREMENT_ID, OFF_CHAIN_AGENT_GRPC_URL};

#[cfg(feature = "ssr")]
pub mod warehouse_events {
    tonic::include_proto!("warehouse_events");
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = gtag)]
    pub fn gtag(cmd: &str, event_name: &str, params: &JsValue);
}

#[derive(Clone, Default)]
pub struct EventHistory {
    pub event_name: RwSignal<String>,
}

pub fn send_event(event_name: &str, params: &serde_json::Value) {
    let event_history: EventHistory = expect_context();

    event_history.event_name.set(event_name.to_string());

    // Warehouse
    send_event_warehouse(event_name, params);

    // gtag GA4
    gtag("event", event_name, &JsValue::from_serde(params).unwrap());
}

pub fn send_user_id(user_id: String) {
    let gtag_measurement_id = GTAG_MEASUREMENT_ID.as_ref();

    gtag(
        "config",
        gtag_measurement_id,
        &JsValue::from_serde(&json!({
            "user_id": user_id,
        }))
            .unwrap(),
    );
}

pub fn send_event_warehouse(_event_name: &str, _params: &serde_json::Value) {
    // let event_name = event_name.to_string();
    // let params_str = params.to_string();
    //
    // spawn_local(async move {
    //     stream_to_offchain_agent(event_name, params_str).await.unwrap();
    // });
}

#[server]
pub async fn stream_to_offchain_agent(event_name: String, params: String) -> Result<(), ServerFnError> {
    use tonic::metadata::MetadataValue;
    use tonic::Request;
    use tonic::transport::Channel;

    let off_chain_agent_url = OFF_CHAIN_AGENT_GRPC_URL.as_ref();
    let channel = Channel::from_static(off_chain_agent_url)
        .connect()
        .await?;

    let off_chain_agent_grpc_auth_token = env::var("GRPC_AUTH_TOKEN").expect("GRPC_AUTH_TOKEN");

    let token: MetadataValue<_> = format!("Bearer {}", off_chain_agent_grpc_auth_token).parse()?;

    let mut client =
        warehouse_events::warehouse_events_client::WarehouseEventsClient::with_interceptor(
            channel,
            move |mut req: Request<()>| {
                req.metadata_mut().insert("authorization", token.clone());
                Ok(req)
            },
        );

    let request = tonic::Request::new(warehouse_events::WarehouseEvent {
        event: event_name,
        params,
    });

    let response = client.send_event(request).await?;
    println!("RESPONSE={:?}", response);

    Ok(())
}
