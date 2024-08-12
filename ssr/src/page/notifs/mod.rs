use candid::types::principal;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// use crate::{
//     component::canisters_prov::{AuthCansProvider, WithAuthCans},
//     state::auth::account_connected_reader,
//     utils::profile::ProfileDetails,
// };

#[component]
fn NotifInnerComponent() -> impl IntoView {
    // let (is_connected, _) = account_connected_reader();

    #[cfg(feature = "hydrate")]
    let token_getter = move || {
        #[wasm_bindgen(module = "/src/page/notifs/setup-firebase-messaging.js")]
        extern "C" {
            fn get_token(flag: bool) -> js_sys::Promise; // Return type as Promise
        }

        #[cfg(feature = "hydrate")]
        {
            spawn_local(async move {
                log::info!("Getting token...");
                let token_promise = get_token(true);
                match JsFuture::from(token_promise).await {
                    Ok(token_js) => {
                        let token: String = token_js.as_string().unwrap_or_default();
                        log::info!("got token: {}", token);
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
        // <h2>{details.username_or_principal()}</h2>
        <div class="flex flex-row gap-2 text-black">
            <button class="p-2 bg-gray-50 rounded-md" on:click=move |_| on_token_click()>"Get Token"</button>
        </div>
    }
}

#[component]
pub fn Notif() -> impl IntoView {
    view! {
        <div class="h-screen w-screen grid grid-cols-1 bg-black justify-items-center place-content-center">
            // <AuthCansProvider let:cans>
                // <NotifInnerComponent details=cans.profile_details()/>
            // </AuthCansProvider>
            <NotifInnerComponent/>
        </div>
    }
}
