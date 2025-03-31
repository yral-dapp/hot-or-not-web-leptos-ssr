use std::env;

use leptos::prelude::*;
use leptos::server;

#[cfg(feature = "ga4")]
#[server]
pub async fn register_device(
    registration_token: String,
    device_fingerprint: String,
) -> Result<(), ServerFnError> {
    use yral_canisters_common::Canisters;
    use yral_metadata_client::{DeviceRegistrationToken, MetadataClient};

    let canisters = expect_context::<Canisters<true>>();
    let metadata_client: MetadataClient<true> =
        MetadataClient::with_base_url(consts::METADATA_API_BASE.clone());

    let user_identity = canisters.identity();
    metadata_client
        .register_device(
            user_identity,
            DeviceRegistrationToken {
                token: registration_token,
                device_fingerprint,
            },
        )
        .await?;

    log::info!("Device registered successfully");

    Ok(())
}

#[cfg(feature = "ga4")]
#[server]
pub async fn deregister_device(
    registration_token: String,
    device_fingerprint: String,
) -> Result<(), ServerFnError> {
    use yral_canisters_common::Canisters;
    use yral_metadata_client::{DeviceRegistrationToken, MetadataClient};

    let canisters = expect_context::<Canisters<true>>();
    let metadata_client: MetadataClient<true> =
        MetadataClient::with_base_url(consts::METADATA_API_BASE.clone());

    let user_identity = canisters.identity();

    metadata_client
        .unregister_device(
            user_identity,
            DeviceRegistrationToken {
                token: registration_token,
                device_fingerprint,
            },
        )
        .await?;

    log::info!("Device registered successfully");

    Ok(())
}
