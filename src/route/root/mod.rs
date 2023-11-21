use ic_agent::{Agent, export::Principal};
use leptos::*;

use crate::backend_canister_bindings::post_cache::Result_;

#[server]
async fn redirect_to_top_post_from_feed() -> Result<String, ServerFnError> {
  // let agent = Agent::builder().build()?;

  // let post_cache_canister_id = Principal::from_text("y6yjf-jyaaa-aaaal-qbd6q-cai").unwrap();

  // let post_cache_canister_service = crate::backend_canister_bindings::post_cache::Service(post_cache_canister_id, &agent);

  // let post_score_item = if let Result_::Ok(post_score_items) = post_cache_canister_service.get_top_posts_aggregated_from_canisters_on_this_network_for_hot_or_not_feed(0, 1).await? {
  //   post_score_items.first().map(|post_score_item| (post_score_item.publisher_canister_id, post_score_item.post_id))
  // } else {
  //   None
  // };

  // logging::log!("🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈🙈 post_score_item: {:?}", post_score_item);
  
  // let redirect_url = match post_score_item {
  //     Some((publisher_canister_id, post_id)) => {
  //       format!("/hot-or-not/{publisher_canister_id}/{post_id}")
  //     },
  //     None => {
  //       format!("/hot-or-not/not-found")
  //     },
  // };

  // leptos_axum::redirect(redirect_url.as_str());
  // leptos_axum::redirect("/something");

  Ok("Hello".to_string())
}

#[component]
pub fn RootPage() -> impl IntoView {
  let redirect_resource = create_blocking_resource(|| {}, |_| async move {redirect_to_top_post_from_feed().await});
  

  view! {
      <h1>Hot or Not</h1>
      <Suspense fallback=move || {
          view! { <p>Loading...</p> }
      }>Received: {move || redirect_resource.get()}</Suspense>
  }
}











