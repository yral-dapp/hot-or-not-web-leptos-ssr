use leptos::{expect_context, StoredValue};
use leptos_use::use_window;
use serde::{Deserialize, Serialize};

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserAgent {
    IosSafari,
    Other,
}

pub type UserAgentCtx = StoredValue<Option<UserAgent>>;

fn get_ua() -> UserAgent {
    #[cfg(feature = "hydrate")]
    {
        use js_sys::RegExp;
        use leptos::window;
        let window = window();
        let Ok(ua) = window.navigator().user_agent() else {
            return UserAgent::Other;
        };
        let ios_re = RegExp::new("iP(ad|od|hone)", "i");
        let safari_re = RegExp::new(r"Version/[\\d\\.]+.*Safari", "");
        if ios_re.test(&ua) && safari_re.test(&ua) {
            UserAgent::IosSafari
        } else {
            UserAgent::Other
        }
    }

    #[cfg(not(feature = "hydrate"))]
    {
        use http::{header::USER_AGENT, request::Parts};
        use leptos::expect_context;
        use regex::{Regex, RegexBuilder};

        let parts: Parts = expect_context();
        let headers = parts.headers;
        let Some(agent) = headers.get(USER_AGENT).and_then(|a| a.to_str().ok()) else {
            return UserAgent::Other;
        };
        let ios_re = RegexBuilder::new(r"iP(ad|od|hone)")
            .case_insensitive(true)
            .build()
            .unwrap();
        let safari_re = Regex::new(r"Version/[\d\.]+.*Safari").unwrap();
        if ios_re.is_match(agent) && safari_re.is_match(agent) {
            UserAgent::IosSafari
        } else {
            UserAgent::Other
        }
    }
}

pub fn user_agent() -> UserAgent {
    let ctx: UserAgentCtx = expect_context();
    if let Some(ua) = ctx.try_get_value().flatten() {
        return ua;
    }

    let ua = get_ua();

    _ = ctx.try_set_value(Some(ua));
    ua
}
