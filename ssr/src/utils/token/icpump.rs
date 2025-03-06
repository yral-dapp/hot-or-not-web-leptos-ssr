use serde::{Deserialize, Serialize};
use yral_canisters_common::utils::time::current_epoch;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};
use yral_config_cf_kv::KVConfig;

use futures::stream::BoxStream;
use futures::StreamExt;

use leptos::*;

use yral_grpc_traits::{AirdropConfig, AirdropConfigProvider, TokenInfoProvider, TokenListItemFS};

#[cfg(feature = "ssr")]
#[derive(Debug, Clone)]
pub struct ICPumpSearchGrpcChannel {
    pub channel: tonic::transport::Channel,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct TokenListItem {
    pub user_id: String,
    pub name: String,
    pub token_name: String,
    pub token_symbol: String,
    pub logo: String,
    pub description: String,
    pub created_at: String,
    pub timestamp: i64,
    pub formatted_created_at: String,
    pub link: String,
    #[serde(default)]
    pub is_nsfw: bool,
}

#[server]
pub async fn get_token_by_id(token_id: String) -> Result<TokenListItemFS, ServerFnError> {
    #[cfg(feature = "firestore")]
    {
        let firestore_db: firestore::FirestoreDb = expect_context();
        const TEST_COLLECTION_NAME: &str = "tokens-list";

        let token: TokenListItemFS = firestore_db
            .fluent()
            .select()
            .by_id_in(TEST_COLLECTION_NAME)
            .obj()
            .one(token_id)
            .await
            .map_err(|e| ServerFnError::ServerError::<std::convert::Infallible>(e.to_string()))?
            .ok_or_else(|| {
                ServerFnError::ServerError::<std::convert::Infallible>(
                    "Token not found".to_string(),
                )
            })?;

        Ok(token)
    }

    #[cfg(not(feature = "firestore"))]
    {
        Err(ServerFnError::ServerError(
            "Firestore feature not enabled".to_string(),
        ))
    }
}

#[server]
/// page is 1-indexed
pub async fn get_paginated_token_list(page: u32) -> Result<Vec<TokenListItem>, ServerFnError> {
    use crate::consts::ICPUMP_LISTING_PAGE_SIZE;
    get_paginated_token_list_with_limit(page, ICPUMP_LISTING_PAGE_SIZE as u32).await
}

#[server]
/// page is 1-indexed
pub async fn get_paginated_token_list_with_limit(
    page: u32,
    limit: u32,
) -> Result<Vec<TokenListItem>, ServerFnError> {
    #[cfg(feature = "firestore")]
    {
        use firestore::*;
        use speedate::DateTime;

        let firestore_db: firestore::FirestoreDb = expect_context();

        const TEST_COLLECTION_NAME: &str = "tokens-list";

        let object_stream: BoxStream<TokenListItemFS> = firestore_db
            .fluent()
            .select()
            .from(TEST_COLLECTION_NAME)
            .order_by([(
                path!(TokenListItem::created_at),
                FirestoreQueryDirection::Descending,
            )])
            .offset((page - 1) * limit)
            .limit(limit)
            .obj()
            .stream_query()
            .await
            .expect("failed to stream");

        let as_vec: Vec<TokenListItemFS> = object_stream.collect().await;

        let res_vec: Vec<TokenListItem> = as_vec
            .iter()
            .map(|item| {
                let created_at_str = item.created_at.clone();
                let timestamp = DateTime::parse_str(&created_at_str).unwrap().timestamp();
                let now = DateTime::now(0).unwrap().timestamp();
                let elapsed = now - timestamp;

                let elapsed_str = if elapsed < 60 {
                    format!("{}s ago", elapsed)
                } else if elapsed < 3600 {
                    format!("{}m ago", elapsed / 60)
                } else if elapsed < 86400 {
                    format!("{}h ago", elapsed / 3600)
                } else {
                    format!("{}d ago", elapsed / 86400)
                };

                TokenListItem {
                    user_id: item.user_id.clone(),
                    name: item.name.clone(),
                    token_name: item.token_name.clone(),
                    token_symbol: item.token_symbol.clone(),
                    logo: item.logo.clone(),
                    description: item.description.clone(),
                    created_at: item.created_at.clone(),
                    timestamp,
                    formatted_created_at: elapsed_str,
                    link: item.link.clone(),
                    is_nsfw: item.is_nsfw,
                }
            })
            .collect();

        Ok(res_vec)
    }

    #[cfg(not(feature = "firestore"))]
    {
        use candid::Principal;
        let start = (page - 1) * limit;
        let end = start + limit;
        logging::log!("page {page}");
        if page == 1 {
            Ok(vec![TokenListItem {
                user_id: Principal::anonymous().to_text(),
                token_name: format!("Test Token"),
                name: "name".to_string(),
                token_symbol: "TST".to_string(),
                logo: format!("https://picsum.photos/200"),
                description: "This is a test token".to_string(),
                created_at: "69".to_string(),
                formatted_created_at: "69 mins".to_string(),
                link: "https://icpump.fun/token/info/lf5yo-eiaaa-aaaah-alwya-cai/".to_string(),
                is_nsfw: false,
                timestamp: 0,
            }])
        } else {
            Ok(vec![])
        }
    }
}

pub async fn get_mocked_paginated_token_list(page: u32) -> Vec<TokenListItem> {
    use crate::consts::ICPUMP_LISTING_PAGE_SIZE;
    use candid::Principal;

    let page_range = if page == 21 {
        0..5
    } else {
        0..ICPUMP_LISTING_PAGE_SIZE
    };

    page_range
        .map(|idx| {
            let id = idx + ((page - 1) as usize * ICPUMP_LISTING_PAGE_SIZE);

            TokenListItem {
                user_id: Principal::anonymous().to_text(),
                name: format!("Test token {}", id),
                token_name: format!("Test token {}", id),
                token_symbol: format!("TST{}", id),
                logo: "https://picsum.photos/200".to_string(),
                description: "This is a test token".to_string(),
                created_at: "69".to_string(),
                formatted_created_at: "69 mins".to_string(),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64,
                link: format!("{} {}", Principal::anonymous().to_text(), id),
                is_nsfw: false,
            }
        })
        .collect()
}

#[cfg(feature = "ssr")]
pub mod icpump_search {
    tonic::include_proto!("search");
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ICPumpSearchResult {
    pub items: Vec<TokenListItem>,
    pub text: String,
    pub rag_data: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ICPumpSearchResultContexual {
    pub text: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ICPumpChatInteraction {
    pub query: String,
    pub response: String,
}

#[server]
pub async fn get_pumpai_results(query: String) -> Result<ICPumpSearchResult, ServerFnError> {
    use tonic::Request;

    let channel: ICPumpSearchGrpcChannel = expect_context();
    let mut client = icpump_search::search_service_client::SearchServiceClient::with_interceptor(
        channel.channel,
        move |req: Request<()>| Ok(req),
    );

    let request = icpump_search::SearchRequest { input_query: query };
    let resp: tonic::Response<icpump_search::SearchResponseV1> = client.search_v1(request).await?;

    let res = resp.into_inner();
    let items = res.items;

    let res_vec: Vec<TokenListItem> = items.into_iter().map(|item| item.into()).collect();

    Ok(ICPumpSearchResult {
        items: res_vec,
        text: res.answer,
        rag_data: res.rag_data,
    })
}

#[server]
pub async fn get_pumpai_results_contextual(
    query: String,
    previous_interactions: Vec<ICPumpChatInteraction>,
    rag_data: String,
) -> Result<ICPumpSearchResultContexual, ServerFnError> {
    use tonic::Request;

    let channel: ICPumpSearchGrpcChannel = expect_context();
    let mut client = icpump_search::search_service_client::SearchServiceClient::with_interceptor(
        channel.channel,
        move |req: Request<()>| Ok(req),
    );

    let request = icpump_search::ContextualSearchRequest {
        input_query: query,
        previous_interactions: previous_interactions
            .into_iter()
            .map(|item| item.into())
            .collect::<Vec<icpump_search::QueryResponsePair>>(),
        rag_data,
    };
    let resp: tonic::Response<icpump_search::ContextualSearchResponse> =
        client.contextual_search(request).await?;

    let res = resp.into_inner();

    Ok(ICPumpSearchResultContexual { text: res.answer })
}

#[cfg(feature = "ssr")]
impl From<ICPumpChatInteraction> for icpump_search::QueryResponsePair {
    fn from(item: ICPumpChatInteraction) -> Self {
        icpump_search::QueryResponsePair {
            query: item.query,
            response: item.response,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<icpump_search::SearchItemV1> for TokenListItem {
    fn from(item: icpump_search::SearchItemV1) -> Self {
        use speedate::DateTime;

        let created_at_str = item.created_at.clone();
        let timestamp = DateTime::parse_str(&created_at_str).unwrap().timestamp();
        let now = DateTime::now(0).unwrap().timestamp();
        let elapsed = now - timestamp;

        let elapsed_str = if elapsed < 60 {
            format!("{}s ago", elapsed)
        } else if elapsed < 3600 {
            format!("{}m ago", elapsed / 60)
        } else if elapsed < 86400 {
            format!("{}h ago", elapsed / 3600)
        } else {
            format!("{}d ago", elapsed / 86400)
        };

        TokenListItem {
            user_id: item.user_id,
            name: item.token_name.clone(),
            token_name: item.token_name,
            token_symbol: item.token_symbol,
            logo: item.logo,
            description: item.description,
            created_at: item.created_at,
            timestamp,
            formatted_created_at: elapsed_str,
            link: item.link,
            is_nsfw: item.is_nsfw,
        }
    }
}

#[derive(Clone, Copy)]
pub struct IcpumpTokenInfo;

impl TokenInfoProvider for IcpumpTokenInfo {
    type Error = ServerFnError;

    async fn get_token_by_id(&self, token_id: String) -> Result<TokenListItemFS, ServerFnError> {
        get_token_by_id(token_id).await
    }
}

fn get_kv_config() -> Result<KVConfig, ServerFnError> {
    let url = env::var("CF_KV_FETCH_URL").map_err(|e| {
        ServerFnError::ServerError::<std::convert::Infallible>(
            "CF_KV_FETCH_URL is not set".to_string(),
        )
    })?;
    let token = env::var("CF_KV_FETCH_TOKEN").map_err(|e| {
        ServerFnError::ServerError::<std::convert::Infallible>(
            "CF_KV_FETCH_TOKEN is not set".to_string(),
        )
    })?;

    Ok(KVConfig::new(url, token))
}

#[server]
async fn get_airdrop_config_from_kv() -> Result<AirdropConfig, ServerFnError> {
    use derive_more::Display;
    use yral_config_keys::key_derive;

    let kv_config = get_kv_config()?;

    #[derive(Display)]
    #[display("CycleDuration")]
    pub struct CycleDuration;
    key_derive!(CycleDuration => u64|120);

    #[derive(Display)]
    #[display("ClaimLimit")]
    pub struct ClaimLimit;
    key_derive!(ClaimLimit => usize|3);

    let cycle_duration = kv_config.get(CycleDuration).await.map_err(|e| {
        ServerFnError::ServerError::<std::convert::Infallible>(
            "cannot fetch airdrop cycle_duration from cf kv".to_string(),
        )
    })?;
    let claim_limit = kv_config.get(ClaimLimit).await.map_err(|e| {
        ServerFnError::ServerError::<std::convert::Infallible>(
            "cannot fetch airdrop claim_limit from cf kv".to_string(),
        )
    })?;

    Ok(AirdropConfig {
        cycle_duration,
        claim_limit,
    })
}

#[derive(Clone, Copy)]
pub struct AirdropKVConfig;

impl AirdropConfigProvider for AirdropKVConfig {
    async fn get_airdrop_config(&self) -> AirdropConfig {
        get_airdrop_config_from_kv().await.unwrap()
    }
}

#[server]
pub async fn get_airdrop_amount_from_kv() -> Result<u64, ServerFnError> {
    use derive_more::Display;
    use rand::prelude::*;
    use yral_config_keys::key_derive;
    use speedate::DateTime;

    let kv_config = get_kv_config()?;

    #[derive(Display)]
    #[display("AirdropUpperLimit")]
    pub struct AirdropUpperLimit;
    key_derive!(AirdropUpperLimit => u64|100);

    #[derive(Display)]
    #[display("AirdropLowerLimit")]
    pub struct AirdropLowerLimit;
    key_derive!(AirdropLowerLimit => u64|10);

    let upper = kv_config.get(AirdropUpperLimit).await.map_err(|e| {
        ServerFnError::ServerError::<std::convert::Infallible>(
            "cannot fetch airdrop cycle_duration from cf kv".to_string(),
        )
    })?;
    let lower = kv_config.get(AirdropLowerLimit).await.map_err(|e| {
        ServerFnError::ServerError::<std::convert::Infallible>(
            "cannot fetch airdrop claim_limit from cf kv".to_string(),
        )
    })?;

    let mut rng = SmallRng::seed_from_u64(DateTime::now(0).unwrap().timestamp() as u64);
    let amount: u64 = rng.random_range(lower..=upper);

    Ok(amount)
}
