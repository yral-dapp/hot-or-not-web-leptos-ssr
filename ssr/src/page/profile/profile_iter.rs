use candid::Principal;
use futures::stream::{FuturesOrdered, StreamExt, TryStreamExt};

use crate::{
    canister::individual_user_template::{GetPostsOfUserProfileError, Result10},
    state::canisters::Canisters,
    utils::posts::{get_post_uid, PostDetails, PostViewError},
};

#[derive(Clone, Copy, PartialEq)]
pub struct FixedFetchCursor<const LIMIT: u64> {
    pub start: u64,
    pub limit: u64,
}

impl<const LIMIT: u64> FixedFetchCursor<LIMIT> {
    pub fn advance(&mut self) {
        self.start += self.limit;
        self.limit = LIMIT;
    }
}

pub struct PostsRes {
    pub posts: Vec<PostDetails>,
    pub end: bool,
}

pub(crate) trait ProfVideoStream<const LIMIT: u64>: Sized {
    async fn fetch_next_posts<const AUTH: bool>(
        cursor: FixedFetchCursor<LIMIT>,
        canisters: &Canisters<AUTH>,
        user_canister: Principal,
    ) -> Result<PostsRes, PostViewError>;
}

pub struct ProfileVideoBetsStream;

impl ProfVideoStream<10> for ProfileVideoBetsStream {
    async fn fetch_next_posts<const AUTH: bool>(
        cursor: FixedFetchCursor<10>,
        canisters: &Canisters<AUTH>,
        user_canister: Principal,
    ) -> Result<PostsRes, PostViewError> {
        let user = canisters.individual_user(user_canister).await;
        let bets = user
            .get_hot_or_not_bets_placed_by_this_profile_with_pagination(cursor.start)
            .await?;
        let end = bets.len() < 10;
        let posts = bets
            .into_iter()
            .map(|bet| get_post_uid(canisters, bet.canister_id, bet.post_id))
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res.transpose() })
            .try_collect::<Vec<_>>()
            .await?;
        Ok(PostsRes { posts, end })
    }
}

pub struct ProfileVideoStream<const LIMIT: u64>;

impl<const LIMIT: u64> ProfVideoStream<LIMIT> for ProfileVideoStream<LIMIT> {
    async fn fetch_next_posts<const AUTH: bool>(
        cursor: FixedFetchCursor<LIMIT>,
        canisters: &Canisters<AUTH>,
        user_canister: Principal,
    ) -> Result<PostsRes, PostViewError> {
        let user = canisters.individual_user(user_canister).await;
        let posts = user
            .get_posts_of_this_user_profile_with_pagination_cursor(cursor.start, cursor.limit)
            .await?;
        match posts {
            Result10::Ok(v) => {
                let end = v.len() < LIMIT as usize;
                let posts = v
                    .into_iter()
                    .map(|details| PostDetails::from_canister_post(AUTH, user_canister, details))
                    .collect::<Vec<_>>();
                Ok(PostsRes { posts, end })
            }
            Result10::Err(GetPostsOfUserProfileError::ReachedEndOfItemsList) => Ok(PostsRes {
                posts: vec![],
                end: true,
            }),
            _ => Err(PostViewError::Canister(
                "user canister refused to send posts".into(),
            )),
        }
    }
}
