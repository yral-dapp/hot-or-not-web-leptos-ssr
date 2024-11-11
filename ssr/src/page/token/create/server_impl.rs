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
mod local_claim {
    use web_time::Duration;

    use candid::{Nat, Principal};
    use ic_agent::{identity::DelegatedIdentity, Identity};
    use leptos::ServerFnError;
    use yral_canisters_client::{
        sns_governance::{
            Account, Amount, Command, Command1, Disburse, DissolveState, ListNeurons, ManageNeuron,
            Neuron, SnsGovernance,
        },
        sns_ledger::{Account as LedgerAccount, SnsLedger, TransferArg, TransferResult},
        sns_root::{ListSnsCanistersArg, SnsRoot},
    };
    use yral_qstash_types::ClaimTokensRequest;

    use crate::{consts::CDAO_SWAP_TIME_SECS, utils::ic::AgentWrapper};

    async fn get_neurons(
        governance: &SnsGovernance<'_>,
        user_principal: Principal,
    ) -> Result<Vec<Neuron>, ServerFnError> {
        let neurons = governance
            .list_neurons(ListNeurons {
                of_principal: Some(user_principal),
                limit: 10,
                start_page_at: None,
            })
            .await?;

        Ok(neurons.neurons)
    }

    async fn claim_tokens(req: ClaimTokensRequest) -> Result<(), ServerFnError> {
        let identity: DelegatedIdentity = req.identity.try_into()?;
        let user_principal = identity
            .sender()
            .expect("Delegated identity without principal?!");

        let agent_w = AgentWrapper::build(|b| b.with_identity(identity));
        let agent = agent_w.get_agent().await;

        let root_canister = SnsRoot(req.token_root, agent);
        let token_cans = root_canister
            .list_sns_canisters(ListSnsCanistersArg {})
            .await?;
        let Some(governance) = token_cans.governance else {
            log::warn!("No governance canister found for token. Ignoring...");
            return Ok(());
        };
        let Some(ledger) = token_cans.ledger else {
            log::warn!("No ledger canister found for token. Ignoring...");
            return Ok(());
        };

        let governance_can = SnsGovernance(governance, agent);

        let neurons = get_neurons(&governance_can, user_principal).await?;
        if neurons.len() < 2 || neurons[1].cached_neuron_stake_e8s == 0 {
            return Ok(());
        }
        let ix = if matches!(
            neurons[1].dissolve_state.as_ref(),
            Some(DissolveState::DissolveDelaySeconds(0))
        ) {
            1
        } else {
            0
        };

        let amount = neurons[ix].cached_neuron_stake_e8s;
        let neuron_id = &neurons[ix]
            .id
            .as_ref()
            .ok_or_else(|| ServerFnError::new("unable to get neuron id"))?
            .id;

        let mut tries = 0;
        loop {
            if tries > 10 {
                return Err(ServerFnError::new(
                    "failed to claim tokens after more than 10 tries",
                ));
            }
            tries += 1;

            let manage_neuron_arg = ManageNeuron {
                subaccount: neuron_id.clone(),
                command: Some(Command::Disburse(Disburse {
                    to_account: Some(Account {
                        owner: Some(user_principal),
                        subaccount: None,
                    }),
                    amount: Some(Amount { e8s: amount }),
                })),
            };
            let manage_neuron = governance_can.manage_neuron(manage_neuron_arg).await?;
            match manage_neuron.command {
                Some(Command1::Disburse(_)) => break,
                Some(Command1::Error(e)) => {
                    if e.error_message.contains("PreInitializationSwap") {
                        log::info!("Governance {governance} is not ready. Retrying...");
                        tokio::time::sleep(Duration::from_secs(8)).await;
                        continue;
                    }
                    return Err(ServerFnError::new(format!("{e:?}")));
                }
                command => return Err(ServerFnError::new(format!("unable to claim: {command:?}"))),
            }
        }

        // Transfer to canister
        let user_canister = req.user_canister;
        let ledger_can = SnsLedger(ledger, agent);
        // User has 50% of the overall amount
        // 20% of this 50% is 10% of the overall amount
        // 10% of the overall amount is reserveed for the canister
        let distribution_amt = Nat::from(amount) * 20u32 / 100u32;
        let transfer_resp = ledger_can
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
                log::error!("Token is in invalid state, user_canister: {user_canister}, governance: {governance}, irrecoverable {e:?}");
                return Err(ServerFnError::new("Token is in invalid state"));
            }
            Err(e) => {
                log::error!("Token is in invalid state, user_canister: {user_canister}, governance: {governance}, irrecoverable {e}");
                return Err(ServerFnError::new("Token is in invalid state"));
            }
            _ => (),
        }

        Ok(())
    }

    pub async fn enqueue_claim_token(req: ClaimTokensRequest) -> Result<(), ServerFnError> {
        tokio::spawn(async move {
            log::info!("started claiming job");
            tokio::time::sleep(Duration::from_secs(CDAO_SWAP_TIME_SECS)).await;
            if let Err(e) = claim_tokens(req).await {
                log::error!("claim job failed: {e:?}");
            }
            log::info!("claiming completed")
        });

        Ok(())
    }
}

#[cfg(feature = "backend-admin")]
mod real_impl {
    use std::str::FromStr;

    use crate::auth::delegate_short_lived_identity;
    use crate::page::token::create::DeployedCdaoCanistersRes;
    use crate::utils::token::nsfw::NSFWInfo;
    use yral_canisters_client::individual_user_template::Result7;
    use yral_canisters_client::sns_swap::{
        NewSaleTicketRequest, RefreshBuyerTokensRequest, Result2,
    };

    use crate::consts::ICP_LEDGER_CANISTER_ID;
    use crate::utils::token::nsfw;
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
    use super::local_claim::enqueue_claim_token;
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
    ) -> Result<DeployedCdaoCanistersRes, ServerFnError> {
        // NSFW check
        let mut nsfw_info = NSFWInfo::default();
        if create_sns.token_logo.is_some() {
            nsfw_info = nsfw::get_nsfw_info(create_sns.token_logo.clone().unwrap())
            .await
            .map_err(|e| ServerFnError::new(format!("failed to get nsfw info {e:?}")))?;
        }

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

        Ok(DeployedCdaoCanistersRes {
            deploy_cdao_canisters: deployed_cans.into(),
            token_nsfw_info: nsfw_info,
        })
    }
}

#[cfg(not(feature = "backend-admin"))]
mod no_op_impl {
    use crate::page::token::create::DeployedCdaoCanistersRes;
    use crate::state::canisters::CanistersAuthWire;
    use crate::utils::token::nsfw::NSFWInfo;
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
    ) -> Result<DeployedCdaoCanistersRes, ServerFnError> {
        Ok(DeployedCdaoCanistersRes {
            deploy_cdao_canisters: DeployedCdaoCanisters {
                governance: Principal::anonymous(),
                swap: Principal::anonymous(),
                root: Principal::anonymous(),
                ledger: Principal::anonymous(),
                index: Principal::anonymous(),
            },
            token_nsfw_info: NSFWInfo::default(),
        })
    }
}
