use crate::{
    consts::{E8S_PER_TOKEN, ONE_DAY_SECONDS, ONE_MONTH_SECONDS, ONE_YEAR_SECONDS},
    pbs::{
        gov_pb::{
            DefaultFollowees, NervousSystemParameters, NeuronPermissionList,
            VotingRewardsParameters,
        },
        nns_pb::Percentage,
    },
};

impl VotingRewardsParameters {
    /// This is an upper bound for `initial_reward_rate_basis_points_ceiling`. High values
    /// may improve the incentives when voting, but too-high values may also lead
    /// to an over-concentration of voting power and high inflation.
    pub const INITIAL_REWARD_RATE_BASIS_POINTS_CEILING: u64 = 10_000;

    pub fn with_default_values() -> Self {
        Self {
            round_duration_seconds: Some(ONE_DAY_SECONDS),
            reward_rate_transition_duration_seconds: Some(0),
            initial_reward_rate_basis_points: Some(0),
            final_reward_rate_basis_points: Some(0),
        }
    }
}

impl NervousSystemParameters {
    /// This is an upper bound for `max_proposals_to_keep_per_action`. Exceeding it
    /// may cause degradation in the governance canister or the subnet hosting the SNS.
    pub const MAX_PROPOSALS_TO_KEEP_PER_ACTION_CEILING: u32 = 700;

    /// This is an upper bound for `max_number_of_neurons`. Exceeding it may cause
    /// degradation in the governance canister or the subnet hosting the SNS.
    /// See also: `MAX_NEURONS_FOR_DIRECT_PARTICIPANTS`.
    pub const MAX_NUMBER_OF_NEURONS_CEILING: u64 = 200_000;

    /// This is an upper bound for `max_number_of_proposals_with_ballots`. Exceeding
    /// it may cause degradation in the governance canister or the subnet hosting the SNS.
    pub const MAX_NUMBER_OF_PROPOSALS_WITH_BALLOTS_CEILING: u64 = 700;

    /// This is an upper bound for `initial_voting_period_seconds`. Exceeding it may cause
    /// degradation in the governance canister or the subnet hosting the SNS.
    pub const INITIAL_VOTING_PERIOD_SECONDS_CEILING: u64 = 30 * ONE_DAY_SECONDS;

    /// This is a lower bound for `initial_voting_period_seconds`. Exceeding it may cause
    /// degradation in the governance canister or the subnet hosting the SNS.
    pub const INITIAL_VOTING_PERIOD_SECONDS_FLOOR: u64 = ONE_DAY_SECONDS;

    /// This is an upper bound for `wait_for_quiet_deadline_increase_seconds`. Exceeding it may cause
    /// degradation in the governance canister or the subnet hosting the SNS.
    pub const WAIT_FOR_QUIET_DEADLINE_INCREASE_SECONDS_CEILING: u64 = 30 * ONE_DAY_SECONDS;

    /// This is a lower bound for `wait_for_quiet_deadline_increase_seconds`. We're setting it to
    /// 1 instead of 0 because values of 0 are not currently well-tested.
    pub const WAIT_FOR_QUIET_DEADLINE_INCREASE_SECONDS_FLOOR: u64 = 1;

    /// This is an upper bound for `max_followees_per_function`. Exceeding it may cause
    /// degradation in the governance canister or the subnet hosting the SNS.
    pub const MAX_FOLLOWEES_PER_FUNCTION_CEILING: u64 = 15;

    /// This is an upper bound for `max_number_of_principals_per_neuron`. Exceeding
    /// it may cause may cause degradation in the governance canister or the subnet
    /// hosting the SNS.
    pub const MAX_NUMBER_OF_PRINCIPALS_PER_NEURON_CEILING: u64 = 15;

    /// This is an upper bound for `max_dissolve_delay_bonus_percentage`. High values
    /// may improve the incentives when voting, but too-high values may also lead
    /// to an over-concentration of voting power. The value used by the NNS is 100.
    pub const MAX_DISSOLVE_DELAY_BONUS_PERCENTAGE_CEILING: u64 = 900;

    /// This is an upper bound for `max_age_bonus_percentage`. High values
    /// may improve the incentives when voting, but too-high values may also lead
    /// to an over-concentration of voting power. The value used by the NNS is 25.
    pub const MAX_AGE_BONUS_PERCENTAGE_CEILING: u64 = 400;

    /// The proportion of "yes votes" as basis points of the total voting power
    /// that is required for the proposal to be adopted. For example, if this field
    /// is 300bp, then the proposal can only be adopted if the number of "yes
    /// votes" is greater than or equal to 3% of the total voting power.
    pub const DEFAULT_MINIMUM_YES_PROPORTION_OF_TOTAL_VOTING_POWER: Percentage =
        Percentage::from_basis_points(300); // 3%

    /// Same as DEFAULT_MINIMUM_YES_PROPORTION_OF_TOTAL_VOTING_POWER, but for "critical" proposals
    pub const CRITICAL_MINIMUM_YES_PROPORTION_OF_TOTAL_VOTING_POWER: Percentage =
        Percentage::from_basis_points(2_000); // 20%

    /// The proportion of "yes votes" as basis points of the exercised voting power
    /// that is required for the proposal to be adopted. For example, if this field
    /// is 5000bp, then the proposal can only be adopted if the number of "yes
    /// votes" is greater than or equal to 50% of the exercised voting power.
    pub const DEFAULT_MINIMUM_YES_PROPORTION_OF_EXERCISED_VOTING_POWER: Percentage =
        Percentage::from_basis_points(5_000); // 50%

    /// Same as DEFAULT_MINIMUM_YES_PROPORTION_OF_EXERCISED_VOTING_POWER, but for "critical" proposals
    pub const CRITICAL_MINIMUM_YES_PROPORTION_OF_EXERCISED_VOTING_POWER: Percentage =
        Percentage::from_basis_points(6_700); // 67%

    pub fn with_default_values() -> Self {
        Self {
            reject_cost_e8s: Some(E8S_PER_TOKEN), // 1 governance token
            neuron_minimum_stake_e8s: Some(E8S_PER_TOKEN), // 1 governance token
            transaction_fee_e8s: Some(10_000),
            max_proposals_to_keep_per_action: Some(100),
            initial_voting_period_seconds: Some(4 * ONE_DAY_SECONDS), // 4d
            wait_for_quiet_deadline_increase_seconds: Some(ONE_DAY_SECONDS), // 1d
            default_followees: Some(DefaultFollowees::default()),
            max_number_of_neurons: Some(200_000),
            neuron_minimum_dissolve_delay_to_vote_seconds: Some(6 * ONE_MONTH_SECONDS), // 6m
            max_followees_per_function: Some(15),
            max_dissolve_delay_seconds: Some(8 * ONE_YEAR_SECONDS), // 8y
            max_neuron_age_for_age_bonus: Some(4 * ONE_YEAR_SECONDS), // 4y
            max_number_of_proposals_with_ballots: Some(700),
            neuron_claimer_permissions: Some(Self::default_neuron_claimer_permissions()),
            neuron_grantable_permissions: Some(NeuronPermissionList::default()),
            max_number_of_principals_per_neuron: Some(5),
            voting_rewards_parameters: Some(VotingRewardsParameters::with_default_values()),
            max_dissolve_delay_bonus_percentage: Some(100),
            max_age_bonus_percentage: Some(25),
            maturity_modulation_disabled: Some(false),
        }
    }

    fn default_neuron_claimer_permissions() -> NeuronPermissionList {
        NeuronPermissionList {
            permissions: vec![2, 4, 3],
        }
    }
}
