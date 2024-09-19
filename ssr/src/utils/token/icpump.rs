use serde::{Deserialize, Serialize};

use futures::stream::BoxStream;
use futures::StreamExt;

use leptos::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenListItem {
    pub user_id: String,
    pub name: String,
    pub token_name: String,
    pub token_symbol: String,
    pub logo: String,
    pub description: String,
    pub created_at: String,
}

#[cfg(feature = "firestore")]
#[server]
pub async fn get_paginated_token_list(page: u32) -> Result<Vec<TokenListItem>, ServerFnError> {
    use firestore::*;

    use crate::consts::ICPUMP_LISTING_PAGE_SIZE;

    let firestore_db: firestore::FirestoreDb = expect_context();

    const TEST_COLLECTION_NAME: &str = "tokens-list"; //"test-tokens-3"

    let object_stream: BoxStream<TokenListItem> = firestore_db
        .fluent()
        .select()
        .from(TEST_COLLECTION_NAME)
        .order_by([(
            path!(TokenListItem::created_at),
            FirestoreQueryDirection::Descending,
        )])
        .offset((page - 1) * ICPUMP_LISTING_PAGE_SIZE as u32)
        .limit(10)
        .obj()
        .stream_query()
        .await
        .expect("failed to stream");

    let as_vec: Vec<TokenListItem> = object_stream.collect().await;

    Ok(as_vec)
}

#[cfg(not(feature = "firestore"))]
#[server]
pub async fn get_paginated_token_list(page: u32) -> Result<Vec<TokenListItem>, ServerFnError> {
    Ok(vec![])
}
