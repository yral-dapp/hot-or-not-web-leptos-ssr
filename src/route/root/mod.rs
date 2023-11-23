use leptos::*;

#[server]
async fn redirect_to_top_post_from_feed() -> Result<(), ServerFnError> {
    use crate::backend_canister_bindings::post_cache::Result_;
    use ic_agent::{export::Principal, Agent};

    let agent = Agent::builder()
        .with_url("https://ic0.app")
        .build()
        .unwrap();

    let post_cache_canister_id = Principal::from_text("y6yjf-jyaaa-aaaal-qbd6q-cai").unwrap();

    let post_cache_canister_service =
        crate::backend_canister_bindings::post_cache::Service(post_cache_canister_id, &agent);

    let post_score_item = if let Result_::Ok(post_score_items) = post_cache_canister_service
        .get_top_posts_aggregated_from_canisters_on_this_network_for_hot_or_not_feed(0, 1)
        .await?
    {
        post_score_items.first().map(|post_score_item| {
            (
                post_score_item.publisher_canister_id,
                post_score_item.post_id,
            )
        })
    } else {
        None
    };

    let redirect_url = match post_score_item {
        Some((publisher_canister_id, post_id)) => {
            format!("/hot-or-not/{publisher_canister_id}/{post_id}")
        }
        None => {
            format!("/hot-or-not/not-found")
        }
    };

    leptos_axum::redirect(redirect_url.as_str());

    Ok(())
}

#[component]
pub fn RootPage() -> impl IntoView {
    let redirect_resource = create_blocking_resource(
        || {},
        |_| async move { redirect_to_top_post_from_feed().await },
    );

    view! { <Suspense>Received: {move || redirect_resource.get()}</Suspense> }
}
