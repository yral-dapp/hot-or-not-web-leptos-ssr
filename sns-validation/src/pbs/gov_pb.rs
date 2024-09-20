use std::collections::BTreeMap;

#[derive(candid::CandidType, candid::Deserialize, Eq, std::hash::Hash)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NeuronId {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,
}

#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NervousSystemParameters {
    pub reject_cost_e8s: ::core::option::Option<u64>,

    pub neuron_minimum_stake_e8s: ::core::option::Option<u64>,

    pub transaction_fee_e8s: ::core::option::Option<u64>,

    pub max_proposals_to_keep_per_action: ::core::option::Option<u32>,

    pub initial_voting_period_seconds: ::core::option::Option<u64>,

    pub wait_for_quiet_deadline_increase_seconds: ::core::option::Option<u64>,

    pub default_followees: ::core::option::Option<DefaultFollowees>,

    pub max_number_of_neurons: ::core::option::Option<u64>,

    pub neuron_minimum_dissolve_delay_to_vote_seconds: ::core::option::Option<u64>,

    pub max_followees_per_function: ::core::option::Option<u64>,

    pub max_dissolve_delay_seconds: ::core::option::Option<u64>,

    pub max_neuron_age_for_age_bonus: ::core::option::Option<u64>,

    pub max_number_of_proposals_with_ballots: ::core::option::Option<u64>,

    pub neuron_claimer_permissions: ::core::option::Option<NeuronPermissionList>,

    pub neuron_grantable_permissions: ::core::option::Option<NeuronPermissionList>,

    pub max_number_of_principals_per_neuron: ::core::option::Option<u64>,

    pub voting_rewards_parameters: ::core::option::Option<VotingRewardsParameters>,

    pub max_dissolve_delay_bonus_percentage: ::core::option::Option<u64>,

    pub max_age_bonus_percentage: ::core::option::Option<u64>,

    pub maturity_modulation_disabled: ::core::option::Option<bool>,
}

#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct VotingRewardsParameters {
    pub round_duration_seconds: ::core::option::Option<u64>,

    pub reward_rate_transition_duration_seconds: ::core::option::Option<u64>,

    pub initial_reward_rate_basis_points: ::core::option::Option<u64>,
    pub final_reward_rate_basis_points: ::core::option::Option<u64>,
}

#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default)]
pub struct DefaultFollowees {
    pub followees: BTreeMap<u64, neuron::Followees>,
}

#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default)]
pub struct NeuronPermissionList {
    pub permissions: Vec<i32>,
}

pub mod neuron {

    #[derive(candid::CandidType, candid::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq)]
    pub struct Followees {
        pub followees: Vec<super::NeuronId>,
    }

    #[derive(candid::CandidType, candid::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq)]
    pub enum DissolveState {
        WhenDissolvedTimestampSeconds(u64),

        DissolveDelaySeconds(u64),
    }
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct CreateServiceNervousSystem {
    pub name: ::core::option::Option<String>,
    pub description: ::core::option::Option<String>,
    pub url: ::core::option::Option<String>,
    pub logo: ::core::option::Option<crate::pbs::nns_pb::Image>,
    pub fallback_controller_principal_ids: Vec<candid::Principal>,
    pub dapp_canisters: Vec<crate::pbs::nns_pb::Canister>,
    pub initial_token_distribution: ::core::option::Option<create_sns::InitialTokenDistribution>,
    pub swap_parameters: ::core::option::Option<create_sns::SwapParameters>,
    pub ledger_parameters: ::core::option::Option<create_sns::LedgerParameters>,
    pub governance_parameters: ::core::option::Option<create_sns::GovernanceParameters>,
}

pub mod create_sns {
    #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Default)]
    pub struct InitialTokenDistribution {
        pub developer_distribution:
            ::core::option::Option<initial_token_distribution::DeveloperDistribution>,
        pub treasury_distribution:
            ::core::option::Option<initial_token_distribution::TreasuryDistribution>,
        pub swap_distribution: ::core::option::Option<initial_token_distribution::SwapDistribution>,
    }

    pub mod initial_token_distribution {
        #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, Default)]
        pub struct DeveloperDistribution {
            pub developer_neurons: Vec<developer_distribution::NeuronDistribution>,
        }

        pub mod developer_distribution {
            #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, Default)]
            pub struct NeuronDistribution {
                pub controller: ::core::option::Option<candid::Principal>,
                pub dissolve_delay: ::core::option::Option<crate::pbs::nns_pb::Duration>,
                pub memo: ::core::option::Option<u64>,
                pub stake: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
                pub vesting_period: ::core::option::Option<crate::pbs::nns_pb::Duration>,
            }
        }
        #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, Default)]
        pub struct TreasuryDistribution {
            pub total: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        }
        #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, Default)]
        pub struct SwapDistribution {
            pub total: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        }
    }
    #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Default)]
    pub struct SwapParameters {
        pub minimum_participants: ::core::option::Option<u64>,
        pub minimum_icp: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub maximum_icp: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub minimum_direct_participation_icp: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub maximum_direct_participation_icp: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub minimum_participant_icp: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub maximum_participant_icp: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub neuron_basket_construction_parameters:
            ::core::option::Option<swap_parameters::NeuronBasketConstructionParameters>,
        pub confirmation_text: ::core::option::Option<String>,
        pub restricted_countries: ::core::option::Option<crate::pbs::nns_pb::Countries>,

        pub start_time: ::core::option::Option<crate::pbs::nns_pb::GlobalTimeOfDay>,
        pub duration: ::core::option::Option<crate::pbs::nns_pb::Duration>,

        pub neurons_fund_investment_icp: ::core::option::Option<crate::pbs::nns_pb::Tokens>,

        pub neurons_fund_participation: ::core::option::Option<bool>,
    }

    pub mod swap_parameters {
        #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, Default)]
        pub struct NeuronBasketConstructionParameters {
            pub count: ::core::option::Option<u64>,
            pub dissolve_delay_interval: ::core::option::Option<crate::pbs::nns_pb::Duration>,
        }
    }
    #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Default)]
    pub struct LedgerParameters {
        pub transaction_fee: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub token_name: ::core::option::Option<String>,
        pub token_symbol: ::core::option::Option<String>,
        pub token_logo: ::core::option::Option<crate::pbs::nns_pb::Image>,
    }

    #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Default)]
    pub struct GovernanceParameters {
        pub proposal_rejection_fee: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub proposal_initial_voting_period: ::core::option::Option<crate::pbs::nns_pb::Duration>,
        pub proposal_wait_for_quiet_deadline_increase:
            ::core::option::Option<crate::pbs::nns_pb::Duration>,
        pub neuron_minimum_stake: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        pub neuron_minimum_dissolve_delay_to_vote:
            ::core::option::Option<crate::pbs::nns_pb::Duration>,
        pub neuron_maximum_dissolve_delay: ::core::option::Option<crate::pbs::nns_pb::Duration>,
        pub neuron_maximum_dissolve_delay_bonus:
            ::core::option::Option<crate::pbs::nns_pb::Percentage>,
        pub neuron_maximum_age_for_age_bonus: ::core::option::Option<crate::pbs::nns_pb::Duration>,
        pub neuron_maximum_age_bonus: ::core::option::Option<crate::pbs::nns_pb::Percentage>,
        pub voting_reward_parameters:
            ::core::option::Option<governance_parameters::VotingRewardParameters>,
    }

    pub mod governance_parameters {
        #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, Default)]
        pub struct VotingRewardParameters {
            pub initial_reward_rate: ::core::option::Option<crate::pbs::nns_pb::Percentage>,
            pub final_reward_rate: ::core::option::Option<crate::pbs::nns_pb::Percentage>,
            pub reward_rate_transition_duration:
                ::core::option::Option<crate::pbs::nns_pb::Duration>,
        }
    }
}
