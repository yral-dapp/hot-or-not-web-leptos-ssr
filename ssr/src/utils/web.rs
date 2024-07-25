use gloo::file::ObjectUrl;
use leptos_use::use_window;

#[derive(Clone)]
pub struct FileWithUrl {
    pub file: gloo::file::File,
    pub url: ObjectUrl,
}

impl FileWithUrl {
    #[cfg(feature = "hydrate")]
    pub fn new(file: gloo::file::File) -> Self {
        let url = ObjectUrl::from(file.clone());
        Self { file, url }
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
        use web_sys::{js_sys::Reflect, ShareData};
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
