use crate::consts::ML_FEED_GRPC_URL;
use candid::Principal;

use super::types::PostId;

#[cfg(feature = "hydrate")]
pub mod ml_feed_grpcweb {
    use super::*;
    use crate::utils::ml_feed::ml_feed_grpcweb::ml_feed_proto::{
        ml_feed_client::MlFeedClient, FeedRequest, PostItem,
    };
    use tonic_web_wasm_client::Client;
    use yral_canisters_common::utils::posts::PostDetails;

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

        pub async fn get_next_feed_clean(
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

            let response = self.client.get_feed_clean(request).await.map_err(|e| {
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

        pub async fn get_next_feed_nsfw(
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

            let response = self.client.get_feed_nsfw(request).await.map_err(|e| {
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

        pub async fn get_next_feed_coldstart(
            mut self,
            limit: u32,
            filter_list: Vec<PostDetails>,
        ) -> Result<Vec<PostId>, tonic::Status> {
            let request = FeedRequest {
                canister_id: "".to_string(),
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

            let response = self.client.get_feed_coldstart(request).await.map_err(|e| {
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

    pub mod ml_feed_proto {
        tonic::include_proto!("ml_feed");
    }

    pub async fn get_coldstart_feed() -> Result<Vec<PostId>, tonic::Status> {
        use crate::utils::ml_feed::ml_feed_grpc::ml_feed_proto::{
            ml_feed_client::MlFeedClient, FeedRequest,
        };
        use tonic::transport::{Channel, ClientTlsConfig};

        let tls_config = ClientTlsConfig::new().with_webpki_roots();

        let channel = Channel::from_static(ML_FEED_GRPC_URL)
            .tls_config(tls_config)
            .expect("Couldn't update TLS config for nsfw agent")
            .connect()
            .await
            .expect("Couldn't connect to ML feed server");

        let mut client = MlFeedClient::new(channel);

        let request = tonic::Request::new(FeedRequest {
            canister_id: "".to_string(),
            filter_posts: vec![],
            num_results: 1,
        });

        let response = client.get_feed_coldstart(request).await.map_err(|e| {
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
