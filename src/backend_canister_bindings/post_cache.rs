// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal, Encode, Decode};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub enum KnownPrincipalType {
  CanisterIdUserIndex,
  CanisterIdConfiguration,
  CanisterIdProjectMemberIndex,
  CanisterIdTopicCacheIndex,
  CanisterIdRootCanister,
  CanisterIdDataBackup,
  CanisterIdPostCache,
  #[serde(rename="CanisterIdSNSController")]
  CanisterIdSnsController,
  UserIdGlobalSuperAdmin,
}

#[derive(CandidType, Deserialize)]
pub struct PostCacheInitArgs {
  pub known_principal_ids: Option<Vec<(KnownPrincipalType,Principal,)>>,
}

#[derive(CandidType, Deserialize)]
pub struct PostScoreIndexItem {
  pub post_id: u64,
  pub score: u64,
  pub publisher_canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum TopPostsFetchError {
  ReachedEndOfItemsList,
  InvalidBoundsPassed,
  ExceededMaxNumberOfItemsAllowedInOneRequest,
}

#[derive(CandidType, Deserialize)]
pub enum Result_ { Ok(Vec<PostScoreIndexItem>), Err(TopPostsFetchError) }

pub struct Service(pub Principal);
impl Service {
  pub async fn get_top_posts_aggregated_from_canisters_on_this_network_for_home_feed(
    &self,
    arg0: u64,
    arg1: u64,
  ) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "get_top_posts_aggregated_from_canisters_on_this_network_for_home_feed", (
      arg0,arg1,
    )).await
  }
  pub async fn get_top_posts_aggregated_from_canisters_on_this_network_for_hot_or_not_feed(
    &self,
    arg0: u64,
    arg1: u64,
  ) -> Result<(Result_,)> {
    ic_cdk::call(self.0, "get_top_posts_aggregated_from_canisters_on_this_network_for_hot_or_not_feed", (
      arg0,arg1,
    )).await
  }
  pub async fn get_well_known_principal_value(
    &self,
    arg0: KnownPrincipalType,
  ) -> Result<(Option<Principal>,)> {
    ic_cdk::call(self.0, "get_well_known_principal_value", (arg0,)).await
  }
  pub async fn receive_top_home_feed_posts_from_publishing_canister(
    &self,
    arg0: Vec<PostScoreIndexItem>,
  ) -> Result<()> {
    ic_cdk::call(self.0, "receive_top_home_feed_posts_from_publishing_canister", (
      arg0,
    )).await
  }
  pub async fn receive_top_hot_or_not_feed_posts_from_publishing_canister(
    &self,
    arg0: Vec<PostScoreIndexItem>,
  ) -> Result<()> {
    ic_cdk::call(self.0, "receive_top_hot_or_not_feed_posts_from_publishing_canister", (
      arg0,
    )).await
  }
  pub async fn remove_all_feed_entries(&self) -> Result<()> {
    ic_cdk::call(self.0, "remove_all_feed_entries", ()).await
  }
}


