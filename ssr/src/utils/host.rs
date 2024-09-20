pub fn get_host() -> String {
    #[cfg(feature = "hydrate")]
    {
        use leptos::window;
        window().location().host().unwrap().to_string()
    }

    #[cfg(not(feature = "hydrate"))]
    {
        use axum::http::request::Parts;
        use leptos::expect_context;

        let parts: Parts = expect_context();
        let headers = parts.headers;
        headers.get("Host").unwrap().to_str().unwrap().to_string()
    }
}

pub fn show_cdao_page() -> bool {
    let host = get_host();
    host == "icpump.fun"
        || host == "localhost:3000"
        // || host == "hot-or-not-web-leptos-ssr-staging.fly.dev"
     //   || host.contains("go-bazzinga-hot-or-not-web-leptos-ssr.fly.dev") // Use this when testing icpump changes
}
