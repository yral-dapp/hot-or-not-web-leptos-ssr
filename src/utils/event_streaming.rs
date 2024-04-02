use gloo_utils::format::JsValueSerdeExt;
use leptos::{server, spawn_local, ServerFnError};
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = gtag)]
    pub fn gtag(cmd: &str, event_name: &str, params: &JsValue);
}

pub fn send_event(event_name: &str, params: &serde_json::Value) {
    // gtag GA4
    gtag("event", event_name, &JsValue::from_serde(params).unwrap());

    // Warehouse
    send_event_warehouse(event_name, params);
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

    // Load your service account key
    let sa_key =
        yup_oauth2::read_service_account_key("/Users/komalsai/Downloads/clientsecret.json") // TODO: change this to read from env var
            .await
            .expect("clientsecret.json");

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
