pub mod wasp {
    use std::ops::Deref;

    use leptos::{html::Video, HtmlElement};
    use serde::{Deserialize, Serialize};
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/js/wasp-wrapper.js")]
    extern "C" {
        type WaspHlsPlayer;

        fn buildPlayer(videoElement: JsValue, config: JsValue) -> WaspHlsPlayer;

        #[wasm_bindgen(method)]
        fn load(this: &WaspHlsPlayer, url: &str);
        #[wasm_bindgen(method)]
        fn dispose(this: &WaspHlsPlayer);
        #[wasm_bindgen(method)]
        fn addEventListener(this: &WaspHlsPlayer, event: &str, cb: &Closure<dyn Fn(String)>);
        #[wasm_bindgen(method)]
        fn stop(this: &WaspHlsPlayer);
    }

    #[derive(Serialize, Deserialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct WaspHlsConfig {
        pub buffer_goal: Option<f64>,
        pub segment_max_retry: Option<f64>,
        pub segment_request_timeout: Option<f64>,
        pub segment_backoff_base: Option<f64>,
        pub segment_backoff_max: Option<f64>,
        pub multi_variant_playlist_max_retry: Option<f64>,
        pub multi_variant_playlist_request_timeout: Option<f64>,
        pub multi_variant_playlist_backoff_base: Option<f64>,
        pub multi_variant_playlist_backoff_max: Option<f64>,
        pub media_playlist_max_retry: Option<f64>,
        pub media_playlist_request_timeout: Option<f64>,
        pub media_playlist_backoff_base: Option<f64>,
        pub media_playlist_backoff_max: Option<f64>,
    }

    pub struct WaspHlsPlayerW(WaspHlsPlayer);

    impl WaspHlsPlayerW {
        pub fn new(video_element: &HtmlElement<Video>, config: Option<WaspHlsConfig>) -> Self {
            let video_raw: &JsValue = video_element.deref();
            let conf = serde_wasm_bindgen::to_value(&config).unwrap();
            let wasp = buildPlayer(video_raw.clone(), conf);
            Self(wasp)
        }

        pub fn load(&self, url: &str) {
            self.0.load(url);
        }

        pub fn stop(&self) {
            self.0.stop();
        }

        pub fn add_event_listener(&self, event: &str, cb: impl Fn(String) + 'static) {
            let cb = Closure::new(cb);
            self.0.addEventListener(event, &cb);
            // move ownership to js GC
            cb.forget();
        }
    }

    impl Drop for WaspHlsPlayerW {
        fn drop(&mut self) {
            self.0.dispose();
        }
    }
}
