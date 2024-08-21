use leptos_use::use_window;

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
