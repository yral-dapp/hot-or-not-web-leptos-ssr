#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SnsInitPayload {
    pub transaction_fee_e8s: ::core::option::Option<u64>,

    pub token_name: ::core::option::Option<String>,

    pub token_symbol: ::core::option::Option<String>,

    pub proposal_reject_cost_e8s: ::core::option::Option<u64>,

    pub neuron_minimum_stake_e8s: ::core::option::Option<u64>,

    pub fallback_controller_principal_ids: Vec<String>,

    pub logo: ::core::option::Option<String>,

    pub url: ::core::option::Option<String>,

    pub name: ::core::option::Option<String>,

    pub description: ::core::option::Option<String>,

    pub neuron_minimum_dissolve_delay_to_vote_seconds: ::core::option::Option<u64>,

    pub initial_reward_rate_basis_points: ::core::option::Option<u64>,
    pub final_reward_rate_basis_points: ::core::option::Option<u64>,

    pub reward_rate_transition_duration_seconds: ::core::option::Option<u64>,

    pub max_dissolve_delay_seconds: ::core::option::Option<u64>,

    pub max_neuron_age_seconds_for_age_bonus: ::core::option::Option<u64>,

    pub max_dissolve_delay_bonus_percentage: ::core::option::Option<u64>,

    pub max_age_bonus_percentage: ::core::option::Option<u64>,

    pub initial_voting_period_seconds: ::core::option::Option<u64>,

    pub wait_for_quiet_deadline_increase_seconds: ::core::option::Option<u64>,

    pub confirmation_text: ::core::option::Option<String>,

    pub restricted_countries: ::core::option::Option<crate::pbs::nns_pb::Countries>,

    pub dapp_canisters: ::core::option::Option<DappCanisters>,

    pub min_participants: ::core::option::Option<u64>,

    pub min_icp_e8s: ::core::option::Option<u64>,

    pub max_icp_e8s: ::core::option::Option<u64>,

    pub min_direct_participation_icp_e8s: ::core::option::Option<u64>,

    pub max_direct_participation_icp_e8s: ::core::option::Option<u64>,

    pub min_participant_icp_e8s: ::core::option::Option<u64>,

    pub max_participant_icp_e8s: ::core::option::Option<u64>,

    pub swap_start_timestamp_seconds: ::core::option::Option<u64>,

    pub swap_due_timestamp_seconds: ::core::option::Option<u64>,

    pub neuron_basket_construction_parameters:
        ::core::option::Option<super::sns_swap_pb::NeuronBasketConstructionParameters>,

    pub nns_proposal_id: ::core::option::Option<u64>,

    pub neurons_fund_participation: ::core::option::Option<bool>,

    pub neurons_fund_participants: ::core::option::Option<NeuronsFundParticipants>,

    pub token_logo: ::core::option::Option<String>,

    pub neurons_fund_participation_constraints:
        ::core::option::Option<super::sns_swap_pb::NeuronsFundParticipationConstraints>,

    pub initial_token_distribution:
        ::core::option::Option<sns_init_payload::InitialTokenDistribution>,
}

pub mod sns_init_payload {

    #[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, Debug)]
    pub enum InitialTokenDistribution {
        FractionalDeveloperVotingPower(super::FractionalDeveloperVotingPower),
    }
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct FractionalDeveloperVotingPower {
    pub developer_distribution: ::core::option::Option<DeveloperDistribution>,

    pub treasury_distribution: ::core::option::Option<TreasuryDistribution>,

    pub swap_distribution: ::core::option::Option<SwapDistribution>,

    pub airdrop_distribution: ::core::option::Option<AirdropDistribution>,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct DeveloperDistribution {
    pub developer_neurons: Vec<NeuronDistribution>,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct TreasuryDistribution {
    pub total_e8s: u64,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct SwapDistribution {
    pub total_e8s: u64,

    pub initial_swap_amount_e8s: u64,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default, Debug)]
pub struct AirdropDistribution {
    pub airdrop_neurons: Vec<NeuronDistribution>,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct NeuronDistribution {
    pub controller: ::core::option::Option<candid::Principal>,

    pub stake_e8s: u64,

    pub memo: u64,

    pub dissolve_delay_seconds: u64,

    pub vesting_period_seconds: ::core::option::Option<u64>,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct DappCanisters {
    pub canisters: Vec<crate::pbs::nns_pb::Canister>,
}
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct NeuronsFundParticipants {
    pub participants: Vec<super::sns_swap_pb::CfParticipant>,
}
