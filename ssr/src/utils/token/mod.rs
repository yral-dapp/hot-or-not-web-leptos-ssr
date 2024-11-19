pub mod firestore;
pub mod icpump;
pub mod nsfw;

use candid::Principal;
use serde::{Deserialize, Serialize};

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
