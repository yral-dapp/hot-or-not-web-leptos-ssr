#[derive(
    candid::CandidType, candid::Deserialize, serde::Serialize, Debug, Eq, Clone, PartialEq,
)]
pub struct SnsInitPayload {
    /// Fee of a transaction.
    pub transaction_fee_e8s: ::core::option::Option<u64>,
    /// The name of the token issued by an SNS Ledger.
    /// This field has no default, a value must be provided by the user.
    /// Must be a string length between {} and {} characters
    ///
    /// Example: Bitcoin
    pub token_name: ::core::option::Option<String>,
    /// The symbol of the token issued by an SNS Ledger. This field has no
    /// default, a value must be provided by the user. Must be a string length
    /// between 3 and 10 characters
    pub token_symbol: ::core::option::Option<String>,
    /// Cost of making a proposal that doesnt pass.
    pub proposal_reject_cost_e8s: ::core::option::Option<u64>,
    /// The minimum amount of SNS Token e8s an SNS Ledger account must have to stake a neuron.
    pub neuron_minimum_stake_e8s: ::core::option::Option<u64>,
    /// If the swap fails, control of the dapp canister(s) will be set to these
    /// principal IDs. In most use-cases, this would be the same as the original
    /// set of controller(s). Must not be empty.
    pub fallback_controller_principal_ids: Vec<String>,
    /// The logo for the SNS project represented as a base64 encoded string.
    pub logo: ::core::option::Option<String>,
    /// Url to the dapp controlled by the SNS project.
    pub url: ::core::option::Option<String>,
    /// Name of the SNS project. This may differ from the name of the associated token.
    pub name: ::core::option::Option<String>,
    /// Description of the SNS project.
    pub description: ::core::option::Option<String>,
    /// The minimum dissolve_delay in seconds a neuron must have to be able to cast votes on proposals.
    pub neuron_minimum_dissolve_delay_to_vote_seconds: ::core::option::Option<u64>,
    /// The amount of rewards is proportional to token_supply * current_rate. In
    /// turn, current_rate is somewhere between these two values. In the first
    /// reward period, it is the initial growth rate, and after the growth rate
    /// transition period has elapsed, the growth rate becomes the final growth
    /// rate, and remains at that value for the rest of time. The transition
    /// between the initial and final growth rates is quadratic, and levels out at
    /// the end of the growth rate transition period.
    ///
    /// (A basis point is one in ten thousand.)
    pub initial_reward_rate_basis_points: ::core::option::Option<u64>,

    pub final_reward_rate_basis_points: ::core::option::Option<u64>,
    /// The amount of time that the growth rate changes (presumably, decreases)
    /// from the initial growth rate to the final growth rate. (See the two
    /// *_reward_rate_basis_points fields bellow.) The transition is quadratic, and
    /// levels out at the end of the growth rate transition period.
    pub reward_rate_transition_duration_seconds: ::core::option::Option<u64>,
    /// The maximum dissolve delay that a neuron can have. That is, the maximum
    /// that a neuron's dissolve delay can be increased to. The maximum is also enforced
    /// when saturating the dissolve delay bonus in the voting power computation.
    pub max_dissolve_delay_seconds: ::core::option::Option<u64>,
    /// The age of a neuron that saturates the age bonus for the voting power computation.
    pub max_neuron_age_seconds_for_age_bonus: ::core::option::Option<u64>,
    /// E.g. if a large dissolve delay can double the voting power of a neuron,
    /// then this field would have a value of 2.0.
    ///
    /// For no bonus, this should be set to 1.
    ///
    /// To achieve functionality equivalent to NNS, this should be set to 2.
    pub max_dissolve_delay_bonus_percentage: ::core::option::Option<u64>,
    /// Analogous to the previous field (see the previous comment),
    /// but this one relates to neuron age instead of dissolve delay.
    ///
    /// To achieve functionality equivalent to NNS, this should be set to 1.25.
    pub max_age_bonus_percentage: ::core::option::Option<u64>,
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
    /// An optional text that swap participants should confirm before they may
    /// participate in the swap. If the field is set, its value should be plain text
    /// with at least 1 and at most 1,000 characters.
    pub confirmation_text: ::core::option::Option<String>,
    /// An optional set of countries that should not participate in the swap.
    pub restricted_countries: ::core::option::Option<super::nns_pb::Countries>,
    /// / Canisters that will be transferred to an SNS.
    pub dapp_canisters: ::core::option::Option<DappCanisters>,
    /// The minimum number of buyers that must participate for the swap
    /// to take place. Must be greater than zero.
    pub min_participants: ::core::option::Option<u64>,
    /// The total number of ICP that is required for this token swap to
    /// take place. This number divided by the number of SNS tokens being
    /// offered gives the seller's reserve price for the swap, i.e., the
    /// minimum number of ICP per SNS tokens that the seller of SNS
    /// tokens is willing to accept. If this amount is not achieved, the
    /// swap will be aborted (instead of committed) when the due date/time
    /// occurs. Must be smaller than or equal to `max_icp_e8s`.
    pub min_icp_e8s: ::core::option::Option<u64>,
    /// The number of ICP that is "targeted" by this token swap. If this
    /// amount is achieved with sufficient participation, the swap will be
    /// triggered immediately, without waiting for the due date
    /// (`end_timestamp_seconds`). This means that an investor knows the minimum
    /// number of SNS tokens received per invested ICP. If this amount is achieved
    /// without reaching sufficient_participation, the swap will abort without
    /// waiting for the due date. Must be at least
    /// `min_participants * min_participant_icp_e8s`.
    pub max_icp_e8s: ::core::option::Option<u64>,
    /// The amount of ICP that is required to be directly contributed for this
    /// token swap to take place. This number + the minimum NF contribution divided
    /// by the number of SNS tokens being offered gives the seller's reserve price
    /// for the swap, i.e., the minimum number of ICP per SNS tokens that the
    /// seller of SNS tokens is willing to accept. If this amount is not achieved,
    /// the swap will be aborted (instead of committed) when the due date/time
    /// occurs. Must be smaller than or equal to `max_icp_e8s`.
    pub min_direct_participation_icp_e8s: ::core::option::Option<u64>,
    /// The amount of ICP that this token swap is "targeting" for direct
    /// contribution. If this amount is achieved with sufficient participation, the
    /// swap will be triggered immediately, without waiting for the due date
    /// (`end_timestamp_seconds`). This means that an investor knows the minimum
    /// number of SNS tokens received per invested ICP. If this amount is achieved
    /// without reaching sufficient_participation, the swap will abort without
    /// waiting for the due date. Must be at least
    /// `min_participants * min_participant_icp_e8s`.
    pub max_direct_participation_icp_e8s: ::core::option::Option<u64>,
    /// The minimum amount of ICP that each buyer must contribute to
    /// participate. Must be greater than zero.
    pub min_participant_icp_e8s: ::core::option::Option<u64>,
    /// The maximum amount of ICP that each buyer can contribute. Must be
    /// greater than or equal to `min_participant_icp_e8s` and less than
    /// or equal to `max_icp_e8s`. Can effectively be disabled by
    /// setting it to `max_icp_e8s`.
    pub max_participant_icp_e8s: ::core::option::Option<u64>,
    /// The date/time when the swap should start.
    pub swap_start_timestamp_seconds: ::core::option::Option<u64>,
    /// The date/time when the swap is due, i.e., it will automatically
    /// end and commit or abort depending on whether the parameters have
    /// been fulfilled.
    pub swap_due_timestamp_seconds: ::core::option::Option<u64>,
    /// The construction parameters for the basket of neurons created for all
    /// investors in the decentralization swap. Each investor, whether via
    /// the Neurons' Fund or direct, will receive `count` Neurons with
    /// increasing dissolve delays. The total number of Tokens swapped for
    /// by the investor will be evenly distributed across the basket. This is
    /// effectively a vesting schedule to ensure there is a gradual release of
    /// SNS Tokens available to all investors instead of being liquid immediately.
    /// See `NeuronBasketConstructionParameters` for more details on how
    /// the basket is configured.
    pub neuron_basket_construction_parameters:
        ::core::option::Option<super::sns_swap_pb::NeuronBasketConstructionParameters>,
    /// The ID of the NNS proposal submitted to launch this SNS decentralization
    /// swap.
    pub nns_proposal_id: ::core::option::Option<u64>,
    /// Whether or not the neurons' fund is participating
    pub neurons_fund_participation: ::core::option::Option<bool>,
    /// The token_logo for the SNS project represented as a base64 encoded string.
    pub token_logo: ::core::option::Option<String>,
    /// Constraints for the Neurons' Fund participation in this swap. These constraints passed from
    /// the NNS Governance (via SNS-W) to an SNS Swap to determine the Neurons' Fund participation
    /// amount as a function of the direct participation amount.
    pub neurons_fund_participation_constraints:
        ::core::option::Option<super::sns_swap_pb::NeuronsFundParticipationConstraints>,
    /// The initial tokens and neurons available at genesis will be distributed according
    /// to the strategy and configuration picked via the initial_token_distribution
    /// parameter.
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
