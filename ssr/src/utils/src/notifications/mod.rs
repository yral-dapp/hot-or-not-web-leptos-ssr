use wasm_bindgen::prelude::*;

pub mod register_device;

#[wasm_bindgen(module = "/src/notifications/setup-firebase-messaging-inline.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = getToken)]
    async fn get_token() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = getDeviceFingerprint)]
    async fn get_device_fingerprint() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_name = getNotificationPermission)]
    async fn get_notification_permission() -> Result<JsValue, JsValue>;
}

pub async fn register_device_for_principal(principal_id: String) {
    let permission = match get_notification_permission().await {
        Ok(permission_js) => permission_js.as_bool().unwrap(),
        Err(err) => {
            log::warn!("Failed to get notification permission: {:?}", err);
            return;
        }
    };
    if !permission {
        // TODO: show a notification to the user to allow notifications
        log::warn!("Notification permission not granted");
        return;
    }

    let device_fingerprint = match get_device_fingerprint().await {
        Ok(device_fingerprint_js) => device_fingerprint_js.as_string().unwrap(),
        Err(err) => {
            log::warn!("Failed to get device fingerprint: {:?}", err);
            return;
        }
    };

    let token = match get_token().await {
        Ok(token_js) => token_js.as_string().unwrap(),
        Err(err) => {
            log::warn!("Failed to get token: {:?}", err);
            return;
        }
    };

    #[cfg(feature = "ga4")]
    {
        use register_device::register_device;
        log::info!(
            "registering device with params: {}, {}, {}",
            token,
            principal_id,
            device_fingerprint
        );
        register_device(token.clone(), device_fingerprint)
            .await
            .unwrap();
    }
}
