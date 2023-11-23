use candid::Principal;
use ic_agent::Agent;
use leptos::*;
use leptos_router::*;

use crate::backend_canister_bindings::individual_user_template::PostDetailsForFrontend;

#[derive(Params, PartialEq)]
struct PostParams {
    canister_id: String,
    post_id: u64,
}

async fn get_post_details(
    canister_id: String,
    post_id: u64,
) -> (u64, String, Principal, u64, String) {
    let agent = Agent::builder()
        .with_url("https://ic0.app")
        .build()
        .unwrap();

    let post_creator_canister_id = Principal::from_text(canister_id).unwrap();

    let post_creator_canister_service =
        crate::backend_canister_bindings::individual_user_template::Service(
            post_creator_canister_id,
            &agent,
        );

    let PostDetailsForFrontend {
        id,
        description,
        created_by_user_principal_id,
        total_view_count,
        video_uid,
        ..
    } = post_creator_canister_service
        .get_individual_post_details_by_id(post_id)
        .await
        .unwrap();

    (
        id,
        description,
        created_by_user_principal_id,
        total_view_count,
        video_uid,
    )
}

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params::<PostParams>();

    let ids = move || {
        params.with(|params| {
            params
                .as_ref()
                .map(|params| (params.canister_id.clone(), params.post_id))
                .unwrap_or_default()
        })
    };

    view! {
        <Await future=move || get_post_details(ids().0, ids().1) let:received_tuple>
            <p>Post Id: {received_tuple.0}</p>
            <p>Description: {received_tuple.1.clone()}</p>
            <p>Creator: {received_tuple.2.to_text()}</p>
            <p>Total Views: {received_tuple.3}</p>

            <iframe
                src=format!(
                    "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{}/iframe",
                    received_tuple.4,
                )

                style="border: none"
                height="720"
                width="1280"
                allow="accelerometer; gyroscope; autoplay; encrypted-media; picture-in-picture;"
                allowfullscreen="true"
            ></iframe>
        </Await>
    }
}
