use candid::Principal;
use consts::ML_FEED_URL;
use leptos::prelude::*;
use yral_canisters_common::utils::posts::PostDetails;
use yral_types::post::FeedRequest;
use yral_types::post::FeedResponse;
use yral_types::post::PostItem;

// use yral_ml_feed_cache::types::PostItem;

// #[cfg(feature = "hydrate")]
// pub mod ml_feed_grpcweb {
//     use super::*;
//     use crate::ml_feed::ml_feed_grpcweb::ml_feed_proto::{
//         ml_feed_client::MlFeedClient, FeedRequest, PostItem,
//     };
//     use tonic_web_wasm_client::Client;
//     use yral_canisters_common::utils::posts::PostDetails;

//     pub mod ml_feed_proto {
//         include!(concat!(env!("OUT_DIR"), "/grpc-web/ml_feed.rs"));
//     }

//     #[derive(Clone)]
//     pub struct MLFeed {
//         pub client: MlFeedClient<Client>,
//     }

//     impl Default for MLFeed {
//         fn default() -> Self {
//             let client = Client::new(ML_FEED_GRPC_URL.to_string());

//             Self {
//                 client: MlFeedClient::new(client),
//             }
//         }
//     }

//     impl MLFeed {
//         pub async fn get_next_feed(
//             mut self,
//             canister_id: &Principal,
//             limit: u32,
//             filter_list: Vec<PostDetails>,
//         ) -> Result<Vec<PostId>, tonic::Status> {
//             let request = FeedRequest {
//                 canister_id: canister_id.to_string(),
//                 filter_posts: filter_list
//                     .iter()
//                     .map(|item| PostItem {
//                         post_id: item.post_id as u32,
//                         canister_id: item.canister_id.to_string(),
//                         video_id: item.uid.clone(),
//                     })
//                     .collect(),
//                 num_results: limit,
//             };

//             let response = self.client.get_feed(request).await.map_err(|e| {
//                 tonic::Status::new(
//                     tonic::Code::Internal,
//                     format!("Error fetching posts: {:?}", e),
//                 )
//             })?;

//             let feed_res = response.into_inner().feed;

//             Ok(feed_res
//                 .iter()
//                 .map(|item| {
//                     (
//                         Principal::from_text(&item.canister_id).unwrap(),
//                         item.post_id as u64,
//                     )
//                 })
//                 .collect())
//         }

//         pub async fn get_next_feed_clean(
//             mut self,
//             canister_id: &Principal,
//             limit: u32,
//             filter_list: Vec<PostDetails>,
//         ) -> Result<Vec<PostId>, tonic::Status> {
//             let request = FeedRequest {
//                 canister_id: canister_id.to_string(),
//                 filter_posts: filter_list
//                     .iter()
//                     .map(|item| PostItem {
//                         post_id: item.post_id as u32,
//                         canister_id: item.canister_id.to_string(),
//                         video_id: item.uid.clone(),
//                     })
//                     .collect(),
//                 num_results: limit,
//             };

//             let response = self.client.get_feed_clean(request).await.map_err(|e| {
//                 tonic::Status::new(
//                     tonic::Code::Internal,
//                     format!("Error fetching posts: {:?}", e),
//                 )
//             })?;

//             let feed_res = response.into_inner().feed;

//             Ok(feed_res
//                 .iter()
//                 .map(|item| {
//                     (
//                         Principal::from_text(&item.canister_id).unwrap(),
//                         item.post_id as u64,
//                     )
//                 })
//                 .collect())
//         }

//         pub async fn get_next_feed_nsfw(
//             mut self,
//             canister_id: &Principal,
//             limit: u32,
//             filter_list: Vec<PostDetails>,
//         ) -> Result<Vec<PostId>, tonic::Status> {
//             let request = FeedRequest {
//                 canister_id: canister_id.to_string(),
//                 filter_posts: filter_list
//                     .iter()
//                     .map(|item| PostItem {
//                         post_id: item.post_id as u32,
//                         canister_id: item.canister_id.to_string(),
//                         video_id: item.uid.clone(),
//                     })
//                     .collect(),
//                 num_results: limit,
//             };

//             let response = self.client.get_feed_nsfw(request).await.map_err(|e| {
//                 tonic::Status::new(
//                     tonic::Code::Internal,
//                     format!("Error fetching posts: {:?}", e),
//                 )
//             })?;

//             let feed_res = response.into_inner().feed;

//             Ok(feed_res
//                 .iter()
//                 .map(|item| {
//                     (
//                         Principal::from_text(&item.canister_id).unwrap(),
//                         item.post_id as u64,
//                     )
//                 })
//                 .collect())
//         }

//         pub async fn get_next_feed_coldstart(
//             mut self,
//             limit: u32,
//             filter_list: Vec<PostDetails>,
//         ) -> Result<Vec<PostId>, tonic::Status> {
//             let request = FeedRequest {
//                 canister_id: "".to_string(),
//                 filter_posts: filter_list
//                     .iter()
//                     .map(|item| PostItem {
//                         post_id: item.post_id as u32,
//                         canister_id: item.canister_id.to_string(),
//                         video_id: item.uid.clone(),
//                     })
//                     .collect(),
//                 num_results: limit,
//             };

//             let response = self.client.get_feed_coldstart(request).await.map_err(|e| {
//                 tonic::Status::new(
//                     tonic::Code::Internal,
//                     format!("Error fetching posts: {:?}", e),
//                 )
//             })?;

//             let feed_res = response.into_inner().feed;

//             Ok(feed_res
//                 .iter()
//                 .map(|item| {
//                     (
//                         Principal::from_text(&item.canister_id).unwrap(),
//                         item.post_id as u64,
//                     )
//                 })
//                 .collect())
//         }
//     }
// }

// #[cfg(feature = "ssr")]
// pub mod ml_feed_grpc {
//     use super::*;

//     pub mod ml_feed_proto {
//         tonic::include_proto!("ml_feed");
//     }

//     pub async fn get_coldstart_feed() -> Result<Vec<PostId>, tonic::Status> {
//         use crate::ml_feed::ml_feed_grpc::ml_feed_proto::{
//             ml_feed_client::MlFeedClient, FeedRequest,
//         };
//         use tonic::transport::{Channel, ClientTlsConfig};

//         let tls_config = ClientTlsConfig::new().with_webpki_roots();

//         let channel = Channel::from_static(ML_FEED_GRPC_URL)
//             .tls_config(tls_config)
//             .expect("Couldn't update TLS config for nsfw agent")
//             .connect()
//             .await
//             .expect("Couldn't connect to ML feed server");

//         let mut client = MlFeedClient::new(channel);

//         let request = tonic::Request::new(FeedRequest {
//             canister_id: "".to_string(),
//             filter_posts: vec![],
//             num_results: 1,
//         });

//         let response = client.get_feed_coldstart(request).await.map_err(|e| {
//             tonic::Status::new(
//                 tonic::Code::Internal,
//                 format!("error fetching posts: {:?}", e),
//             )
//         })?;

//         let feed_res = response.into_inner().feed;

//         Ok(feed_res
//             .iter()
//             .map(|item| {
//                 (
//                     Principal::from_text(&item.canister_id).unwrap(),
//                     item.post_id as u64,
//                 )
//             })
//             .collect())
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct CustomMlFeedCacheItem {
//     post_id: u64,
//     canister_id: String,
//     video_id: String,
//     creator_principal_id: String,
// }

// #[server]
// pub async fn get_posts_ml_feed_cache_paginated(
//     canister_id: Principal,
//     start: u64,
//     limit: u64,
// ) -> Result<Vec<PostId>, ServerFnError> {
//     get_posts_ml_feed_cache_paginated_impl(canister_id.to_text(), start, limit).await
// }

// #[server]
// pub async fn get_coldstart_feed_paginated(
//     start: u64,
//     limit: u64,
// ) -> Result<Vec<PostId>, ServerFnError> {
//     get_posts_ml_feed_cache_paginated_impl("global-feed".to_string(), start, limit).await
// }

// #[server]
// pub async fn get_coldstart_nsfw_feed_paginated(
//     start: u64,
//     limit: u64,
// ) -> Result<Vec<PostId>, ServerFnError> {
//     get_posts_ml_feed_cache_paginated_impl("global-feed-nsfw".to_string(), start, limit).await
// }

// pub async fn get_posts_ml_feed_cache_paginated_impl(
//     canister_id_str: String,
//     start: u64,
//     limit: u64,
// ) -> Result<Vec<PostId>, ServerFnError> {
//     let client = reqwest::Client::new();

//     let url = format!(
//         "https://yral-ml-feed-cache.go-bazzinga.workers.dev/feed-cache/{}?start={}&limit={}",
//         canister_id_str, start, limit
//     );

//     let response = client
//         .get(&url)
//         .header("Content-Type", "application/json")
//         .send()
//         .await?;

//     if !response.status().is_success() {
//         return Ok(vec![]);
//     }

//     let response = response.json::<Vec<CustomMlFeedCacheItem>>().await.unwrap();

//     Ok(response
//         .into_iter()
//         .map(|item| {
//             (
//                 Principal::from_text(&item.canister_id).unwrap(),
//                 item.post_id,
//             )
//         })
//         .collect::<Vec<PostId>>())
// }

// New v2 REST APIs

pub async fn get_ml_feed_coldstart_clean(
    canister_id: Principal,
    num_results: u32,
    filter_results: Vec<PostDetails>,
) -> Result<Vec<PostItem>, anyhow::Error> {
    let client = reqwest::Client::new();
    let ml_feed_url = ML_FEED_URL.join("api/v1/feed/coldstart/clean").unwrap();

    let req = FeedRequest {
        canister_id,
        filter_results: post_details_to_post_item(filter_results),
        num_results,
    };

    let response = client.post(ml_feed_url).json(&req).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(format!(
            "Error fetching ML feed: {:?}",
            response.text().await?
        )));
    }
    let response = response.json::<FeedResponse>().await?;

    Ok(response.posts)
}

pub async fn get_ml_feed_coldstart_nsfw(
    canister_id: Principal,
    num_results: u32,
    filter_results: Vec<PostDetails>,
) -> Result<Vec<PostItem>, anyhow::Error> {
    let client = reqwest::Client::new();
    let ml_feed_url = ML_FEED_URL.join("api/v1/feed/coldstart/nsfw").unwrap();

    let req = FeedRequest {
        canister_id,
        filter_results: post_details_to_post_item(filter_results),
        num_results,
    };

    let response = client.post(ml_feed_url).json(&req).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(format!(
            "Error fetching ML feed: {:?}",
            response.text().await?
        )));
    }
    let response = response.json::<FeedResponse>().await?;

    Ok(response.posts)
}

pub async fn get_ml_feed_coldstart_mixed(
    canister_id: Principal,
    num_results: u32,
    filter_results: Vec<PostDetails>,
) -> Result<Vec<PostItem>, anyhow::Error> {
    let client = reqwest::Client::new();
    let ml_feed_url = ML_FEED_URL.join("api/v1/feed/coldstart/mixed").unwrap();

    let req = FeedRequest {
        canister_id,
        filter_results: post_details_to_post_item(filter_results),
        num_results,
    };

    let response = client.post(ml_feed_url).json(&req).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(format!(
            "Error fetching ML feed: {:?}",
            response.text().await?
        )));
    }
    let response = response.json::<FeedResponse>().await?;

    Ok(response.posts)
}

pub async fn get_ml_feed_clean(
    canister_id: Principal,
    num_results: u32,
    filter_results: Vec<PostDetails>,
) -> Result<Vec<PostItem>, anyhow::Error> {
    let client = reqwest::Client::new();
    let ml_feed_url = ML_FEED_URL.join("api/v1/feed/clean").unwrap();

    let req = FeedRequest {
        canister_id,
        filter_results: post_details_to_post_item(filter_results),
        num_results,
    };

    let response = client.post(ml_feed_url).json(&req).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(format!(
            "Error fetching ML feed: {:?}",
            response.text().await?
        )));
    }
    let response = response.json::<FeedResponse>().await?;

    Ok(response.posts)
}

pub async fn get_ml_feed_nsfw(
    canister_id: Principal,
    num_results: u32,
    filter_results: Vec<PostDetails>,
) -> Result<Vec<PostItem>, anyhow::Error> {
    let client = reqwest::Client::new();
    let ml_feed_url = ML_FEED_URL.join("api/v1/feed/nsfw").unwrap();

    let req = FeedRequest {
        canister_id,
        filter_results: post_details_to_post_item(filter_results),
        num_results,
    };

    let response = client.post(ml_feed_url).json(&req).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(format!(
            "Error fetching ML feed: {:?}",
            response.text().await?
        )));
    }
    let response = response.json::<FeedResponse>().await?;

    Ok(response.posts)
}

pub async fn get_ml_feed_mixed(
    canister_id: Principal,
    num_results: u32,
    filter_results: Vec<PostDetails>,
) -> Result<Vec<PostItem>, anyhow::Error> {
    let client = reqwest::Client::new();
    let ml_feed_url = ML_FEED_URL.join("api/v1/feed/mixed").unwrap();

    let req = FeedRequest {
        canister_id,
        filter_results: post_details_to_post_item(filter_results),
        num_results,
    };

    let response = client.post(ml_feed_url).json(&req).send().await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(format!(
            "Error fetching ML feed: {:?}",
            response.text().await?
        )));
    }
    let response = response.json::<FeedResponse>().await?;

    Ok(response.posts)
}

pub fn post_details_to_post_item(post_details: Vec<PostDetails>) -> Vec<PostItem> {
    post_details
        .into_iter()
        .map(|post_detail| PostItem {
            post_id: post_detail.post_id,
            canister_id: post_detail.canister_id,
            video_id: post_detail.uid,
            nsfw_probability: post_detail.nsfw_probability,
        })
        .collect()
}
