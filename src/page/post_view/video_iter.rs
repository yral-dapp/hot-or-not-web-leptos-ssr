use candid::Principal;
use futures::{stream::FuturesOrdered, Stream, StreamExt};

use crate::{
    canister::post_cache::{self},
    state::canisters::Canisters,
};

use super::FetchCursor;

pub async fn get_post_uid(
    canisters: &Canisters,
    user_canister: Principal,
    post_id: u64,
) -> Option<String> {
    let post_creator_can = canisters.individual_user(user_canister);
    let post_details = post_creator_can
        .get_individual_post_details_by_id(post_id)
        .await
        .ok()?;

    let post_uuid = post_details.video_uid;
    let req_url = format!(
        "https://customer-2p3jflss4r4hmpnz.cloudflarestream.com/{}/manifest/video.m3u8",
        post_uuid,
    );
    let head_req = ehttp::Request {
        method: "HEAD".into(),
        url: req_url,
        body: vec![],
        headers: Default::default(),
    };
    let res = ehttp::fetch_async(head_req).await.ok()?;
    if res.status != 200 {
        return None;
    }

    Some(post_uuid)
}

pub struct VideoFetchStream<'a> {
    canisters: &'a Canisters,
    cursor: FetchCursor,
}

impl<'a> VideoFetchStream<'a> {
    pub fn new(canisters: &'a Canisters, cursor: FetchCursor) -> Self {
        Self { canisters, cursor }
    }

    pub async fn fetch_post_uids_chunked(
        self,
        chunks: usize,
    ) -> Option<impl Stream<Item = Vec<String>> + 'a> {
        let post_cache = self.canisters.post_cache();
        let top_posts_fut = post_cache
            .get_top_posts_aggregated_from_canisters_on_this_network_for_hot_or_not_feed(
                self.cursor.start,
                self.cursor.start + self.cursor.limit,
            );
        // TODO: error handling
        let post_cache::Result_::Ok(top_posts) = top_posts_fut.await.unwrap() else {
            unimplemented!();
        };
        let chunk_stream = top_posts
            .into_iter()
            .map(move |item| get_post_uid(self.canisters, item.publisher_canister_id, item.post_id))
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res })
            .chunks(chunks);

        Some(chunk_stream)
    }
}
