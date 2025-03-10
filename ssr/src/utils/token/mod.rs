pub mod firestore;
pub mod icpump;
pub mod nsfw;

use candid::Principal;
use leptos::ServerFnError;
use serde::{Deserialize, Serialize};

use crate::consts::PUMP_AND_DUMP_WORKER_URL;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeployedCdaoCanisters {
    pub root: Principal,
    pub swap: Principal,
    pub ledger: Principal,
    pub index: Principal,
    pub governance: Principal,
}

impl From<yral_canisters_client::individual_user_template::DeployedCdaoCanisters>
    for DeployedCdaoCanisters
{
    fn from(value: yral_canisters_client::individual_user_template::DeployedCdaoCanisters) -> Self {
        Self {
            root: value.root,
            swap: value.swap,
            ledger: value.ledger,
            index: value.index,
            governance: value.governance,
        }
    }
}

pub async fn claim_cents_airdrop(user_canister: Principal) -> Result<(), ServerFnError> {
    let airdrop_url = PUMP_AND_DUMP_WORKER_URL
        .join(&format!("/airdrop/{}", user_canister))
        .unwrap();
    let res = reqwest::get(airdrop_url).await?;
    if res.status() != 200 {
        let e = res.text().await?;
        return Err(ServerFnError::new(e));
    }

    Ok(())
}
