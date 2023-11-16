use ic_agent::{Agent, export::Principal};
use leptos::*;

// #[server]
async fn redirect_to_top_post_from_feed() -> Result<(), ServerFnError> {
  let agent = Agent::builder().build()?;

  let post_cache_canister = Principal::from_text("y6yjf-jyaaa-aaaal-qbd6q-cai").unwrap();

  let top_post_details = agent.query(&post_cache_canister, "get_top_posts_aggregated_from_canisters_on_this_network_for_home_feed").with_arg(candid::encode_args((0, 1)).unwrap()).call().await.unwrap();

  // leptos_axum::redirect("/hot-or-not/<canister_id>/1");

  Ok(())
}

mod test {
  use super::*;

  #[test]
  fn test_redirect_to_top_post_from_feed() {
    redirect_to_top_post_from_feed();
  }
}



























