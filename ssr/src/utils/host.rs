pub async fn get_host_async() -> String {
    #[cfg(feature = "hydrate")]
    {
        use leptos::window;
        window().location().host().unwrap().to_string()
    }

    #[cfg(not(feature = "hydrate"))]
    {
        use http::header::HeaderMap;
        use leptos_axum::extract;

        let headers: HeaderMap = extract().await.unwrap();
        let host = headers.get("Host").unwrap().to_str().unwrap();
        host.into()
    }
}


pub fn get_host() -> String {

    #[cfg(feature = "hydrate")]
    {
        use leptos::window;
        window().location().host().unwrap().to_string()
    }

    #[cfg(not(feature = "hydrate"))]
    {
        use axum::http::request::Parts;
        use http::header::HeaderMap;
        use leptos::expect_context;

        let parts: Parts = expect_context();
        let headers = parts.headers;
        headers.get("Host").unwrap().to_str().unwrap().to_string()
    }
}
