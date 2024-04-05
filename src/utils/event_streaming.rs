use std::env;

use gloo_utils::format::JsValueSerdeExt;
use leptos::*;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

use crate::consts::GTAG_MEASUREMENT_ID;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = gtag)]
    pub fn gtag(cmd: &str, event_name: &str, params: &JsValue);
}

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub enum EventType {
//     GA4Event,
//     WarehouseEvent,
//     All,
// }

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct EventRequest {
//     event_name: String,
//     params: Value,
//     event_type: EventType,
// }

// impl EventRequest {
//     pub fn new(event_name: String, params: Value, event_type: EventType) -> Self {
//         Self {
//             event_name,
//             params,
//             event_type,
//         }
//     }
// }

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

pub fn send_event_warehouse(event_name: &str, params: &serde_json::Value) {
    let data = serde_json::json!({
        "kind": "bigquery#tableDataInsertAllRequest",
        "rows": [
            {
                "json": {
                    "event": event_name.to_string(),
                    "params": params.to_string()
                }
            }
        ]
    });

    spawn_local(async move {
        stream_to_bigquery(data).await.unwrap();
    });
}

#[cfg(feature = "ssr")]
async fn get_access_token() -> String {
    use yup_oauth2::ServiceAccountAuthenticator;

    let sa_key_file = env::var("GOOGLE_SA_KEY_FILE").unwrap();

    // Load your service account key
    let sa_key = yup_oauth2::parse_service_account_key(sa_key_file).expect("clientsecret.json");

    let auth = ServiceAccountAuthenticator::builder(sa_key)
        .build()
        .await
        .unwrap();

    let scopes = &["https://www.googleapis.com/auth/bigquery.insertdata"];
    let token = auth.token(scopes).await.unwrap();

    match token.token() {
        Some(t) => t.to_string(),
        _ => panic!("No access token found"),
    }
}

#[cfg(feature = "ssr")]
async fn stream_to_bigquery_impl(data: Value) -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::Client;

    use crate::consts::BIGQUERY_INGESTION_URL;

    println!("Data: {:?}", data);

    let token = get_access_token().await;
    let client = Client::new();
    let request_url = BIGQUERY_INGESTION_URL.to_string();
    let response = client
        .post(request_url)
        .bearer_auth(token)
        .json(&data)
        .send()
        .await?;

    match response.status().is_success() {
        true => Ok(()),
        false => Err(format!("Failed to stream data - {:?}", response.text().await?).into()),
    }
}

#[server]
async fn stream_to_bigquery(data: Value) -> Result<(), ServerFnError> {
    match stream_to_bigquery_impl(data).await {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}
