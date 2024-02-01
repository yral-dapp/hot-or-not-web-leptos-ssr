use candid::Principal;
use futures::{stream::FuturesOrdered, Stream, StreamExt};
use serde::{Deserialize, Serialize};

use crate::{
    canister::post_cache::{self},
    state::canisters::Canisters,
};

use super::error::PostViewError;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct FetchCursor {
    pub start: u64,
    pub limit: u64,
}

impl Default for FetchCursor {
    fn default() -> Self {
        Self {
            start: 0,
            limit: 10,
        }
    }
}

impl FetchCursor {
    pub fn advance(&mut self) {
        self.start += self.limit;
        self.limit = 20;
    }
}

pub async fn get_post_uid(
    canisters: &Canisters<false>,
    user_canister: Principal,
    post_id: u64,
) -> Result<Option<PostDetails>, PostViewError> {
    let post_creator_can = canisters.individual_user(user_canister);
    let post_details = match post_creator_can
        .get_individual_post_details_by_id(post_id)
        .await
    {
        Ok(p) => p,
        Err(e) => {
            log::warn!("failed to get post details: {}, skipping", e);
            return Ok(None);
        }
    };

    let post_uuid = post_details.video_uid;
    let req_url = format!(
        "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{}/manifest/video.m3u8",
        post_uuid,
    );
    let res = reqwest::Client::default().head(req_url).send().await?;
    if res.status() != 200 {
        return Ok(None);
    }

    Ok(Some(PostDetails {
        canister_id: user_canister,
        post_id,
        uid: post_uuid,
    }))
}

#[derive(Clone, PartialEq, Debug, Hash, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PostDetails {
    pub canister_id: Principal,
    pub post_id: u64,
    pub uid: String,
}

pub struct VideoFetchStream<'a> {
    canisters: &'a Canisters<false>,
    cursor: FetchCursor,
}

impl<'a> VideoFetchStream<'a> {
    pub fn new(canisters: &'a Canisters<false>, cursor: FetchCursor) -> Self {
        Self { canisters, cursor }
    }

    pub async fn fetch_post_uids_chunked(
        self,
        chunks: usize,
    ) -> Result<impl Stream<Item = Vec<Result<PostDetails, PostViewError>>> + 'a, PostViewError>
    {
        let post_cache = self.canisters.post_cache();
        let top_posts_fut = post_cache
            .get_top_posts_aggregated_from_canisters_on_this_network_for_hot_or_not_feed(
                self.cursor.start,
                self.cursor.start + self.cursor.limit,
            );
        // TODO: error handling
        let post_cache::Result_::Ok(top_posts) = top_posts_fut.await? else {
            return Err(PostViewError::Canister(
                "canister refused to send posts".into(),
            ));
        };
        let chunk_stream = top_posts
            .into_iter()
            .map(move |item| get_post_uid(self.canisters, item.publisher_canister_id, item.post_id))
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res.transpose() })
            .chunks(chunks);

        Ok(chunk_stream)
    }
}
