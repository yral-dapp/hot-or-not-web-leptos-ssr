use crate::consts::ML_FEED_GRPC_URL;
use candid::Principal;

use super::types::PostId;

#[cfg(feature = "hydrate")]
pub mod ml_feed_grpcweb {
    use super::*;
    use crate::utils::ml_feed::ml_feed_grpcweb::ml_feed_proto::{
        ml_feed_client::MlFeedClient, FeedRequest, PostItem,
    };
    use crate::utils::posts::PostDetails;
    use tonic_web_wasm_client::Client;

    pub mod ml_feed_proto {
        include!(concat!(env!("OUT_DIR"), "/grpc-web/ml_feed.rs"));
    }

    #[derive(Clone)]
    pub struct MLFeed {
        pub client: MlFeedClient<Client>,
    }

    impl Default for MLFeed {
        fn default() -> Self {
            let client = Client::new(ML_FEED_GRPC_URL.to_string());

            Self {
                client: MlFeedClient::new(client),
            }
        }
    }

    impl MLFeed {
        pub async fn get_next_feed(
            mut self,
            canister_id: &Principal,
            limit: u32,
            filter_list: Vec<PostDetails>,
        ) -> Result<Vec<PostId>, tonic::Status> {
            let request = FeedRequest {
                canister_id: canister_id.to_string(),
                filter_posts: filter_list
                    .iter()
                    .map(|item| PostItem {
                        post_id: item.post_id as u32,
                        canister_id: item.canister_id.to_string(),
                        video_id: item.uid.clone(),
                    })
                    .collect(),
                num_results: limit,
            };

            let response = self.client.get_feed(request).await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Internal,
                    format!("Error fetching posts: {:?}", e),
                )
            })?;

            let feed_res = response.into_inner().feed;

            Ok(feed_res
                .iter()
                .map(|item| {
                    (
                        Principal::from_text(&item.canister_id).unwrap(),
                        item.post_id as u64,
                    )
                })
                .collect())
        }
    }
}

#[cfg(feature = "ssr")]
pub mod ml_feed_grpc {
    use super::*;
    use crate::utils::posts::PostDetails;

    pub mod ml_feed_proto {
        tonic::include_proto!("ml_feed");
    }

    pub async fn get_start_feed(
        canister_id: &Principal,
        limit: u32,
        filter_list: Vec<PostDetails>,
    ) -> Result<Vec<PostId>, tonic::Status> {
        use crate::utils::ml_feed::ml_feed_grpc::ml_feed_proto::{
            ml_feed_client::MlFeedClient, FeedRequest, PostItem,
        };
        use tonic::transport::Channel;

        let channel = Channel::from_static("https://yral-ml-feed-server-staging.fly.dev:443")
            .connect()
            .await
            .expect("Couldn't connect to ML feed server");

        // let mut client = MlFeedClient::connect(ML_FEED_GRPC_URL).await.unwrap();

        let mut client = MlFeedClient::new(channel);

        let request = tonic::Request::new(FeedRequest {
            canister_id: canister_id.to_string(),
            filter_posts: filter_list
                .iter()
                .map(|item| PostItem {
                    post_id: item.post_id as u32,
                    canister_id: item.canister_id.to_string(),
                    video_id: item.uid.clone(),
                })
                .collect(),
            num_results: limit,
        });

        let response = client.get_feed(request).await.map_err(|e| {
            tonic::Status::new(
                tonic::Code::Internal,
                format!("error fetching posts: {:?}", e),
            )
        })?;

        let feed_res = response.into_inner().feed;

        Ok(feed_res
            .iter()
            .map(|item| {
                (
                    Principal::from_text(&item.canister_id).unwrap(),
                    item.post_id as u64,
                )
            })
            .collect())
    }
}
