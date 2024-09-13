use crate::{
    canister::{
        sns_governance::{
            Account, Amount, Command, Command1, Disburse, DissolveState, ManageNeuron, Neuron,
        },
        sns_ledger::{Account as LedgerAccount, TransferArg, TransferResult},
    },
    state::canisters::CanistersAuthWire,
};
use candid::{Decode, Nat, Principal};
use leptos::ServerFnError;

// TODO: XX: this should happen at token creation to prevent races
pub async fn claim_tokens_from_first_neuron(
    cans_wire: CanistersAuthWire,
    governance_principal: Principal,
    ledger_principal: Principal,
    raw_neurons: Vec<u8>,
) -> Result<(), ServerFnError> {
    let cans = cans_wire.canisters()?;
    let user_principal = cans.user_principal();
    log::trace!("!!!!! Claiming tokens from first neuron");
    log::trace!("!!!!! user_principal: {:?}", user_principal);
    log::trace!("!!!!! root: {:?}", governance_principal);

    let governance = cans.sns_governance(governance_principal).await;
    let neurons = Decode!(&raw_neurons, Vec<Neuron>)?;

    if neurons.len() < 2 || neurons[1].cached_neuron_stake_e8s == 0 {
        return Ok(());
    }
    // let neuron = neurons[0];
    let mut ix = 0;
    // if let Some(neuron) = neurons.get(1) {
    //     if let Some(dissolve) = neuron.dissolve_state {
    //         match dissolve {
    //             DissolveState::DissolveDelaySeconds(x) => {
    //                 if x > 0 {
    //                     ix = 1;
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    // }
    if neurons[1].dissolve_state.is_some() {
        if let Some(DissolveState::DissolveDelaySeconds(x)) = neurons[1].dissolve_state.as_ref() {
            if *x == 0 {
                ix = 1;
            }
        }
    }
    let neuron_id = neurons[ix].id.as_ref().unwrap().id.clone();
    let amount = neurons[ix].cached_neuron_stake_e8s;
    let manage_neuron_arg = ManageNeuron {
        subaccount: neuron_id,
        command: Some(Command::Disburse(Disburse {
            to_account: Some(Account {
                owner: Some(user_principal),
                subaccount: None,
            }),
            amount: Some(Amount { e8s: amount }),
        })),
    };
    let manage_neuron = governance.manage_neuron(manage_neuron_arg).await?;
    if !matches!(manage_neuron.command, Some(Command1::Disburse(_))) {
        return Err(ServerFnError::new("Failed to claim tokens"));
    }
    // Transfer to canister
    let user_canister = cans.user_canister();
    let ledger = cans.sns_ledger(ledger_principal).await;
    // User has 50% of the overall amount
    // 20% of this 50% is 10% of the overall amount
    // 10% of the overall amount is reserveed for the canister
    let distribution_amt = Nat::from(amount) * 20u32 / 100u32;
    let transfer_resp = ledger
        .icrc_1_transfer(TransferArg {
            to: LedgerAccount {
                owner: user_canister,
                subaccount: None,
            },
            fee: None,
            memo: None,
            from_subaccount: None,
            amount: distribution_amt,
            created_at_time: None,
        })
        .await;

    match transfer_resp {
        Ok(TransferResult::Err(e)) => {
            log::error!("Token is in invalid state, user_canister: {user_canister}, governance: {governance_principal}, irrecoverable {e:?}");
            return Err(ServerFnError::new("Failed to transfer to user canister"));
        }
        Err(e) => {
            log::error!("Token is in invalid state, user_canister: {user_canister}, governance: {governance_principal}, irrecoverable {e}");
            return Err(ServerFnError::new("Failed to transfer to user canister"));
        }
        _ => (),
    }

    Ok(())
}
