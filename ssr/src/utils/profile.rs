use candid::Principal;
use ic_agent::AgentError;
use leptos::{RwSignal, SignalUpdateUntracked};
use serde::{Deserialize, Serialize};

use crate::{
    canister::individual_user_template::{
        BetDirection, BetOutcomeForBetMaker, PlacedBetDetail, Result5,
        UserProfileDetailsForFrontend,
    },
    component::infinite_scroller::{CursoredDataProvider, KeyedData, PageEntry},
    consts::{GOBGOB_PROPIC_URL, GOBGOB_TOTAL_COUNT},
    state::canisters::Canisters,
};

use super::posts::PostDetails;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProfileDetails {
    pub username: Option<String>,
    pub lifetime_earnings: u64,
    pub followers_cnt: u64,
    pub following_cnt: u64,
    pub profile_pic: Option<String>,
    pub display_name: Option<String>,
    pub principal: Principal,
    pub hots: u64,
    pub nots: u64,
}

impl From<UserProfileDetailsForFrontend> for ProfileDetails {
    fn from(user: UserProfileDetailsForFrontend) -> Self {
        Self {
            username: user.unique_user_name,
            lifetime_earnings: user.lifetime_earnings,
            followers_cnt: user.followers_count,
            following_cnt: user.following_count,
            profile_pic: user.profile_picture_url,
            display_name: user.display_name,
            principal: user.principal_id,
            hots: user.profile_stats.hot_bets_received,
            nots: user.profile_stats.not_bets_received,
        }
    }
}

fn index_from_principal(principal: Principal) -> u32 {
    let hash_value = crc32fast::hash(principal.as_slice());
    (hash_value % GOBGOB_TOTAL_COUNT) + 1
}

impl ProfileDetails {
    pub fn username_or_principal(&self) -> String {
        self.username
            .clone()
            .unwrap_or_else(|| self.principal.to_text())
    }

    pub fn display_name_or_fallback(&self) -> String {
        self.display_name
            .clone()
            .unwrap_or_else(|| self.username_or_principal())
    }

    pub fn profile_pic_or_random(&self) -> String {
        // let propic = self.profile_pic.clone().unwrap_or_default();
        // if !propic.is_empty() {
        //     return propic;
        // }

        propic_from_principal(self.principal)
    }
}

pub fn propic_from_principal(principal: Principal) -> String {
    let index = index_from_principal(principal);
    format!("{GOBGOB_PROPIC_URL}{}/public", index)
}

#[derive(Clone, Copy)]
pub enum BetOutcome {
    Won(u64),
    Draw(u64),
    Lost,
    AwaitingResult,
}

#[derive(Clone, Copy)]
pub enum BetKind {
    Hot,
    Not,
}

#[derive(Clone)]
pub struct BetDetails {
    pub outcome: BetOutcome,
    pub post_id: u64,
    pub canister_id: Principal,
    pub bet_kind: BetKind,
    pub bet_amount: u64,
}

impl From<PlacedBetDetail> for BetDetails {
    fn from(bet: PlacedBetDetail) -> Self {
        let outcome = match bet.outcome_received {
            BetOutcomeForBetMaker::Lost => BetOutcome::Lost,
            BetOutcomeForBetMaker::Draw(w) => BetOutcome::Draw(w),
            BetOutcomeForBetMaker::Won(w) => BetOutcome::Won(w),
            BetOutcomeForBetMaker::AwaitingResult => BetOutcome::AwaitingResult,
        };
        let bet_kind = match bet.bet_direction {
            BetDirection::Hot => BetKind::Hot,
            BetDirection::Not => BetKind::Not,
        };
        Self {
            outcome,
            post_id: bet.post_id,
            canister_id: bet.canister_id,
            bet_kind,
            bet_amount: bet.amount_bet,
        }
    }
}

impl KeyedData for BetDetails {
    type Key = (Principal, u64);

    fn key(&self) -> Self::Key {
        (self.canister_id, self.post_id)
    }
}

pub const PROFILE_CHUNK_SZ: usize = 10;

#[derive(Clone)]
pub struct PostsProvider {
    canisters: Canisters<false>,
    video_queue: RwSignal<Vec<PostDetails>>,
    start_index: RwSignal<usize>,
    user: Principal,
}

impl PostsProvider {
    pub fn new(
        canisters: Canisters<false>,
        video_queue: RwSignal<Vec<PostDetails>>,
        start_index: RwSignal<usize>,
        user: Principal,
    ) -> Self {
        Self {
            canisters,
            video_queue,
            start_index,
            user,
        }
    }
}

impl KeyedData for PostDetails {
    type Key = (Principal, u64);

    fn key(&self) -> Self::Key {
        (self.canister_id, self.post_id)
    }
}

impl CursoredDataProvider for PostsProvider {
    type Data = PostDetails;
    type Error = AgentError;

    async fn get_by_cursor(
        &self,
        start: usize,
        end: usize,
    ) -> Result<PageEntry<PostDetails>, AgentError> {
        let user = self.canisters.individual_user(self.user).await?;
        let limit = end - start;
        let posts = user
            .get_posts_of_this_user_profile_with_pagination_cursor(start as u64, limit as u64)
            .await?;
        let posts = match posts {
            Result5::Ok(v) => v,
            Result5::Err(_) => {
                log::warn!("failed to get posts");
                return Ok(PageEntry {
                    data: vec![],
                    end: true,
                });
            }
        };
        let list_end = posts.len() < (end - start);
        self.start_index.update_untracked(|c| *c = end);
        let post_details: Vec<PostDetails> = posts
            .into_iter()
            .map(|details| PostDetails::from_canister_post(false, self.user, details))
            .collect();
        self.video_queue.update_untracked(|vq| {
            vq.extend_from_slice(&post_details);
        });
        Ok(PageEntry {
            data: post_details,
            end: list_end,
        })
    }
}

#[derive(Clone)]
pub struct BetsProvider {
    canisters: Canisters<false>,
    user: Principal,
}

impl BetsProvider {
    pub fn new(canisters: Canisters<false>, user: Principal) -> Self {
        Self { canisters, user }
    }
}

impl CursoredDataProvider for BetsProvider {
    type Data = BetDetails;
    type Error = AgentError;

    async fn get_by_cursor(
        &self,
        start: usize,
        end: usize,
    ) -> Result<PageEntry<BetDetails>, AgentError> {
        let user = self.canisters.individual_user(self.user).await?;
        assert_eq!(end - start, 10);
        let bets = user
            .get_hot_or_not_bets_placed_by_this_profile_with_pagination(start as u64)
            .await?;
        let list_end = bets.len() < (end - start);
        Ok(PageEntry {
            data: bets.into_iter().map(PlacedBetDetail::into).collect(),
            end: list_end,
        })
    }
}
