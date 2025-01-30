use std::sync::LazyLock;

use regex::Regex;

pub fn get_host() -> String {
    #[cfg(feature = "hydrate")]
    {
        use leptos::window;
        window().location().host().unwrap().to_string()
    }

    #[cfg(not(feature = "hydrate"))]
    {
        use axum::http::request::Parts;
        use leptos::{expect_context, use_context};

        let parts: Option<Parts> = use_context();
        if parts.is_none() {
            return "".to_string();
        }
        let headers = parts.unwrap().headers;
        headers.get("Host").unwrap().to_str().unwrap().to_string()
    }
}

pub fn show_cdao_page() -> bool {
    let host = get_host();
    show_cdao_condition(host)
}

pub fn is_host_a_preview_link(host: &str) -> bool {
    static PR_PREVIEW_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^pr-\d*-yral-dapp-hot-or-not-web-leptos-ssr\.fly\.dev$").unwrap()
    });

    PR_PREVIEW_PATTERN.is_match_at(host, 0)
}

pub fn show_preview_component() -> bool {
    let host = get_host();
    host.contains("yral-dapp-hot-or-not-web-leptos-ssr.fly.dev")
}

pub fn show_cdao_condition(host: String) -> bool {
    host == "icpump.fun"
    // || host == "localhost:3000"
    // || host == "hot-or-not-web-leptos-ssr-staging.fly.dev"
    || host.contains("yral-dapp-hot-or-not-web-leptos-ssr.fly.dev") // Use this when testing icpump changes
}

pub fn show_nsfw_content() -> bool {
    let host = get_host();
    show_nsfw_condition(host)
}

pub fn show_nsfw_condition(host: String) -> bool {
    host == "hotornot.wtf" || host == "127.0.0.1:3000"
    // || host.contains("yral-dapp-hot-or-not-web-leptos-ssr.fly.dev")
}
