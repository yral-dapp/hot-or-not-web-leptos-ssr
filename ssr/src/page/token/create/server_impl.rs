#[cfg(not(feature = "backend-admin"))]
pub use no_op_impl::{deploy_cdao_canisters, is_server_available};
#[cfg(feature = "backend-admin")]
pub use real_impl::{deploy_cdao_canisters, is_server_available};

#[cfg(all(feature = "backend-admin", feature = "qstash"))]
mod qstash_claim {
    use leptos::{expect_context, ServerFnError};
    use yral_qstash_types::ClaimTokensRequest;

    pub async fn enqueue_claim_token(req: ClaimTokensRequest) -> Result<(), ServerFnError> {
        use crate::utils::qstash::QStashClient;
        let client: QStashClient = expect_context();
        client.enqueue_claim_token(req).await?;

        Ok(())
    }
}

#[cfg(all(feature = "backend-admin", not(feature = "qstash")))]
mod no_op_claim {
    use yral_qstash_types::ClaimTokensRequest;

    pub async fn enqueue_claim_token(_req: ClaimTokensRequest) -> Result<(), ServerFnError> {
        Ok(())
    }
}

#[cfg(feature = "backend-admin")]
mod real_impl {
    use std::str::FromStr;

    use crate::auth::delegate_short_lived_identity;
    use crate::canister::individual_user_template::Result7;
    use crate::canister::sns_swap::{NewSaleTicketRequest, RefreshBuyerTokensRequest};
    use crate::consts::ICP_LEDGER_CANISTER_ID;
    use crate::utils::token::DeployedCdaoCanisters;
    use candid::{Decode, Encode, Nat, Principal};
    use ic_base_types::PrincipalId;
    use icp_ledger::{AccountIdentifier, Subaccount};
    use leptos::ServerFnError;
    use sns_validation::pbs::sns_pb::SnsInitPayload;
    use yral_qstash_types::ClaimTokensRequest;

    use crate::page::token::types::{Icrc1BalanceOfArg, Recipient, Transaction, TransferResult};
    use crate::state::admin_canisters::admin_canisters;
    use crate::state::canisters::CanistersAuthWire;

    #[cfg(all(feature = "backend-admin", not(feature = "qstash")))]
    use super::no_op_claim::enqueue_claim_token;
    #[cfg(all(feature = "backend-admin", feature = "qstash"))]
    use super::qstash_claim::enqueue_claim_token;

    const ICP_TX_FEE: u64 = 10000;

    pub async fn is_server_available() -> Result<(bool, AccountIdentifier), ServerFnError> {
        let admin_cans = admin_canisters();
        let admin_principal = admin_cans.principal();
        let agent = admin_cans.get_agent().await;

        let balance_res: Vec<u8> = agent
            .query(
                &Principal::from_str(ICP_LEDGER_CANISTER_ID).unwrap(),
                "icrc1_balance_of",
            )
            .with_arg(
                candid::encode_one(Icrc1BalanceOfArg {
                    owner: admin_principal,
                    subaccount: None,
                })
                .unwrap(),
            )
            .call()
            .await?;
        let balance: Nat = Decode!(&balance_res, Nat).unwrap();
        let acc_id = AccountIdentifier::new(PrincipalId(admin_principal), None);
        if balance >= (1000000 + ICP_TX_FEE) {
            // amount we participate + icp tx fee
            Ok((true, acc_id))
        } else {
            Ok((false, acc_id))
        }
    }

    async fn participate_in_swap(swap_canister: Principal) -> Result<(), ServerFnError> {
        use crate::canister::sns_swap::Result2;

        let admin_cans = admin_canisters();
        let admin_principal = admin_cans.principal();
        let agent = admin_cans.get_agent().await;

        let swap = admin_cans.sns_swap(swap_canister).await;

        let new_sale_ticket = swap
            .new_sale_ticket(NewSaleTicketRequest {
                amount_icp_e8s: 100_000,
                subaccount: None,
            })
            .await?;
        match new_sale_ticket.result {
            Some(Result2::Ok(_)) => (),
            None | Some(Result2::Err(_)) => {
                return Err(ServerFnError::new("failed to perform swap new_sale_ticket"))
            }
        };

        // transfer icp
        let subaccount = Subaccount::from(&PrincipalId(admin_principal));
        let transfer_args = Transaction {
            memo: Some(vec![0]),
            amount: Nat::from(1000000_u64),
            fee: None,
            from_subaccount: None,
            to: Recipient {
                owner: swap_canister,
                subaccount: Some(subaccount.to_vec()),
            },
            created_at_time: None,
        };
        let res: Vec<u8> = agent
            .update(
                &Principal::from_str(ICP_LEDGER_CANISTER_ID).unwrap(),
                "icrc1_transfer",
            )
            .with_arg(Encode!(&transfer_args).unwrap())
            .call_and_wait()
            .await?;
        let transfer_result: TransferResult = Decode!(&res, TransferResult).unwrap();
        if let TransferResult::Err(e) = transfer_result {
            return Err(ServerFnError::new(format!(
                "failed to perform swap icrc1_transfer {e:?}"
            )));
        }

        swap.refresh_buyer_tokens(RefreshBuyerTokensRequest {
            buyer: admin_principal.to_string(),
            confirmation_text: None,
        })
        .await?;

        Ok(())
    }

    pub async fn deploy_cdao_canisters(
        cans_wire: CanistersAuthWire,
        create_sns: SnsInitPayload,
    ) -> Result<DeployedCdaoCanisters, ServerFnError> {
        let cans = cans_wire.canisters().unwrap();
        log::debug!("deploying canisters {:?}", cans.user_canister().to_string());
        let res = cans
            .deploy_cdao_sns(create_sns)
            .await
            .map_err(|e| ServerFnError::new(format!("{e:?}")))?;

        let deployed_cans = match res {
            Result7::Ok(c) => {
                log::debug!("deployed canister {}", c.governance);
                c
            }
            Result7::Err(e) => return Err(ServerFnError::new(format!("{e:?}"))),
        };

        participate_in_swap(deployed_cans.swap).await?;

        let temp_id = delegate_short_lived_identity(cans.identity());
        let claim_req = ClaimTokensRequest {
            identity: temp_id,
            user_canister: cans.user_canister(),
            token_root: deployed_cans.root,
        };
        enqueue_claim_token(claim_req).await?;

        Ok(deployed_cans.into())
    }
}

#[cfg(not(feature = "backend-admin"))]
mod no_op_impl {
    use crate::state::canisters::CanistersAuthWire;
    use crate::utils::token::DeployedCdaoCanisters;
    use candid::Principal;
    use ic_base_types::PrincipalId;
    use icp_ledger::AccountIdentifier;
    use leptos::ServerFnError;
    use sns_validation::pbs::sns_pb::SnsInitPayload;

    pub async fn is_server_available() -> Result<(bool, AccountIdentifier), ServerFnError> {
        Ok((
            false,
            AccountIdentifier::new(PrincipalId::from(Principal::anonymous()), None),
        ))
    }

    pub async fn deploy_cdao_canisters(
        _cans_wire: CanistersAuthWire,
        _create_sns: SnsInitPayload,
    ) -> Result<DeployedCdaoCanisters, ServerFnError> {
        Ok(DeployedCdaoCanisters {
            governance: Principal::anonymous(),
            swap: Principal::anonymous(),
            root: Principal::anonymous(),
            ledger: Principal::anonymous(),
            index: Principal::anonymous(),
        })
    }
}
