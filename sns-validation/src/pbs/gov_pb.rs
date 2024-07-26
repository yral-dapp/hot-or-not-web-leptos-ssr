use std::collections::BTreeMap;

#[derive(candid::CandidType, candid::Deserialize, Eq, std::hash::Hash)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NeuronId {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,
}

/// The nervous system's parameters, which are parameters that can be changed, via proposals,
/// by each nervous system community.
/// For some of the values there are specified minimum values (floor) or maximum values
/// (ceiling). The motivation for this is a) to prevent that the nervous system accidentally
/// chooses parameters that result in an un-upgradable (and thus stuck) governance canister
/// and b) to prevent the canister from growing too big (which could harm the other canisters
/// on the subnet).
///
/// Required invariant: the canister code assumes that all system parameters are always set.
#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NervousSystemParameters {
    /// The number of e8s (10E-8 of a token) that a rejected
    /// proposal costs the proposer.
    pub reject_cost_e8s: ::core::option::Option<u64>,
    /// The minimum number of e8s (10E-8 of a token) that can be staked in a neuron.
    ///
    /// To ensure that staking and disbursing of the neuron work, the chosen value
    /// must be larger than the transaction_fee_e8s.
    pub neuron_minimum_stake_e8s: ::core::option::Option<u64>,
    /// The transaction fee that must be paid for ledger transactions (except
    /// minting and burning governance tokens).
    pub transaction_fee_e8s: ::core::option::Option<u64>,
    /// The maximum number of proposals to keep, per action. When the
    /// total number of proposals for a given action is greater than this
    /// number, the oldest proposals that have reached final decision state
    /// (rejected, executed, or failed) and final rewards status state
    /// (settled) may be deleted.
    ///
    /// The number must be larger than zero and at most be as large as the
    /// defined ceiling MAX_PROPOSALS_TO_KEEP_PER_ACTION_CEILING.
    pub max_proposals_to_keep_per_action: ::core::option::Option<u32>,
    /// The initial voting period of a newly created proposal.
    /// A proposal's voting period may then be further increased during
    /// a proposal's lifecycle due to the wait-for-quiet algorithm.
    ///
    /// The voting period must be between (inclusive) the defined floor
    /// INITIAL_VOTING_PERIOD_SECONDS_FLOOR and ceiling
    /// INITIAL_VOTING_PERIOD_SECONDS_CEILING.
    pub initial_voting_period_seconds: ::core::option::Option<u64>,
    /// The wait for quiet algorithm extends the voting period of a proposal when
    /// there is a flip in the majority vote during the proposal's voting period.
    /// This parameter determines the maximum time period that the voting period
    /// may be extended after a flip. If there is a flip at the very end of the
    /// original proposal deadline, the remaining time will be set to this parameter.
    /// If there is a flip before or after the original deadline, the deadline will
    /// extended by somewhat less than this parameter.
    /// The maximum total voting period extension is 2 * wait_for_quiet_deadline_increase_seconds.
    /// For more information, see the wiki page on the wait-for-quiet algorithm:
    /// <https://wiki.internetcomputer.org/wiki/Network_Nervous_System#Proposal_decision_and_wait-for-quiet>
    pub wait_for_quiet_deadline_increase_seconds: ::core::option::Option<u64>,
    /// TODO NNS1-2169: This field currently has no effect.
    /// TODO NNS1-2169: Design and implement this feature.
    ///
    /// The set of default followees that every newly created neuron will follow
    /// per function. This is specified as a mapping of proposal functions to followees.
    ///
    /// If unset, neurons will have no followees by default.
    /// The set of followees for each function can be at most of size
    /// max_followees_per_function.
    pub default_followees: ::core::option::Option<DefaultFollowees>,
    /// The maximum number of allowed neurons. When this maximum is reached, no new
    /// neurons will be created until some are removed.
    ///
    /// This number must be larger than zero and at most as large as the defined
    /// ceiling MAX_NUMBER_OF_NEURONS_CEILING.
    pub max_number_of_neurons: ::core::option::Option<u64>,
    /// The minimum dissolve delay a neuron must have to be eligible to vote.
    ///
    /// The chosen value must be smaller than max_dissolve_delay_seconds.
    pub neuron_minimum_dissolve_delay_to_vote_seconds: ::core::option::Option<u64>,
    /// The maximum number of followees each neuron can establish for each nervous system function.
    ///
    /// This number can be at most as large as the defined ceiling
    /// MAX_FOLLOWEES_PER_FUNCTION_CEILING.
    pub max_followees_per_function: ::core::option::Option<u64>,
    /// The maximum dissolve delay that a neuron can have. That is, the maximum
    /// that a neuron's dissolve delay can be increased to. The maximum is also enforced
    /// when saturating the dissolve delay bonus in the voting power computation.
    pub max_dissolve_delay_seconds: ::core::option::Option<u64>,
    /// The age of a neuron that saturates the age bonus for the voting power computation.
    pub max_neuron_age_for_age_bonus: ::core::option::Option<u64>,
    /// The max number of proposals for which ballots are still stored, i.e.,
    /// unsettled proposals. If this number of proposals is reached, new proposals
    /// can only be added in exceptional cases (for few proposals it is defined
    /// that they are allowed even if resources are low to guarantee that the relevant
    /// canisters can be upgraded).
    ///
    /// This number must be larger than zero and at most as large as the defined
    /// ceiling MAX_NUMBER_OF_PROPOSALS_WITH_BALLOTS_CEILING.
    pub max_number_of_proposals_with_ballots: ::core::option::Option<u64>,
    /// The default set of neuron permissions granted to the principal claiming a neuron.
    pub neuron_claimer_permissions: ::core::option::Option<NeuronPermissionList>,
    /// The superset of neuron permissions a principal with permission
    /// `NeuronPermissionType::ManagePrincipals` for a given neuron can grant to another
    /// principal for this same neuron.
    /// If this set changes via a ManageNervousSystemParameters proposal, previous
    /// neurons' permissions will be unchanged and only newly granted permissions will be affected.
    pub neuron_grantable_permissions: ::core::option::Option<NeuronPermissionList>,
    /// The maximum number of principals that can have permissions for a neuron
    pub max_number_of_principals_per_neuron: ::core::option::Option<u64>,
    /// When this field is not populated, voting rewards are "disabled". Once this
    /// is set, it probably should not be changed, because the results would
    /// probably be pretty confusing.
    pub voting_rewards_parameters: ::core::option::Option<VotingRewardsParameters>,
    /// E.g. if a large dissolve delay can double the voting power of a neuron,
    /// then this field would have a value of 100, indicating a maximum of
    /// 100% additional voting power.
    ///
    /// For no bonus, this should be set to 0.
    ///
    /// To achieve functionality equivalent to NNS, this should be set to 100.
    pub max_dissolve_delay_bonus_percentage: ::core::option::Option<u64>,
    /// Analogous to the previous field (see the previous comment),
    /// but this one relates to neuron age instead of dissolve delay.
    ///
    /// To achieve functionality equivalent to NNS, this should be set to 25.
    pub max_age_bonus_percentage: ::core::option::Option<u64>,
    /// By default, maturity modulation is enabled; however, an SNS can use this
    /// field to disable it. When disabled, this canister will still poll the
    /// Cycles Minting Canister (CMC), and store the value received therefrom.
    /// However, the fetched value does not get used when this is set to true.
    ///
    /// The reason we call this "disabled" instead of (positive) "enabled" is so
    /// that the PB default (bool fields are false) and our application default
    /// (enabled) agree.
    pub maturity_modulation_disabled: ::core::option::Option<bool>,
}

#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct VotingRewardsParameters {
    /// The amount of time between reward events.
    ///
    /// Must be > 0.
    ///
    /// During such periods, proposals enter the ReadyToSettle state. Once the
    /// round is over, voting for those proposals entitle voters to voting
    /// rewards. Such rewards are calculated by the governance canister's
    /// heartbeat.
    ///
    /// This is a nominal amount. That is, the actual time between reward
    /// calculations and distribution cannot be guaranteed to be perfectly
    /// periodic, but actual inter-reward periods are generally expected to be
    /// within a few seconds of this.
    ///
    /// This supersedes super.reward_distribution_period_seconds.
    pub round_duration_seconds: ::core::option::Option<u64>,
    /// The amount of time that the growth rate changes (presumably, decreases)
    /// from the initial growth rate to the final growth rate. (See the two
    /// *_reward_rate_basis_points fields bellow.) The transition is quadratic, and
    /// levels out at the end of the growth rate transition period.
    pub reward_rate_transition_duration_seconds: ::core::option::Option<u64>,
    /// The amount of rewards is proportional to token_supply * current_rate. In
    /// turn, current_rate is somewhere between `initial_reward_rate_basis_points`
    /// and `final_reward_rate_basis_points`. In the first reward period, it is the
    /// initial growth rate, and after the growth rate transition period has elapsed,
    /// the growth rate becomes the final growth rate, and remains at that value for
    /// the rest of time. The transition between the initial and final growth rates is
    /// quadratic, and levels out at the end of the growth rate transition period.
    ///
    /// (A basis point is one in ten thousand.)
    pub initial_reward_rate_basis_points: ::core::option::Option<u64>,
    pub final_reward_rate_basis_points: ::core::option::Option<u64>,
}
/// The set of default followees that every newly created neuron will follow per function.
/// This is specified as a mapping of proposal functions to followees for that function.
#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default)]
pub struct DefaultFollowees {
    pub followees: BTreeMap<u64, neuron::Followees>,
}
/// A wrapper for a list of neuron permissions.
#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Default)]
pub struct NeuronPermissionList {
    pub permissions: Vec<i32>,
}
/// Nested message and enum types in `Neuron`.
pub mod neuron {
    /// A list of a neuron's followees for a specific function.
    #[derive(candid::CandidType, candid::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq)]
    pub struct Followees {
        pub followees: Vec<super::NeuronId>,
    }
    /// The neuron's dissolve state, specifying whether the neuron is dissolving,
    /// non-dissolving, or dissolved.
    ///
    /// At any time, at most only one of `when_dissolved_timestamp_seconds` and
    /// `dissolve_delay_seconds` are specified.
    ///
    /// `NotDissolving`. This is represented by `dissolve_delay_seconds` being
    /// set to a non zero value.
    ///
    /// `Dissolving`. This is represented by `when_dissolved_timestamp_seconds` being
    /// set, and this value is in the future.
    ///
    /// `Dissolved`. All other states represent the dissolved
    /// state. That is, (a) `when_dissolved_timestamp_seconds` is set and in the past,
    /// (b) `when_dissolved_timestamp_seconds` is set to zero, (c) neither value is set.
    #[derive(candid::CandidType, candid::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq)]
    pub enum DissolveState {
        /// When the dissolve timer is running, this stores the timestamp,
        /// in seconds from the Unix epoch, at which the neuron is dissolved.
        ///
        /// At any time while the neuron is dissolving, the neuron owner
        /// may pause dissolving, in which case `dissolve_delay_seconds`
        /// will get assigned to: `when_dissolved_timestamp_seconds -
        /// <timestamp when the action is taken>`.
        WhenDissolvedTimestampSeconds(u64),
        /// When the dissolve timer is stopped, this stores how much time,
        /// in seconds, the dissolve timer will be started with if the neuron is set back to 'Dissolving'.
        ///
        /// At any time while in this state, the neuron owner may (re)start
        /// dissolving, in which case `when_dissolved_timestamp_seconds`
        /// will get assigned to: `<timestamp when the action is taken> +
        /// dissolve_delay_seconds`.
        DissolveDelaySeconds(u64),
    }
}

/// Mainly, calls the deploy_new_sns Candid method on the SNS-WASMs canister.
/// Therefore, most of the fields here have equivalents in SnsInitPayload.
/// Please, consult the comments therein.
///
/// Metadata
/// --------
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
    /// Nested message and enum types in `InitialTokenDistribution`.
    pub mod initial_token_distribution {
        #[derive(candid::CandidType, candid::Deserialize, serde::Serialize)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, Default)]
        pub struct DeveloperDistribution {
            pub developer_neurons: Vec<developer_distribution::NeuronDistribution>,
        }
        /// Nested message and enum types in `DeveloperDistribution`.
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
        /// The swap occurs at a specific time of day, in UTC.
        /// It will happen the first time start_time occurs that's at least 24h after
        /// the proposal is adopted.
        pub start_time: ::core::option::Option<crate::pbs::nns_pb::GlobalTimeOfDay>,
        pub duration: ::core::option::Option<crate::pbs::nns_pb::Duration>,
        /// The amount that the Neuron's Fund will collectively spend in maturity on
        /// the swap.
        pub neurons_fund_investment_icp: ::core::option::Option<crate::pbs::nns_pb::Tokens>,
        /// Whether Neurons' Fund participation is requested.
        /// Cannot be set to true until Matched Funding is released
        pub neurons_fund_participation: ::core::option::Option<bool>,
    }
    /// Nested message and enum types in `SwapParameters`.
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
    /// Proposal Parameters
    /// -------------------
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
    /// Nested message and enum types in `GovernanceParameters`.
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
