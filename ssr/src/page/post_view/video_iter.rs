use std::pin::Pin;

use candid::Principal;
use codee::string::JsonSerdeCodec;
use futures::{stream::FuturesOrdered, Stream, StreamExt};
use leptos::*;
use leptos_use::storage::use_local_storage;

use yral_canisters_client::post_cache::{self, NsfwFilter};

use crate::{
    consts::USER_CANISTER_ID_STORE,
    state::canisters::auth_canisters_store,
    utils::{
        host::show_nsfw_content,
        ml_feed::{
            get_coldstart_feed_paginated, get_coldstart_nsfw_feed_paginated,
            get_posts_ml_feed_cache_paginated,
        },
        posts::FetchCursor,
    },
};
use yral_canisters_common::{utils::posts::PostDetails, Canisters, Error as CanistersError};

type PostsStream<'a> = Pin<Box<dyn Stream<Item = Vec<Result<PostDetails, CanistersError>>> + 'a>>;

#[derive(Debug, Eq, PartialEq)]
pub enum FeedResultType {
    PostCache,
    MLFeedCache,
    MLFeed,
    MLFeedColdstart,
}

pub struct FetchVideosRes<'a> {
    pub posts_stream: PostsStream<'a>,
    pub end: bool,
    pub res_type: FeedResultType,
}

pub struct VideoFetchStream<'a, const AUTH: bool> {
    canisters: &'a Canisters<AUTH>,
    cursor: FetchCursor,
}

impl<'a, const AUTH: bool> VideoFetchStream<'a, AUTH> {
    pub fn new(canisters: &'a Canisters<AUTH>, cursor: FetchCursor) -> Self {
        Self { canisters, cursor }
    }

    pub async fn fetch_post_uids_chunked(
        &self,
        chunks: usize,
        allow_nsfw: bool,
    ) -> Result<FetchVideosRes<'a>, ServerFnError> {
        let post_cache = self.canisters.post_cache().await;
        let top_posts_fut = post_cache
            .get_top_posts_aggregated_from_canisters_on_this_network_for_home_feed_cursor(
                self.cursor.start,
                self.cursor.limit,
                None,
                None,
                Some(if allow_nsfw {
                    NsfwFilter::IncludeNsfw
                } else {
                    NsfwFilter::ExcludeNsfw
                }),
            );
        let top_posts = match top_posts_fut.await? {
            post_cache::Result_::Ok(top_posts) => top_posts,
            post_cache::Result_::Err(post_cache::TopPostsFetchError::ReachedEndOfItemsList) => {
                return Ok(FetchVideosRes {
                    posts_stream: Box::pin(futures::stream::empty()),
                    end: true,
                    res_type: FeedResultType::PostCache,
                })
            }
            post_cache::Result_::Err(_) => {
                return Err(ServerFnError::new("canister refused to send posts"))
            }
        };

        let end = top_posts.len() < self.cursor.limit as usize;
        let chunk_stream = top_posts
            .into_iter()
            .map(move |item| {
                self.canisters
                    .get_post_details(item.publisher_canister_id, item.post_id)
            })
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res.transpose() })
            .chunks(chunks);

        Ok(FetchVideosRes {
            posts_stream: Box::pin(chunk_stream),
            end,
            res_type: FeedResultType::PostCache,
        })
    }

    pub async fn fetch_post_uids_ml_feed_chunked(
        &self,
        chunks: usize,
        _allow_nsfw: bool,
        video_queue: Vec<PostDetails>,
    ) -> Result<FetchVideosRes<'a>, ServerFnError> {
        #[cfg(feature = "hydrate")]
        {
            use crate::utils::ml_feed::ml_feed_grpcweb::MLFeed;

            let ml_feed: MLFeed = expect_context();

            let (user_canister_id_local_storage, _, _) =
                use_local_storage::<Option<Principal>, JsonSerdeCodec>(USER_CANISTER_ID_STORE);
            let user_canister_id;
            if let Some(canister_id) = user_canister_id_local_storage.get_untracked() {
                user_canister_id = canister_id;
            } else {
                let cans_store = auth_canisters_store();
                let mut cans_stream = cans_store.to_stream();
                let cans;
                loop {
                    if let Some(cans_val) = cans_stream.next().await.flatten() {
                        cans = cans_val;
                        break;
                    } else {
                        continue;
                    }
                }
                user_canister_id = cans.user_canister();
            }

            let show_nsfw = show_nsfw_content();
            let top_posts = if show_nsfw {
                ml_feed
                    .get_next_feed_nsfw(&user_canister_id, self.cursor.limit as u32, video_queue)
                    .await
            } else {
                ml_feed
                    .get_next_feed_clean(&user_canister_id, self.cursor.limit as u32, video_queue)
                    .await
            };

            let top_posts = match top_posts {
                Ok(top_posts) => top_posts,
                Err(e) => {
                    return Err(ServerFnError::new(
                        format!("Error fetching ml feed: {e:?}",),
                    ));
                }
            };

            log::info!("top_posts length: {}", top_posts.len());

            let end = false;
            let chunk_stream = top_posts
                .into_iter()
                .map(move |item| self.canisters.get_post_details(item.0, item.1))
                .collect::<FuturesOrdered<_>>()
                .filter_map(|res| async { res.transpose() })
                .chunks(chunks);

            Ok(FetchVideosRes {
                posts_stream: Box::pin(chunk_stream),
                end,
                res_type: FeedResultType::MLFeed,
            })
        }

        #[cfg(not(feature = "hydrate"))]
        {
            return Ok(FetchVideosRes {
                posts_stream: Box::pin(futures::stream::empty()),
                end: true,
                res_type: FeedResultType::MLFeed,
            });
        }
    }

    pub async fn fetch_post_uids_ml_feed_coldstart_chunked(
        &self,
        chunks: usize,
        _allow_nsfw: bool,
        _video_queue: Vec<PostDetails>,
    ) -> Result<FetchVideosRes<'a>, ServerFnError> {
        let top_posts = get_coldstart_feed_paginated(self.cursor.start, self.cursor.limit).await;

        let top_posts = match top_posts {
            Ok(top_posts) => top_posts,
            Err(e) => {
                return Err(ServerFnError::new(
                    format!("Error fetching ml feed: {e:?}",),
                ));
            }
        };

        let end = false;
        let chunk_stream = top_posts
            .into_iter()
            .map(move |item| self.canisters.get_post_details(item.0, item.1))
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res.transpose() })
            .chunks(chunks);

        Ok(FetchVideosRes {
            posts_stream: Box::pin(chunk_stream),
            end,
            res_type: FeedResultType::MLFeedColdstart,
        })
    }

    pub async fn fetch_post_uids_ml_feed_nsfw_coldstart_chunked(
        &self,
        chunks: usize,
        _allow_nsfw: bool,
        _video_queue: Vec<PostDetails>,
    ) -> Result<FetchVideosRes<'a>, ServerFnError> {
        let top_posts =
            get_coldstart_nsfw_feed_paginated(self.cursor.start, self.cursor.limit).await;

        let top_posts = match top_posts {
            Ok(top_posts) => top_posts,
            Err(e) => {
                return Err(ServerFnError::new(
                    format!("Error fetching ml feed: {e:?}",),
                ));
            }
        };

        let end = false;
        let chunk_stream = top_posts
            .into_iter()
            .map(move |item| self.canisters.get_post_details(item.0, item.1))
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res.transpose() })
            .chunks(chunks);

        Ok(FetchVideosRes {
            posts_stream: Box::pin(chunk_stream),
            end,
            res_type: FeedResultType::MLFeedColdstart,
        })
    }
}

impl<'a> VideoFetchStream<'a, true> {
    pub async fn fetch_post_uids_mlfeed_cache_chunked(
        &self,
        chunks: usize,
        allow_nsfw: bool,
        video_queue: Vec<PostDetails>,
    ) -> Result<FetchVideosRes<'a>, ServerFnError> {
        let cans_true = self.canisters;

        let user_canister_id = cans_true.user_canister();

        let top_posts = match get_posts_ml_feed_cache_paginated(
            user_canister_id,
            self.cursor.start,
            self.cursor.limit,
        )
        .await
        {
            Ok(posts) if posts.is_empty() => {
                return self
                    .fetch_post_uids_ml_feed_coldstart_chunked(chunks, allow_nsfw, video_queue)
                    .await;
            }
            Ok(posts) => posts,
            Err(_) => {
                return self
                    .fetch_post_uids_ml_feed_coldstart_chunked(chunks, allow_nsfw, video_queue)
                    .await;
            }
        };

        let end = false;
        let chunk_stream = top_posts
            .into_iter()
            .map(move |item| self.canisters.get_post_details(item.0, item.1))
            .collect::<FuturesOrdered<_>>()
            .filter_map(|res| async { res.transpose() })
            .chunks(chunks);

        Ok(FetchVideosRes {
            posts_stream: Box::pin(chunk_stream),
            end,
            res_type: FeedResultType::MLFeedCache,
        })
    }

    pub async fn fetch_post_uids_hybrid(
        &mut self,
        chunks: usize,
        _allow_nsfw: bool,
        video_queue: Vec<PostDetails>,
    ) -> Result<FetchVideosRes<'a>, ServerFnError> {
        if show_nsfw_content() {
            let res = self
                .fetch_post_uids_ml_feed_chunked(chunks, _allow_nsfw, video_queue.clone())
                .await;

            match res {
                Ok(res) => {
                    return Ok(res);
                }
                Err(_) => {
                    self.cursor.set_limit(15);
                    return self
                        .fetch_post_uids_ml_feed_nsfw_coldstart_chunked(
                            chunks,
                            _allow_nsfw,
                            video_queue,
                        )
                        .await;
                }
            }
        }

        if video_queue.len() < 10 {
            self.cursor.set_limit(15);
            self.fetch_post_uids_mlfeed_cache_chunked(chunks, _allow_nsfw, video_queue)
                .await
        } else {
            let res = self
                .fetch_post_uids_ml_feed_chunked(chunks, _allow_nsfw, video_queue.clone())
                .await;

            match res {
                Ok(res) => Ok(res),
                Err(_) => {
                    self.cursor.set_limit(15);
                    self.fetch_post_uids_ml_feed_coldstart_chunked(chunks, _allow_nsfw, video_queue)
                        .await
                }
            }
        }
    }
}
