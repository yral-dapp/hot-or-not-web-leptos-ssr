use candid::{Nat, Principal};
use ic_agent::AgentError;
use leptos::ServerFnError;
use serde::{Deserialize, Serialize};

use crate::{
    canister::{
        sns_governance::GetMetadataArg, sns_ledger::Account, sns_root::ListSnsCanistersArg,
    },
    state::canisters::Canisters,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenMetadata {
    pub logo_b64: String,
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub balance: Nat,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenCans {
    pub governance: Principal,
    pub ledger: Principal,
    pub root: Principal,
}

pub async fn token_metadata_by_root<const A: bool>(
    cans: &Canisters<A>,
    user_canister: Principal,
    token_root: Principal,
) -> Result<Option<TokenMetadata>, ServerFnError> {
    let root = cans.sns_root(token_root).await;
    let sns_cans = root.list_sns_canisters(ListSnsCanistersArg {}).await?;
    let Some(governance) = sns_cans.governance else {
        return Ok(None);
    };
    let Some(ledger) = sns_cans.ledger else {
        return Ok(None);
    };
    let metadata = get_token_metadata(cans, user_canister, governance, ledger).await?;

    Ok(Some(metadata))
}

pub async fn get_token_metadata<const A: bool>(
    cans: &Canisters<A>,
    user_canister: Principal,
    governance: Principal,
    ledger: Principal,
) -> Result<TokenMetadata, AgentError> {
    let governance = cans.sns_governance(governance).await;
    let metadata = governance.get_metadata(GetMetadataArg {}).await?;

    let ledger = cans.sns_ledger(ledger).await;
    let symbol = ledger.icrc_1_symbol().await?;

    let acc = Account {
        owner: user_canister,
        subaccount: None,
    };
    let balance = ledger.icrc_1_balance_of(acc).await?;

    Ok(TokenMetadata {
        logo_b64: metadata.logo.unwrap_or_default(),
        name: metadata.name.unwrap_or_default(),
        description: metadata.description.unwrap_or_default(),
        symbol,
        balance,
    })
}
