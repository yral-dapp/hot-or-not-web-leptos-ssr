use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/install_pwa.js")]
extern "C" {
    #[wasm_bindgen(js_name = triggerPwaInstall)]
    fn trigger_pwa_install();
}

#[component]
pub fn DownloadPwaLink(download_pwa_text: &'static str) -> impl IntoView {
    let install_app = move |_| {
        trigger_pwa_install();
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
