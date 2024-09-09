use std::env;

use candid::{Decode, Encode, Principal, CandidType, Deserialize};
use ic_agent::Identity;
use ic_base_types::PrincipalId;
use leptos::{server, ServerFnError};
use ic_agent::{identity::BasicIdentity, Agent};
use icp_ledger::{AccountIdentifier, Memo, Subaccount, TimeStamp, Tokens as ledgerTokens, TransferArgs};

use crate::consts::{AGENT_URL, ICP_LEDGER_CANISTER_ID};
use crate::canister::sns_swap::{NewSaleTicketRequest, NewSaleTicketResponse, RefreshBuyerTokensRequest, RefreshBuyerTokensResponse};

#[derive(CandidType, Deserialize, Debug)]
pub struct Transaction {
    pub to: Recipient,
    pub fee: Option<Nat>,
    pub memo: Option<Vec<u8>>,
    pub from_subaccount: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Recipient {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransferResult {
    Ok(Nat),
    Err(CustomTransferError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum CustomTransferError {
    GenericError { message: String, error_code: Nat },
    TemporarilyUnavailable,
    BadBurn { min_burn_amount: Nat },
    Duplicate { duplicate_of: Nat },
    BadFee { expected_fee: Nat },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    InsufficientFunds { balance: Nat },
}

#[server]
async fn participate_in_swap(swap_canister: Principal) -> Result<(), ServerFnError> {
    let admin_id_pem: String =
        env::var("BACKEND_ADMIN_IDENTITY").expect("`BACKEND_ADMIN_IDENTITY` is required!");
    let admin_id_pem_by = admin_id_pem.as_bytes();
    let admin_id =
        BasicIdentity::from_pem(admin_id_pem_by).expect("Invalid `BACKEND_ADMIN_IDENTITY`");
    let admin_principal = admin_id.sender().unwrap();

    let agent = Agent::builder()
        .with_url(AGENT_URL)
        .with_identity(admin_id)
        .build()
        .unwrap();
    agent.fetch_root_key().await.unwrap();

    // new_sale_ticket
    let new_sale_ticket_request = NewSaleTicketRequest {
        amount_icp_e8s: 100_000,
        subaccount: None,
    };
    let res = agent
        .update(&swap_canister, "new_sale_ticket")
        .with_arg(Encode!(&new_sale_ticket_request).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    let new_sale_ticket_response: NewSaleTicketResponse = Decode!(&res, NewSaleTicketResponse).unwrap();
    println!("new_sale_ticket_response: {:?}", new_sale_ticket_response);

    // transfer icp
    let subaccount = Subaccount::from(&PrincipalId(admin_principal));
    let transfer_args = Transaction {
        memo: Some(vec![0]),
        amount: Nat::from(1000000 as u64),
        fee: Some(Nat::from(0 as u64)),
        from_subaccount: None,
        to: Recipient {
            owner: swap_canister,
            subaccount: Some(subaccount.to_vec()),
        },
        created_at_time: None,
    };
    let res = agent
        .update(&ICP_LEDGER_CANISTER_ID, "icrc1_transfer")
        .with_arg(Encode!(&transfer_args).unwrap())
        .call_and_wait()
        .await
        .unwrap();
    let transfer_result: TransferResult = Decode!(&res, TransferResult).unwrap();
    println!("transfer_result: {:?}", transfer_result);

    // refresh_buyer_tokens
    let refresh_buyer_tokens_request = RefreshBuyerTokensRequest {
        buyer: admin_principal,
        confirmation_text: None
    };
    let res = agent
        .update(&swap_canister, "refresh_buyer_tokens")
        .call_and_wait()
        .await
        .unwrap();
    let refresh_buyer_tokens_response: RefreshBuyerTokensResponse = Decode!(&res, RefreshBuyerTokensResponse).unwrap();
    println!("refresh_buyer_tokens_response: {:?}", refresh_buyer_tokens_response);

    Ok(())
}