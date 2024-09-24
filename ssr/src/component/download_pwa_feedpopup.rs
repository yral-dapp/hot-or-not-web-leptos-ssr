use js_sys::Promise;
use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/install_pwa.js")]
extern "C" {
    #[wasm_bindgen(js_name = triggerPwaInstall)]
    fn trigger_pwa_install() -> Promise;
}

#[component]
pub fn DownloadPwaLink(download_pwa_text: &'static str) -> impl IntoView {
    let install_app = move |_| {
        spawn_local(async move {
            let promise = trigger_pwa_install(); // Get the Promise
            match JsFuture::from(promise).await {
                Ok(result) => {
                    let outcome = result.as_string().unwrap_or_else(|| "Unknown".to_string());
                    log::info!("PWA install outcome: {}", outcome);
                }
                Err(err) => {
                    log::error!("Failed to trigger PWA install: {:?}", err);
                }
            }
        });
    };
    view! {
        <button
            id="installApp"
            class="py-4 px-6 w-full text-lg font-bold text-center text-white rounded-full md:py-5 md:text-xl bg-primary-600"
            on:click=install_app
        >
            {download_pwa_text}
        </button>
    }
}
