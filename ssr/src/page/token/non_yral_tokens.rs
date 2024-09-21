use std::collections::HashSet;

use candid::Principal;
use leptos::*;
use server_fn::codec::Cbor;

use crate::canister::individual_user_template::Result2;
use crate::state::canisters::CanistersAuthWire;

pub struct NonYralTokensRoot(Vec<Principal>);

impl Default for NonYralTokensRoot {
    fn default() -> Self {
        let supported_tokens_root = vec![
            // yral's Dolr token
            Principal::from_text("67bll-riaaa-aaaaq-aaauq-cai").unwrap(),
        ];
        Self(supported_tokens_root)
    }
}

impl NonYralTokensRoot {
    pub fn filter_unregistered_non_yral_tokens(
        &self,
        registerd_token_root: Vec<Principal>,
    ) -> Vec<Principal> {
        let registerd_token_root_set: HashSet<_> = registerd_token_root.into_iter().collect();
        let unregistered_non_yral_token_root: Vec<Principal> = self
            .0
            .clone()
            .into_iter()
            .filter(|token_root| !registerd_token_root_set.contains(token_root))
            .collect();

        unregistered_non_yral_token_root
    }
}

#[server(input = Cbor)]
pub async fn register_non_yral_token_to_user_canister(
    cans_wire: CanistersAuthWire,
    user_principal: Principal,
    unregistered_non_yral_token_root: Vec<Principal>,
) -> Result<Vec<Principal>, ServerFnError> {
    let canisters = cans_wire.canisters().unwrap();
    let user_canister = canisters
        .get_individual_canister_by_user_principal(user_principal)
        .await?
        .ok_or_else(|| ServerFnError::new("User not found"))?;

    let user = canisters.individual_user(user_canister).await;

    let mut finalised_non_yral_token_root = vec![];
    for token_root_id in unregistered_non_yral_token_root {
        let res = user.add_token(token_root_id).await.unwrap();
        if let Result2::Ok(_) = res {
            finalised_non_yral_token_root.push(token_root_id)
        }
    }

    Ok(finalised_non_yral_token_root)
}
