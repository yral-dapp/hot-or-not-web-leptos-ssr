use std::time::UNIX_EPOCH;
use leptos::view;
use candid::{Nat, Principal};
use leptos::{component, IntoView, ServerFnError, SignalWith};
use leptos_router::use_params;
use serde::Deserialize;
use yral_canisters_client::sns_ledger::{Account, AllowanceArgs, ApproveArgs, SnsLedger};
use crate::page::token::info::TokenKeyParam;
use web_time::SystemTime;

use crate::state::canisters::authenticated_canisters;
#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct SwapTokenData{
    pub ledger: Principal,
    pub amt: Nat
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct TokenPairs{
    pub token_a: SwapTokenData,
    pub token_b: SwapTokenData
}

#[component]
pub fn SwapRequest(token_pairs: TokenPairs) -> impl IntoView{

    let key_principal = use_params::<TokenKeyParam>();
    let key_principal = move || key_principal.with(|p| p.as_ref().map(|p| p.key_principal).ok());

    let resource = authenticated_canisters().derive(move || (token_pairs.clone(), key_principal()), move |cans_wire, (token_pairs, key_principal)| async move {
        let Some(key_principal) = key_principal else {return Err(ServerFnError::new("Requestee Principal not provided"))};
        let TokenPairs { token_a: SwapTokenData { ledger: ledger_a, amt: amt_a}, token_b: SwapTokenData { ledger: ledger_b, amt: amt_b } } = token_pairs;
        
        let cans_wire = cans_wire?.canisters()?;

        let ledger_a = cans_wire.sns_ledger(ledger_a).await;
        let ledger_b = cans_wire.sns_ledger(ledger_b).await;
        if !is_icrc2_supported_token(&ledger_a).await{
            return Err(ServerFnError::new("Token A ICRC 2 not supported"))
        }
        
        if !is_icrc2_supported_token(&ledger_b).await{
            return Err(ServerFnError::new("Token B ICRC 2 not supported"))
        }

        let Some(spender_canister_id) = cans_wire.get_individual_canister_by_user_principal(key_principal).await? else {
            return Err(ServerFnError::new("Canister Not Found for the Principal"))
        };
        
        let previous_approval_amt = ledger_a.icrc_2_allowance(AllowanceArgs{
            account: Account{
                owner: cans_wire.user_principal(),
                subaccount: None
            },
            spender: Account{
                owner: spender_canister_id,
                subaccount: None
            }
        }).await?.allowance;

        let _ = ledger_a.icrc_2_approve(ApproveArgs{
            from_subaccount: None,
            spender: Account{
                owner: spender_canister_id,
                subaccount: None
            },
            amount: previous_approval_amt + amt_a,
            fee: None,
            memo: None,
            created_at_time: None,
            expected_allowance: None,
            expires_at: Some(web_time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64 + SWAP_REQUEST_EXPIRY)// fingers crossed that it doesnt go out of bounds lol
        }).await?;

        Ok(())
    });
    view!{

    }
}


const SWAP_REQUEST_EXPIRY: u64 = 7 * 24 * 60 * 60 * 1_000_000_000; // 1 wk

async fn is_icrc2_supported_token(ledger: &SnsLedger<'_>) -> bool{
    ledger.icrc_1_supported_standards().await.map(|l| l.into_iter().any(|rec| rec.name == "ICRC-2")).unwrap_or(false)
}

#[component]
pub fn SwapRequestCancel(token_pairs: TokenPairs, key_principal: Principal, requested_time: SystemTime) -> impl IntoView{
    let resource = authenticated_canisters().derive(move ||(token_pairs.clone(), key_principal.clone(), requested_time.clone()), move |cans_wire, (token_pairs, key_principal, requested_time)| async move {
        let TokenPairs { token_a: SwapTokenData { ledger: ledger_a, amt: amt_a}, token_b: SwapTokenData { ledger: ledger_b, amt: amt_b } } = token_pairs;
        
        let cans_wire = cans_wire?.canisters()?;

        let ledger_a = cans_wire.sns_ledger(ledger_a).await;
        let ledger_b = cans_wire.sns_ledger(ledger_b).await;

        let Some(spender_canister_id) = cans_wire.get_individual_canister_by_user_principal(key_principal).await? else {
            return Err(ServerFnError::new("Canister Not Found for the Principal"))
        };
        
        let previous_approval_amt = ledger_a.icrc_2_allowance(AllowanceArgs{
            account: Account{
                owner: cans_wire.user_principal(),
                subaccount: None
            },
            spender: Account{
                owner: spender_canister_id,
                subaccount: None
            }
        }).await?.allowance;

        let _ = ledger_a.icrc_2_approve(ApproveArgs{
            from_subaccount: None,
            spender: Account{
                owner: spender_canister_id,
                subaccount: None
            },
            amount: previous_approval_amt - amt_a,
            fee: None,
            memo: None,
            created_at_time: None,
            expected_allowance: None,
            expires_at: None // fingers crossed that it doesnt go out of bounds lol
        }).await?; // resets the expiry for all for some reason might need to store the previous expiry in the canister and then deduct that from this

        Ok(())
    });

    view! {

    }
} 