use std::env;

use gloo_utils::format::JsValueSerdeExt;
use leptos::*;
use serde::Serialize;
use serde_json::json;
use wasm_bindgen::prelude::*;

use crate::consts::GTAG_MEASUREMENT_ID;

pub mod events;

#[derive(Debug, Serialize)]
struct GA4Event {
    client_id: String,
    user_id: Option<String>,
    events: Vec<Event>,
}

#[derive(Debug, Serialize)]
struct Event {
    name: String,
    params: serde_json::Value,
}

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

#[cfg(feature = "ga4")]
#[server]
pub async fn send_event_ssr(
    event_name: String,
    params: serde_json::Value,
) -> Result<(), ServerFnError> {
    use super::host::get_host;

    let host_str = get_host();
    let mut params = params.clone();
    params["host"] = json!(host_str);

    // Warehouse
    send_event_warehouse(&event_name, &params).await;

    // GA4
    // get client_id as user_id from params
    let user_id = params["user_id"].as_str().unwrap_or("0");
    let res = send_event_ga4_ssr(user_id, &event_name, &params).await;

    if let Err(e) = res {
        log::error!("Error sending event to GA4: {:?}", e);
    }

    Ok(())
}

#[cfg(feature = "ga4")]
pub fn send_event_ssr_spawn(event_name: String, params: serde_json::Value) {
    spawn_local(async move {
        let _ = send_event_ssr(event_name, params).await;
    });
}

#[cfg(feature = "ga4")]
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

#[cfg(feature = "ga4")]
pub async fn send_event_warehouse(event_name: &str, params: &serde_json::Value) {
    use super::host::get_host;

    let event_name = event_name.to_string();

    let mut params = params.clone();
    if params["host"].is_null() {
        let host_str = get_host();
        params["host"] = json!(host_str);
    }

    let params_str = params.to_string();

    let res = stream_to_offchain_agent(event_name, params_str).await;
    if let Err(e) = res {
        log::error!("Error sending event to warehouse: {:?}", e);
    }
}

#[cfg(feature = "ga4")]
#[server]
pub async fn send_event_warehouse_ssr(
    event_name: String,
    params: serde_json::Value,
) -> Result<(), ServerFnError> {
    send_event_warehouse(&event_name, &params).await;

    Ok(())
}

#[cfg(feature = "ga4")]
pub fn send_event_warehouse_ssr_spawn(event_name: String, params: serde_json::Value) {
    spawn_local(async move {
        let _ = send_event_warehouse_ssr(event_name, params).await;
    });
}

#[cfg(feature = "ga4")]
#[server]
pub async fn stream_to_offchain_agent(event: String, params: String) -> Result<(), ServerFnError> {
    use tonic::metadata::MetadataValue;
    use tonic::transport::Channel;
    use tonic::Request;

    let channel: Channel = expect_context();

    let mut off_chain_agent_grpc_auth_token = env::var("GRPC_AUTH_TOKEN").expect("GRPC_AUTH_TOKEN");
    // removing whitespaces and new lines for proper parsing
    off_chain_agent_grpc_auth_token.retain(|c| !c.is_whitespace());

    let token: MetadataValue<_> = format!("Bearer {}", off_chain_agent_grpc_auth_token).parse()?;

    let mut client =
        warehouse_events::warehouse_events_client::WarehouseEventsClient::with_interceptor(
            channel,
            move |mut req: Request<()>| {
                req.metadata_mut().insert("authorization", token.clone());
                Ok(req)
            },
        );

    let request = tonic::Request::new(warehouse_events::WarehouseEvent { event, params });

    client.send_event(request).await?;

    Ok(())
}

#[cfg(feature = "ga4")]
pub async fn send_event_ga4_ssr(
    user_id: &str,
    event_name: &str,
    params: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::Client;

    let measurement_id: &str = GTAG_MEASUREMENT_ID.as_ref();
    let api_secret = env::var("GA4_API_SECRET")?;

    let client = Client::new();
    let url = format!(
        "https://www.google-analytics.com/mp/collect?measurement_id={}&api_secret={}",
        measurement_id, api_secret
    );

    let payload = GA4Event {
        client_id: "12345".to_string(), // Should be some unique id
        user_id: Some(user_id.to_string()),
        events: vec![Event {
            name: event_name.to_string(),
            params: params.clone(),
        }],
    };

    let response = client.post(&url).json(&payload).send().await?;

    if !response.status().is_success() {
        return Err(format!("GA4 request failed: {:?}", response.status()).into());
    }

    Ok(())
}
