use candid::Nat;
use serde::{Deserialize, Serialize};

use crate::{
    component::infinite_scroller::{CursoredDataProvider, KeyedData, PageEntry},
    error_template::AppError,
};
use futures::stream::BoxStream;
use futures::StreamExt;

use leptos::*;

use super::{TokenBalance, TokenMetadata};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen_futures::JsFuture;

// #[derive(Clone)]
// pub struct TokensProvider {
//     query: String,
// }

// impl TokensProvider {
//     pub fn new(query: String) -> Self {
//         Self { query }
//     }
// }

// impl KeyedData for TokenMetadata {
//     type Key = String;

//     fn key(&self) -> Self::Key {
//         self.symbol.clone()
//     }
// }

// const TEST_IMG: &str = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAQAAAAEACAIAAADTED8xAAADS0lEQVR4nOzVIW4UYBRGUYYQBILBTtgACThWMQqCxCFAIElAsA4SFCvAsgBa1RU09U3a1LWiqqJdxBN/JvecFXzi3bwnxx8ePzpkf969Xz1h5N/v/eoJI5+3n1ZPGDns64chAZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgbbO7OFu9YeTu/4/VE0a+/T1ZPWHkZnu+esKID0CaAEgTAGkCIE0ApAmANAGQJgDSBECaAEgTAGkCIE0ApAmANAGQJgDSBECaAEgTAGkCIE0ApAmANAGQJgDSBECaAEgTAGkCIE0ApAmANAGQJgDSBECaAEgTAGkCIE0ApAmANAGQJgDSBECaAEgTAGkCIE0ApAmANAGQJgDSBECaAEgTAGkCIE0ApAmANAGQJgDSBECaAEgTAGkCIE0ApAmANAGQtvn+Zb96w8jV5enqCSMfX39dPWHk+dHt6gkjPgBpAiBNAKQJgDQBkCYA0gRAmgBIEwBpAiBNAKQJgDQBkCYA0gRAmgBIEwBpAiBNAKQJgDQBkCYA0gRAmgBIEwBpAiBNAKQJgDQBkCYA0gRAmgBIEwBpAiBNAKQJgDQBkCYA0gRAmgBIEwBpAiBNAKQJgDQBkCYA0gRAmgBIEwBpAiBNAKQJgDQBkCYA0gRAmgBIEwBpAiBNAKQJgDQBkCYA0gRA2ubVi5erN4xc/3y7esLI/e7p6gkjv569WT1hxAcgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASBMAaQIgTQCkCYA0AZAmANIEQJoASHsIAAD//37rGOCZoockAAAAAElFTkSuQmCC";

// impl CursoredDataProvider for TokensProvider {
//     type Data = TokenMetadata;
//     type Error = AppError;

//     async fn get_by_cursor(
//         &self,
//         start: usize,
//         end: usize,
//     ) -> Result<PageEntry<TokenMetadata>, AppError> {
//         leptos::logging::log!("Fetching tokens from {} to {}", start, end);

//         let mut data = vec![];

//         for i in start..end {
//             data.push(TokenMetadata {
//                 logo_b64: TEST_IMG.to_string(),
//                 name: format!("FLAME{} name", i),
//                 description: format!("inflammable token{}", i),
//                 symbol: format!("FLAME{}", i),
//                 balance: TokenBalance::new(Nat::from(100u8), 8),
//                 fees: TokenBalance::new(Nat::from(100u8), 8),
//             });
//         }

//         Ok(PageEntry { data, end: false })
//     }
//
//
// }

// #[wasm_bindgen(module = "/src/utils/token/icpump-firestore.js")]
// extern "C" {
//     fn get_token_list() -> js_sys::Promise;
// }

pub async fn get_and_print_tokens() {
    // let token_promise = get_token_list();
    // match JsFuture::from(token_promise).await {
    //     Ok(token_js) => {
    //         let token: String = token_js.as_string().unwrap_or_default();
    //     }
    //     Err(err) => {
    //         log::warn!("Failed to get tokens: {:?}", err);
    //         // Err(())
    //     }
    // }
}

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

    const TEST_COLLECTION_NAME: &'static str = "tokens-list"; //"test-tokens-3"

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
