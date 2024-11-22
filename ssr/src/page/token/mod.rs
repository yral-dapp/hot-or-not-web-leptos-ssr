pub mod create;
pub mod create_token_faq;
pub mod info;
pub mod non_yral_tokens;
mod popups;
mod sns_form;
pub mod transfer;
pub mod types;
pub mod swap;

use std::{fmt::Display, str::FromStr};

use candid::Principal;
use ic_agent::export::PrincipalError;
use leptos::Params;
use leptos_router::Params;
use serde::{Deserialize, Serialize};

use crate::{
    state::canisters::Canisters,
    utils::token::{get_ck_metadata, token_metadata_by_root, TokenMetadata},
};

#[derive(Params, PartialEq, Clone)]
struct TokenParams {
    token_root: RootType,
}

#[derive(Params, PartialEq, Clone)]
struct TokenInfoParams {
    token_root: RootType,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Hash, Eq, Debug)]
pub enum RootType {
    BTC { ledger: Principal, index: Principal },
    USDC { ledger: Principal, index: Principal },
    Other(Principal),
}

impl RootType {
    pub async fn get_metadata<const A: bool>(
        &self,
        user_principal: Option<Principal>,
        cans: Canisters<A>,
    ) -> Option<TokenMetadata> {
        match self {
            RootType::BTC { ledger, index } => {
                get_ck_metadata(cans, user_principal, *ledger, *index)
                    .await
                    .ok()?
            }
            RootType::USDC { ledger, index } => {
                get_ck_metadata(cans, user_principal, *ledger, *index)
                    .await
                    .ok()?
            }
            RootType::Other(root) => token_metadata_by_root(&cans, user_principal, *root)
                .await
                .ok()?,
        }
    }
}

impl FromStr for RootType {
    type Err = PrincipalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "btc" => Ok(Self::BTC {
                ledger: Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")?,
                index: Principal::from_text("n5wcd-faaaa-aaaar-qaaea-cai")?,
            }),
            "usdc" => Ok(Self::USDC {
                ledger: Principal::from_text("xevnm-gaaaa-aaaar-qafnq-cai")?,
                index: Principal::from_text("xrs4b-hiaaa-aaaar-qafoa-cai")?,
            }),
            _ => Ok(Self::Other(Principal::from_text(s)?)),
        }
    }
}

impl Display for RootType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BTC { .. } => f.write_str("btc"),
            Self::USDC { .. } => f.write_str("usdc"),
            Self::Other(principal) => f.write_str(&principal.to_text()),
        }
    }
}
