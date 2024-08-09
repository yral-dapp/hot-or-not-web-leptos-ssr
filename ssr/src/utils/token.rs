use candid::Principal;
use ic_agent::AgentError;
use serde::{Deserialize, Serialize};

use crate::{canister::sns_governance::GetMetadataArg, state::canisters::Canisters};

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenMetadata {
    pub logo_b64: String,
    pub name: String,
    pub description: String,
    pub symbol: String,
}

pub async fn get_token_metadata(
    cans: &Canisters<false>,
    governance: Principal,
    ledger: Principal,
) -> Result<TokenMetadata, AgentError> {
    let governance = cans.sns_governance(governance).await?;
    let metadata = governance.get_metadata(GetMetadataArg {}).await?;

    let ledger = cans.sns_ledger(ledger).await?;
    let symbol = ledger.icrc_1_symbol().await?;

    Ok(TokenMetadata {
        logo_b64: metadata.logo.unwrap_or_default(),
        name: metadata.name.unwrap_or_default(),
        description: metadata.description.unwrap_or_default(),
        symbol,
    })
}
