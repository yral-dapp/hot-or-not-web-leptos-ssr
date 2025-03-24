use std::env;

use leptos::prelude::*;
use leptos::server;

#[cfg(feature = "ga4")]
#[server]
pub async fn register_device(
    registration_token: String,
    device_fingerprint: String,
) -> Result<(), ServerFnError> {
    use std::collections::HashMap;

    use ic_agent::Identity;
    use reqwest::Client;
    use yral_canisters_common::Canisters;
    use yral_metadata_client::{DeviceRegistrationToken, MetadataClient, NotificationKey};

    let canisters = expect_context::<Canisters<true>>();
    let metadata_client: MetadataClient<true> =
        MetadataClient::with_base_url(consts::METADATA_API_BASE.clone());

    let client = Client::new();
    let url = "https://fcm.googleapis.com/fcm/notification";

    let user_identity = canisters.identity();
    let user_principal = user_identity.sender().unwrap();
    let principal_id = user_principal.to_text();
    let notification_key_name = format!("notification_key_{}", principal_id);
    let mut user_metadata = metadata_client
        .get_user_metadata(user_principal)
        .await?
        .ok_or(ServerFnError::new("metadata for principal not found"))?;

    let data = if let Some(notification_key) = user_metadata.notification_key.as_ref() {
        format!(
            r#"{{
                "operation": "add",
                "notification_key_name": "{}",
                "notification_key": "{}",
                "registration_ids": ["{}"]
            }}"#,
            notification_key_name, notification_key.key, registration_token
        )
    } else {
        format!(
            r#"{{
            "operation": "create",
            "notification_key_name": "{}",
            "registration_ids": ["{}"]
        }}"#,
            notification_key_name, registration_token
        )
    };

    // TODO: get the token from the app state
    let firebase_token = "temp_token";
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", firebase_token))
        .header("Content-Type", "application/json")
        .header("project_id", "hot-or-not-feed-intelligence")
        .header("access_token_auth", "true")
        .body(data)
        .send()
        .await;

    if response.is_err() || !response.as_ref().unwrap().status().is_success() {
        log::error!("Error registering device: {:?}", response);
        return Err(ServerFnError::new("Error registering device"));
    }

    let response = response.unwrap();
    let response = response.json::<HashMap<String, String>>().await?;
    let notification_key = response["notification_key"].clone();

    if user_metadata.notification_key.as_ref().is_none() {
        user_metadata.notification_key = Some(NotificationKey {
            key: notification_key,
            registration_tokens: vec![DeviceRegistrationToken {
                token: registration_token.to_string(),
                device_fingerprint: device_fingerprint.to_string(),
            }],
        });
    } else {
        user_metadata
            .notification_key
            .as_mut()
            .unwrap()
            .registration_tokens
            .push(DeviceRegistrationToken {
                token: registration_token.to_string(),
                device_fingerprint: device_fingerprint.to_string(),
            });
    }
    metadata_client
        .set_user_metadata(user_identity, user_metadata)
        .await?;

    log::info!("Device registered successfully");

    Ok(())
}

#[cfg(feature = "ga4")]
#[server]
pub async fn deregister_device(device_fingerprint: String) -> Result<(), ServerFnError> {
    use ic_agent::Identity;
    use reqwest::Client;
    use yral_canisters_common::Canisters;
    use yral_metadata_client::MetadataClient;

    let canisters = expect_context::<Canisters<true>>();
    let metadata_client: MetadataClient<true> =
        MetadataClient::with_base_url(consts::METADATA_API_BASE.clone());

    let client = Client::new();
    let url = "https://fcm.googleapis.com/fcm/notification";

    let user_identity = canisters.identity();
    let user_principal = user_identity.sender().unwrap();
    let principal_id = user_principal.to_text();
    let notification_key_name = format!("notification_key_{}", principal_id);
    let mut user_metadata = metadata_client
        .get_user_metadata(user_principal)
        .await?
        .ok_or(ServerFnError::new("metadata for principal not found"))?;

    let notification_key = user_metadata
        .notification_key
        .as_ref()
        .ok_or(ServerFnError::new("notification key not found"))?;
    let registration_token = notification_key
        .registration_tokens
        .iter()
        .filter(|token| token.device_fingerprint == device_fingerprint)
        .map(|token| token.token.clone())
        .next()
        .ok_or(ServerFnError::new("device fingerprint not found"))?;

    let data = format!(
        r#"{{
            "operation": "remove",
            "notification_key_name": "{}",
            "notification_key": "{}",
            "registration_ids": ["{}"]
        }}"#,
        notification_key_name, notification_key.key, registration_token
    );

    // TODO: get the token from the app state
    let firebase_token = "temp_token";
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", firebase_token))
        .header("Content-Type", "application/json")
        .header("project_id", "hot-or-not-feed-intelligence")
        .header("access_token_auth", "true")
        .body(data)
        .send()
        .await;

    if response.is_err() || !response.as_ref().unwrap().status().is_success() {
        log::error!("Error deregistering device: {:?}", response);
        return Err(ServerFnError::new("Error deregistering device"));
    }

    user_metadata
        .notification_key
        .as_mut()
        .unwrap()
        .registration_tokens
        .retain(|token| token.device_fingerprint != device_fingerprint);

    metadata_client
        .set_user_metadata(user_identity, user_metadata)
        .await?;

    log::info!("Device registered successfully");

    Ok(())
}
