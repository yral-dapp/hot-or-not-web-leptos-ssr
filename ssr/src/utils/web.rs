use leptos_use::use_window;
use wasm_bindgen::prelude::*;
pub struct Share;

#[wasm_bindgen(module = "/public/share.js")]
extern "C" {
    fn shareImage(
        title: Option<&str>,
        text: Option<&str>,
        url: Option<&str>,
        image: Option<js_sys::Array>,
    );
}

impl Share {
    pub fn refer(url: &str) -> Option<()> {
        #[cfg(not(feature = "hydrate"))]
        {
            _ = url;
            None
        }

        #[cfg(feature = "hydrate")]
        {
            use wasm_bindgen::JsValue;

            const TITLE: &str = "Tap Into YRAL: Watch, Play, and Earn!";

            const TEXT: &str  = "Yral is the best web3 social platform where you get paid to watch, engage, and create content. Powered by Blockchain & AI, YRAL rewards your time and attention with real earnings." ;

            const FILE_URL: &str =
             "https://drive.google.com/file/d/1OtB9cvC_Yo6wPxlZAUQDU4za8Ypnh9jx/view?usp=drive_link";

            use leptos::window;
            use web_sys::js_sys::Reflect;
            let window = window();
            let nav = window.navigator();
            if !Reflect::has(&nav, &JsValue::from_str("share")).unwrap_or_default() {
                return None;
            }

            shareImage(
                Some(TITLE),
                Some(TEXT),
                Some(url),
                Some(
                    vec![FILE_URL.to_string()]
                        .into_iter()
                        .map(JsValue::from)
                        .collect(),
                ),
            );

            //   _ = nav.share_with_data(&meta);
            Some(())
        }
    }
}

/// Share a URL with the Web Share API
/// returns None if the API is not available
pub fn share_url(url: &str) -> Option<()> {
    #[cfg(not(feature = "hydrate"))]
    {
        _ = url;
        None
    }
    #[cfg(feature = "hydrate")]
    {
        use leptos::window;
        use wasm_bindgen::JsValue;
        use web_sys::js_sys::Reflect;
        use web_sys::ShareData;
        let window = window();
        let nav = window.navigator();
        if !Reflect::has(&nav, &JsValue::from_str("share")).unwrap_or_default() {
            return None;
        }
        let share_data = ShareData::new();
        share_data.set_url(url);
        _ = nav.share_with_data(&share_data);
        Some(())
    }
}

/// Copy text to clipboard
/// returns None if the API is not available
pub fn copy_to_clipboard(text: &str) -> Option<()> {
    let navigator = use_window().navigator()?;
    _ = navigator.clipboard().write_text(text);
    Some(())
}
