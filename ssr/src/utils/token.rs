use candid::{Nat, Principal};
use ic_agent::AgentError;
use leptos::ServerFnError;
use serde::{Deserialize, Serialize};

use crate::{
    canister::{
        sns_governance::{Account, Amount, Command, Command1, Disburse, GetMetadataArg, ListNeurons, ManageNeuron, Neuron}, sns_ledger::Account as LedgerAccount, sns_root::ListSnsCanistersArg,
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
    pub fees: Nat,
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

    let acc = LedgerAccount {
        owner: user_canister,
        subaccount: None,
    };
    let balance = ledger.icrc_1_balance_of(acc).await?;
    let fees = ledger.icrc_1_fee().await?;

    Ok(TokenMetadata {
        logo_b64: metadata.logo.unwrap_or_default(),
        name: metadata.name.unwrap_or_default(),
        description: metadata.description.unwrap_or_default(),
        symbol,
        fees,
        balance,
    })
}

pub async fn get_neurons<const A: bool>(
    cans: &Canisters<A>,
    user_principal: Principal,
    governance: Principal,
) -> Option<Vec<Neuron>> {
    let governance = cans.sns_governance(governance).await;
    let neurons = governance.list_neurons(ListNeurons {
        of_principal: Some(user_principal),
        limit: 10,
        start_page_at: None
    }).await;

    if neurons.is_ok() {
        let neurons = neurons.unwrap().neurons;
        Some(neurons)
    } else {
        None
    }
}

pub async fn claim_tokens_from_first_neuron(
    cans: &Canisters<true>,
    user_principal: Principal,
    governance: Principal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // if !A {
        //     println!("!!!!! Not authenticated");
        //     return Err("Not authenticaled".into());
        // }
        let governance_can = cans.sns_governance(governance).await;

        let neurons = get_neurons(cans, user_principal, governance).await.unwrap();
        if neurons.len() == 0 || neurons[0].cached_neuron_stake_e8s == 0 {
            return Ok(());
        }
        // let neuron = neurons[0];
        let neuron_id = neurons[0].id.as_ref().unwrap().id.clone();
        let amount = neurons[0].cached_neuron_stake_e8s;
        let manage_neuron_arg = ManageNeuron {
            subaccount: neuron_id,
            command: Some(
                Command::Disburse(Disburse {
                    to_account: Some(Account {
                        owner: Some(user_principal),
                        subaccount: None
                    }),
                    amount: Some(Amount { e8s: amount })
                })
            )
        };
        let manage_neuron = governance_can.manage_neuron(manage_neuron_arg).await;
        if manage_neuron.is_ok() {
            let manage_neuron_res = manage_neuron.unwrap().command.unwrap();
            println!("!!!!! manage_neuron_res: {:?}", manage_neuron_res);
            match manage_neuron_res {
                Command1::Disburse(_) => {
                    return Ok(());
                }
                _ => {
                    println!("!!!!! Failed to claim tokens");
                    return Err("Failed to claim tokens".into());
                }
            }
        } else {
            return Err("Failed to claim tokens".into());
        }
    }