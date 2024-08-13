use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use crate::{
    component::canisters_prov::AuthCansProvider,
    state::auth::account_connected_reader,
    utils::{device_id::send_principal_and_token_offchain, profile::ProfileDetails},
};

#[component]
fn NotifInnerComponent(details: ProfileDetails) -> impl IntoView {
    let (_, _) = account_connected_reader();

    #[cfg(feature = "hydrate")]
    let token_getter = move || {
        #[wasm_bindgen(module = "/src/page/notifs/setup-firebase-messaging.js")]
        extern "C" {
            fn get_token(vapidKey: String) -> js_sys::Promise;
        }

        #[cfg(feature = "hydrate")]
        {
            let principal_id = details.principal.to_string();

            spawn_local(async move {
                log::info!("Getting token...");

                let token_promise = get_token(env!("vapidKey").to_string());
                match JsFuture::from(token_promise).await {
                    Ok(token_js) => {
                        let token: String = token_js.as_string().unwrap_or_default();
                        log::info!("sending offchain with params: {}, {}", token, principal_id);
                        send_principal_and_token_offchain(token, principal_id)
                            .await
                            .unwrap();
                    }
                    Err(err) => {
                        log::warn!("Failed to get token: {:?}", err);
                    }
                }
            });
        }
    };

    let on_token_click = move || {
        #[cfg(feature = "hydrate")]
        {
            token_getter();
        }
    };

    view! {
        <h1>"YRAL Notifs for"</h1>
        <h2>{details.username_or_principal()}</h2>
        <br/>
        <div class="flex flex-row gap-2 text-black">
            <button class="p-2 bg-gray-200 rounded-md" on:click=move |_| on_token_click()>"Get Token"</button>
        </div>
    }
}

#[component]
pub fn Notif() -> impl IntoView {
    view! {
        <div class="h-screen w-screen grid grid-cols-1 justify-items-center place-content-center">
            <AuthCansProvider let:cans>
                <NotifInnerComponent details=cans.profile_details()/>
            </AuthCansProvider>
        </div>
    }
}
