use leptos_use::use_window;
use wasm_bindgen::JsCast;
pub struct Share;
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

            //   const FILE_URL: &str =
            // "https://drive.google.com/file/d/1OtB9cvC_Yo6wPxlZAUQDU4za8Ypnh9jx/view?usp=drive_link";

            // spawn_local(async {
            //   file = match   Self::fetch_image(FILE_URL).await {
            //     Ok(blob) => blob,
            // }  ;
            // });

            const URL: &str = "https://yral.com/?user_refer=cp7dg-n36pb-3bcja-caqkm-vcanj-t37c7-p7ptb-h3tls-6srot-2jz7m-6ae";

            let meta = web_sys::ShareData::new()
                .text(&TEXT)
                .title(&TITLE)
                .url(&URL)
                //                .files(&JsValue::from_str(FILE_URL))
                .clone();

            use leptos::window;
            use web_sys::js_sys::Reflect;
            let window = window();
            let nav = window.navigator();
            if !Reflect::has(&nav, &JsValue::from_str("share")).unwrap_or_default() {
                return None;
            }
            _ = nav.share_with_data(&meta);
            Some(())
        }
    }

    //  async fn fetch_image(url: &str) -> Result<web_sys::Response, wasm_bindgen::JsValue> {
    //   use leptos::window;
    // let window = window();

    //   let resp_promise = window.fetch_with_str(url);
    //  let resp = JsFuture::from(resp_promise).await?;

    // Ensure the response is a Blob
    //  let response: web_sys::Response = resp.dyn_into()?;
    //   let blob_promise = response.blob()?;
    //  let blob = wasm_bindgen_futures::JsFuture::from(blob_promise).await?;

    // Ok(blob.dyn_into()?)
    // }
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
        _ = nav.share_with_data(ShareData::new().url(url));
        Some(())
    }
}

/// Copy text to clipboard
/// returns None if the API is not available
pub fn copy_to_clipboard(text: &str) -> Option<()> {
    let navigator = use_window().navigator()?;
    _ = navigator.clipboard()?.write_text(text);
    Some(())
}
