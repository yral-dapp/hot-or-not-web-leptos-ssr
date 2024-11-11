use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    num::NonZeroU64,
    str::FromStr,
};

use candid::Principal;

use crate::{
    humanize::E8,
    pbs::{
        gov_pb::{NervousSystemParameters, NeuronPermissionList, VotingRewardsParameters},
        sns_pb::{
            sns_init_payload::InitialTokenDistribution, AirdropDistribution, DeveloperDistribution,
            FractionalDeveloperVotingPower, NeuronDistribution, SnsInitPayload, SwapDistribution,
        },
    },
};

use super::neurons_fund;

pub const MAX_DAPP_CANISTERS_COUNT: usize = 25;

pub const MAX_CONFIRMATION_TEXT_LENGTH: usize = 1_000;

pub const MAX_CONFIRMATION_TEXT_BYTES: usize = 8 * MAX_CONFIRMATION_TEXT_LENGTH;

pub const MIN_CONFIRMATION_TEXT_LENGTH: usize = 1;

pub const MAX_FALLBACK_CONTROLLER_PRINCIPAL_IDS_COUNT: usize = 15;

pub const MAX_DIRECT_ICP_CONTRIBUTION_TO_SWAP: u64 = 1_000_000_000 * E8;

pub const MIN_SNS_NEURONS_PER_BASKET: u64 = 2;

pub const MAX_SNS_NEURONS_PER_BASKET: u64 = 10;

enum MinDirectParticipationThresholdValidationError {
    // This value must be specified.
    Unspecified,
    // Needs to be greater or equal the minimum amount of ICP collected from direct participants.
    BelowSwapDirectIcpMin {
        min_direct_participation_threshold_icp_e8s: u64,
        min_direct_participation_icp_e8s: u64,
    },
    // Needs to be less than the maximum amount of ICP collected from direct participants.
    AboveSwapDirectIcpMax {
        min_direct_participation_threshold_icp_e8s: u64,
        max_direct_participation_icp_e8s: u64,
    },
}

impl std::fmt::Display for MinDirectParticipationThresholdValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "MinDirectParticipationThresholdValidationError: ";
        match self {
            Self::Unspecified => {
                write!(
                    f,
                    "{}min_direct_participation_threshold_icp_e8s must be specified.",
                    prefix
                )
            }
            Self::BelowSwapDirectIcpMin {
                min_direct_participation_threshold_icp_e8s,
                min_direct_participation_icp_e8s,
            } => {
                write!(
                    f,
                    "{}min_direct_participation_threshold_icp_e8s ({}) should be greater \
                    than or equal min_direct_participation_icp_e8s ({}).",
                    prefix,
                    min_direct_participation_threshold_icp_e8s,
                    min_direct_participation_icp_e8s,
                )
            }
            Self::AboveSwapDirectIcpMax {
                min_direct_participation_threshold_icp_e8s,
                max_direct_participation_icp_e8s,
            } => {
                write!(
                    f,
                    "{}min_direct_participation_threshold_icp_e8s ({}) should be less \
                    than or equal max_direct_participation_icp_e8s ({}).",
                    prefix,
                    min_direct_participation_threshold_icp_e8s,
                    max_direct_participation_icp_e8s,
                )
            }
        }
    }
}

enum MaxNeuronsFundParticipationValidationError {
    // This value must be specified.
    Unspecified,
    // Does not make sense if no SNS neurons can be created.
    BelowSingleParticipationLimit {
        max_neurons_fund_participation_icp_e8s: NonZeroU64,
        min_participant_icp_e8s: u64,
    },
    // The Neuron's Fund should never provide more funds than can be contributed directly.
    AboveSwapMaxDirectIcp {
        max_neurons_fund_participation_icp_e8s: u64,
        max_direct_participation_icp_e8s: u64,
    },
}

impl std::fmt::Display for MaxNeuronsFundParticipationValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "MaxNeuronsFundParticipationValidationError: ";
        match self {
            Self::Unspecified => {
                write!(
                    f,
                    "{}max_neurons_fund_participation_icp_e8s must be specified.",
                    prefix
                )
            }
            Self::BelowSingleParticipationLimit {
                max_neurons_fund_participation_icp_e8s,
                min_participant_icp_e8s,
            } => {
                write!(
                    f,
                    "{}max_neurons_fund_participation_icp_e8s ({} > 0) \
                    should be greater than or equal min_participant_icp_e8s ({}).",
                    prefix, max_neurons_fund_participation_icp_e8s, min_participant_icp_e8s,
                )
            }
            Self::AboveSwapMaxDirectIcp {
                max_neurons_fund_participation_icp_e8s,
                max_direct_participation_icp_e8s,
            } => {
                write!(
                    f,
                    "{}max_neurons_fund_participation_icp_e8s ({}) \
                    should be less than or equal max_direct_participation_icp_e8s ({}).",
                    prefix,
                    max_neurons_fund_participation_icp_e8s,
                    max_direct_participation_icp_e8s,
                )
            }
        }
    }
}

enum NeuronsFundParticipationConstraintsValidationError {
    SetBeforeProposalExecution,
    RelatedFieldUnspecified(String),
    MinDirectParticipationThresholdValidationError(MinDirectParticipationThresholdValidationError),
    MaxNeuronsFundParticipationValidationError(MaxNeuronsFundParticipationValidationError),
    // "Inherit" the remaining, local error cases.
    Local(neurons_fund::NeuronsFundParticipationConstraintsValidationError),
}

impl std::fmt::Display for NeuronsFundParticipationConstraintsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "NeuronsFundParticipationConstraintsValidationError: ";
        match self {
            Self::SetBeforeProposalExecution => {
                write!(
                    f,
                    "{}neurons_fund_participation_constraints must not be set before \
                    the CreateServiceNervousSystem proposal is executed.",
                    prefix
                )
            }
            Self::RelatedFieldUnspecified(related_field_name) => {
                write!(f, "{}{} must be specified.", prefix, related_field_name,)
            }
            Self::MinDirectParticipationThresholdValidationError(error) => {
                write!(f, "{}{}", prefix, error)
            }
            Self::MaxNeuronsFundParticipationValidationError(error) => {
                write!(f, "{}{}", prefix, error)
            }
            Self::Local(error) => write!(f, "{}{}", prefix, error),
        }
    }
}

impl From<NeuronsFundParticipationConstraintsValidationError> for Result<(), String> {
    fn from(value: NeuronsFundParticipationConstraintsValidationError) -> Self {
        Err(value.to_string())
    }
}

#[derive(Clone, Copy)]
pub enum NeuronBasketConstructionParametersValidationError {
    ExceedsMaximalDissolveDelay(u64),
    ExceedsU64,
    BasketSizeTooSmall,
    BasketSizeTooBig,
    InadequateDissolveDelay,
    UnexpectedInLegacyFlow,
}

impl NeuronBasketConstructionParametersValidationError {
    fn field_name() -> String {
        "SnsInitPayload.neuron_basket_construction_parameters".to_string()
    }
}

impl std::fmt::Display for NeuronBasketConstructionParametersValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::ExceedsMaximalDissolveDelay(max_dissolve_delay_seconds) => {
                format!(
                    "must satisfy (count - 1) * dissolve_delay_interval_seconds \
                    < SnsInitPayload.max_dissolve_delay_seconds = {max_dissolve_delay_seconds}"
                )
            }
            Self::BasketSizeTooSmall => format!(
                "basket count must be at least {}",
                MIN_SNS_NEURONS_PER_BASKET
            ),
            Self::BasketSizeTooBig => format!(
                "basket count must be at most {}",
                MAX_SNS_NEURONS_PER_BASKET
            ),
            Self::InadequateDissolveDelay => {
                "dissolve_delay_interval_seconds must be at least 1".to_string()
            }
            Self::ExceedsU64 => {
                format!(
                    "must satisfy (count - 1) * dissolve_delay_interval_seconds \
                    < u64::MAX = {}",
                    u64::MAX
                )
            }
            Self::UnexpectedInLegacyFlow => {
                "must not be set with the legacy flow for SNS decentralization swaps".to_string()
            }
        };
        write!(f, "{} {msg}", Self::field_name())
    }
}

impl From<NeuronBasketConstructionParametersValidationError> for Result<(), String> {
    fn from(val: NeuronBasketConstructionParametersValidationError) -> Self {
        Err(val.to_string())
    }
}

impl FractionalDeveloperVotingPower {
    pub(crate) fn swap_distribution(&self) -> Result<&SwapDistribution, String> {
        self.swap_distribution
            .as_ref()
            .ok_or_else(|| "Expected swap distribution to exist".to_string())
    }

    fn validate_neurons(
        &self,
        developer_distribution: &DeveloperDistribution,
        airdrop_distribution: &AirdropDistribution,
        nervous_system_parameters: &NervousSystemParameters,
    ) -> Result<(), String> {
        let neuron_minimum_dissolve_delay_to_vote_seconds = nervous_system_parameters
            .neuron_minimum_dissolve_delay_to_vote_seconds
            .as_ref()
            .expect("Expected NervousSystemParameters.neuron_minimum_dissolve_delay_to_vote_seconds to be set");

        let max_dissolve_delay_seconds = nervous_system_parameters
            .max_dissolve_delay_seconds
            .as_ref()
            .expect("Expected NervousSystemParameters.max_dissolve_delay_seconds to be set");

        let missing_developer_principals_count = developer_distribution
            .developer_neurons
            .iter()
            .filter(|neuron_distribution| neuron_distribution.controller.is_none())
            .count();

        if missing_developer_principals_count != 0 {
            return Err(format!(
                "Error: {} developer_neurons are missing controllers",
                missing_developer_principals_count
            ));
        }

        let deduped_dev_neurons = developer_distribution
            .developer_neurons
            .iter()
            .map(|neuron_distribution| {
                (
                    (neuron_distribution.controller, neuron_distribution.memo),
                    neuron_distribution.stake_e8s,
                )
            })
            .collect::<BTreeMap<_, _>>();

        if deduped_dev_neurons.len() != developer_distribution.developer_neurons.len() {
            return Err(
                "Error: Neurons with the same controller and memo found in developer_neurons"
                    .to_string(),
            );
        }

        // The max number of DeveloperDistributions that can be specified in the SnsInitPayload.
        const MAX_DEVELOPER_DISTRIBUTION_COUNT: usize = 100;

        // The max number of AirdropDistributions that can be specified in the SnsInitPayload.
        const MAX_AIRDROP_DISTRIBUTION_COUNT: usize = 1000;

        if deduped_dev_neurons.len() > MAX_DEVELOPER_DISTRIBUTION_COUNT {
            return Err(format!(
                "Error: The number of developer neurons must be less than {}. Current count is {}",
                MAX_DEVELOPER_DISTRIBUTION_COUNT,
                deduped_dev_neurons.len(),
            ));
        }

        // Range of allowed memos for neurons distributed via an SNS swap. This range is used to choose
        // the memos of neurons in the neuron basket, and to enforce that other memos (e.g. for Airdrop
        // neurons) do not conflict with the neuron basket memos.
        const NEURON_BASKET_MEMO_RANGE_START: u64 = 1_000_000;
        const SALE_NEURON_MEMO_RANGE_END: u64 = 10_000_000;

        for (controller, memo) in deduped_dev_neurons.keys() {
            if NEURON_BASKET_MEMO_RANGE_START <= *memo && *memo <= SALE_NEURON_MEMO_RANGE_END {
                return Err(format!(
                    "Error: Developer neuron with controller {} cannot have a memo in the range {} to {}",
                    controller.unwrap(),
                    NEURON_BASKET_MEMO_RANGE_START,
                    SALE_NEURON_MEMO_RANGE_END
                ));
            }
        }

        let missing_airdrop_principals_count = airdrop_distribution
            .airdrop_neurons
            .iter()
            .filter(|neuron_distribution| neuron_distribution.controller.is_none())
            .count();

        if missing_airdrop_principals_count != 0 {
            return Err(format!(
                "Error: {} airdrop_neurons are missing controllers",
                missing_airdrop_principals_count
            ));
        }

        let deduped_airdrop_neurons = airdrop_distribution
            .airdrop_neurons
            .iter()
            .map(|neuron_distribution| {
                (
                    (neuron_distribution.controller, neuron_distribution.memo),
                    neuron_distribution.stake_e8s,
                )
            })
            .collect::<BTreeMap<_, _>>();

        if deduped_airdrop_neurons.len() != airdrop_distribution.airdrop_neurons.len() {
            return Err(
                "Error: Neurons with the same controller and memo detected in airdrop_neurons"
                    .to_string(),
            );
        }

        if deduped_airdrop_neurons.len() > MAX_AIRDROP_DISTRIBUTION_COUNT {
            return Err(format!(
                "Error: The number of airdrop neurons must be less than {}. Current count is {}",
                MAX_AIRDROP_DISTRIBUTION_COUNT,
                deduped_airdrop_neurons.len(),
            ));
        }

        for (controller, memo) in deduped_airdrop_neurons.keys() {
            if NEURON_BASKET_MEMO_RANGE_START <= *memo && *memo <= SALE_NEURON_MEMO_RANGE_END {
                return Err(format!(
                    "Error: Airdrop neuron with controller {} cannot have a memo in the range {} to {}",
                    controller.unwrap(),
                    NEURON_BASKET_MEMO_RANGE_START,
                    SALE_NEURON_MEMO_RANGE_END
                ));
            }
        }

        let mut duplicated_neuron_principals = vec![];
        for developer_principal in deduped_dev_neurons.keys() {
            if deduped_airdrop_neurons.contains_key(developer_principal) {
                // Safe to unwrap due to the checks done above
                duplicated_neuron_principals.push(developer_principal.0.unwrap())
            }
        }

        if !duplicated_neuron_principals.is_empty() {
            return Err(format!(
                "Error: The following controllers are present in AirdropDistribution \
                and DeveloperDistribution: {:?}",
                duplicated_neuron_principals
            ));
        }

        let configured_at_least_one_voting_neuron = developer_distribution
            .developer_neurons
            .iter()
            .chain(&airdrop_distribution.airdrop_neurons)
            .any(|neuron_distribution| {
                neuron_distribution.dissolve_delay_seconds
                    >= *neuron_minimum_dissolve_delay_to_vote_seconds
            });

        if !configured_at_least_one_voting_neuron {
            return Err(format!(
                "Error: There needs to be at least one voting-eligible neuron configured. To be \
                 eligible to vote, a neuron must have dissolve_delay_seconds of at least {}",
                neuron_minimum_dissolve_delay_to_vote_seconds
            ));
        }

        let misconfigured_dissolve_delay_principals: Vec<Principal> = developer_distribution
            .developer_neurons
            .iter()
            .chain(&airdrop_distribution.airdrop_neurons)
            .filter(|neuron_distribution| {
                neuron_distribution.dissolve_delay_seconds > *max_dissolve_delay_seconds
            })
            .map(|neuron_distribution| neuron_distribution.controller.unwrap())
            .collect();

        if !misconfigured_dissolve_delay_principals.is_empty() {
            return Err(format!(
                "Error: The following PrincipalIds have a dissolve_delay_seconds configured greater than \
                 the allowed max_dissolve_delay_seconds ({}): {:?}", max_dissolve_delay_seconds, misconfigured_dissolve_delay_principals
            ));
        }

        Ok(())
    }

    pub fn validate(
        &self,
        nervous_system_parameters: &NervousSystemParameters,
    ) -> Result<(), String> {
        let developer_distribution = self
            .developer_distribution
            .as_ref()
            .ok_or("Error: developer_distribution must be specified")?;

        self.treasury_distribution
            .as_ref()
            .ok_or("Error: treasury_distribution must be specified")?;

        let swap_distribution = self
            .swap_distribution
            .as_ref()
            .ok_or("Error: swap_distribution must be specified")?;

        let airdrop_distribution = self
            .airdrop_distribution
            .as_ref()
            .ok_or("Error: airdrop_distribution must be specified")?;

        self.validate_neurons(
            developer_distribution,
            airdrop_distribution,
            nervous_system_parameters,
        )?;

        match Self::get_total_distributions(&airdrop_distribution.airdrop_neurons) {
            Ok(_) => (),
            Err(_) => return Err("Error: The sum of all airdrop allocated tokens overflowed and is an invalid distribution".to_string()),
        };

        if swap_distribution.initial_swap_amount_e8s == 0 {
            return Err(
                "Error: swap_distribution.initial_swap_amount_e8s must be greater than 0"
                    .to_string(),
            );
        }

        if swap_distribution.total_e8s < swap_distribution.initial_swap_amount_e8s {
            return Err("Error: swap_distribution.total_e8 must be greater than or equal to swap_distribution.initial_swap_amount_e8s".to_string());
        }

        let total_developer_e8s = match Self::get_total_distributions(&developer_distribution.developer_neurons) {
            Ok(total) => total,
            Err(_) => return Err("Error: The sum of all developer allocated tokens overflowed and is an invalid distribution".to_string()),
        };

        if total_developer_e8s > swap_distribution.total_e8s {
            return Err("Error: The sum of all developer allocated tokens must be less than or equal to swap_distribution.total_e8s".to_string());
        }

        Ok(())
    }

    fn get_total_distributions(distributions: &Vec<NeuronDistribution>) -> Result<u64, String> {
        let mut distribution_total: u64 = 0;
        for distribution in distributions {
            distribution_total = match distribution_total.checked_add(distribution.stake_e8s) {
                Some(total) => total,
                None => {
                    return Err(
                        "The total distribution overflowed and is not a valid distribution"
                            .to_string(),
                    )
                }
            }
        }

        Ok(distribution_total)
    }
}

impl SnsInitPayload {
    /// Due to conflict with the prost derived macros on the generated Rust structs, this method
    /// acts like `SnsInitPayload::default()` except that it will provide default "real" values
    /// for default-able parameters.
    pub fn with_default_values() -> Self {
        let nervous_system_parameters_default = NervousSystemParameters::with_default_values();
        let voting_rewards_parameters = nervous_system_parameters_default
            .voting_rewards_parameters
            .as_ref()
            .unwrap();
        Self {
            transaction_fee_e8s: nervous_system_parameters_default.transaction_fee_e8s,
            reward_rate_transition_duration_seconds: voting_rewards_parameters
                .reward_rate_transition_duration_seconds,
            initial_reward_rate_basis_points: voting_rewards_parameters
                .initial_reward_rate_basis_points,
            final_reward_rate_basis_points: voting_rewards_parameters
                .final_reward_rate_basis_points,
            token_name: None,
            token_symbol: None,
            token_logo: None,
            proposal_reject_cost_e8s: nervous_system_parameters_default.reject_cost_e8s,
            neuron_minimum_stake_e8s: nervous_system_parameters_default.neuron_minimum_stake_e8s,
            neuron_minimum_dissolve_delay_to_vote_seconds: nervous_system_parameters_default
                .neuron_minimum_dissolve_delay_to_vote_seconds,
            initial_token_distribution: None,
            fallback_controller_principal_ids: vec![],
            logo: None,
            url: None,
            name: None,
            description: None,
            max_dissolve_delay_seconds: nervous_system_parameters_default
                .max_dissolve_delay_seconds,
            max_neuron_age_seconds_for_age_bonus: nervous_system_parameters_default
                .max_neuron_age_for_age_bonus,
            max_dissolve_delay_bonus_percentage: nervous_system_parameters_default
                .max_dissolve_delay_bonus_percentage,
            max_age_bonus_percentage: nervous_system_parameters_default.max_age_bonus_percentage,
            initial_voting_period_seconds: nervous_system_parameters_default
                .initial_voting_period_seconds,
            wait_for_quiet_deadline_increase_seconds: nervous_system_parameters_default
                .wait_for_quiet_deadline_increase_seconds,
            dapp_canisters: None,
            min_participants: None,
            min_icp_e8s: None,
            max_icp_e8s: None,
            min_direct_participation_icp_e8s: None,
            max_direct_participation_icp_e8s: None,
            min_participant_icp_e8s: None,
            max_participant_icp_e8s: None,
            swap_start_timestamp_seconds: None,
            swap_due_timestamp_seconds: None,
            neuron_basket_construction_parameters: None,
            confirmation_text: None,
            restricted_countries: None,
            nns_proposal_id: None,
            neurons_fund_participation_constraints: None,
            neurons_fund_participation: None,
        }
    }

    fn get_swap_distribution(&self) -> Result<&SwapDistribution, String> {
        match &self.initial_token_distribution {
            None => Err("Error: initial-token-distribution must be specified".to_string()),
            Some(InitialTokenDistribution::FractionalDeveloperVotingPower(f)) => {
                f.swap_distribution()
            }
        }
    }

    /// Returns a complete NervousSystemParameter struct with its corresponding SnsInitPayload
    /// fields filled out.
    fn get_nervous_system_parameters(&self) -> NervousSystemParameters {
        let nervous_system_parameters = NervousSystemParameters::with_default_values();
        let all_permissions = NeuronPermissionList {
            permissions: (0..=10).collect(),
        };

        let SnsInitPayload {
            transaction_fee_e8s,
            token_name: _,
            token_symbol: _,
            proposal_reject_cost_e8s: reject_cost_e8s,
            neuron_minimum_stake_e8s,
            fallback_controller_principal_ids: _,
            logo: _,
            url: _,
            name: _,
            description: _,
            neuron_minimum_dissolve_delay_to_vote_seconds,
            reward_rate_transition_duration_seconds,
            initial_reward_rate_basis_points,
            final_reward_rate_basis_points,
            initial_token_distribution: _,
            max_dissolve_delay_seconds,
            max_neuron_age_seconds_for_age_bonus: max_neuron_age_for_age_bonus,
            max_dissolve_delay_bonus_percentage,
            max_age_bonus_percentage,
            initial_voting_period_seconds,
            wait_for_quiet_deadline_increase_seconds,
            dapp_canisters: _,
            confirmation_text: _,
            restricted_countries: _,
            min_participants: _,
            min_icp_e8s: _,
            max_icp_e8s: _,
            min_direct_participation_icp_e8s: _,
            max_direct_participation_icp_e8s: _,
            min_participant_icp_e8s: _,
            max_participant_icp_e8s: _,
            swap_start_timestamp_seconds: _,
            swap_due_timestamp_seconds: _,
            neuron_basket_construction_parameters: _,
            nns_proposal_id: _,
            token_logo: _,
            neurons_fund_participation_constraints: _,
            neurons_fund_participation: _,
        } = self.clone();

        let voting_rewards_parameters = Some(VotingRewardsParameters {
            reward_rate_transition_duration_seconds,
            initial_reward_rate_basis_points,
            final_reward_rate_basis_points,
            ..nervous_system_parameters.voting_rewards_parameters.unwrap()
        });

        NervousSystemParameters {
            neuron_claimer_permissions: Some(all_permissions.clone()),
            neuron_grantable_permissions: Some(all_permissions),
            transaction_fee_e8s,
            reject_cost_e8s,
            neuron_minimum_stake_e8s,
            neuron_minimum_dissolve_delay_to_vote_seconds,
            voting_rewards_parameters,
            max_dissolve_delay_seconds,
            max_neuron_age_for_age_bonus,
            max_dissolve_delay_bonus_percentage,
            max_age_bonus_percentage,
            initial_voting_period_seconds,
            wait_for_quiet_deadline_increase_seconds,
            ..nervous_system_parameters
        }
    }

    /// Validates all the fields that are shared with CreateServiceNervousSystem.
    /// For use in e.g. the SNS CLI or in NNS Governance before the proposal has
    /// been executed.
    pub fn validate_pre_execution(&self) -> Result<Self, String> {
        let validation_fns = [
            self.validate_token_symbol(),
            self.validate_token_name(),
            self.validate_token_logo(),
            self.validate_token_distribution(),
            self.validate_participation_constraints(),
            self.validate_neuron_minimum_stake_e8s(),
            self.validate_neuron_minimum_dissolve_delay_to_vote_seconds(),
            self.validate_neuron_basket_construction_params(),
            self.validate_proposal_reject_cost_e8s(),
            self.validate_transaction_fee_e8s(),
            self.validate_fallback_controller_principal_ids(),
            self.validate_url(),
            self.validate_logo(),
            self.validate_description(),
            self.validate_name(),
            self.validate_initial_reward_rate_basis_points(),
            self.validate_final_reward_rate_basis_points(),
            self.validate_reward_rate_transition_duration_seconds(),
            self.validate_max_dissolve_delay_seconds(),
            self.validate_max_neuron_age_seconds_for_age_bonus(),
            self.validate_max_dissolve_delay_bonus_percentage(),
            self.validate_max_age_bonus_percentage(),
            self.validate_initial_voting_period_seconds(),
            self.validate_wait_for_quiet_deadline_increase_seconds(),
            self.validate_dapp_canisters(),
            self.validate_confirmation_text(),
            self.validate_restricted_countries(),
            // Ensure that the values that can only be known after the execution
            // of the CreateServiceNervousSystem proposal are not set.
            self.validate_nns_proposal_id_pre_execution(),
            self.validate_swap_start_timestamp_seconds_pre_execution(),
            self.validate_swap_due_timestamp_seconds_pre_execution(),
            self.validate_neurons_fund_participation_constraints(true),
            self.validate_neurons_fund_participation(),
            // Obsolete fields are not set
            self.validate_min_icp_e8s(),
            self.validate_max_icp_e8s(),
        ];

        self.join_validation_results(&validation_fns)
    }

    pub fn validate_post_execution(&self) -> Result<Self, String> {
        let validation_fns = [
            self.validate_token_symbol(),
            self.validate_token_name(),
            self.validate_token_logo(),
            self.validate_token_distribution(),
            self.validate_participation_constraints(),
            self.validate_neuron_minimum_stake_e8s(),
            self.validate_neuron_minimum_dissolve_delay_to_vote_seconds(),
            self.validate_neuron_basket_construction_params(),
            self.validate_proposal_reject_cost_e8s(),
            self.validate_transaction_fee_e8s(),
            self.validate_fallback_controller_principal_ids(),
            self.validate_url(),
            self.validate_logo(),
            self.validate_description(),
            self.validate_name(),
            self.validate_initial_reward_rate_basis_points(),
            self.validate_final_reward_rate_basis_points(),
            self.validate_reward_rate_transition_duration_seconds(),
            self.validate_max_dissolve_delay_seconds(),
            self.validate_max_neuron_age_seconds_for_age_bonus(),
            self.validate_max_dissolve_delay_bonus_percentage(),
            self.validate_max_age_bonus_percentage(),
            self.validate_initial_voting_period_seconds(),
            self.validate_wait_for_quiet_deadline_increase_seconds(),
            self.validate_dapp_canisters(),
            self.validate_confirmation_text(),
            self.validate_restricted_countries(),
            self.validate_all_post_execution_swap_parameters_are_set(),
            self.validate_nns_proposal_id(),
            self.validate_swap_start_timestamp_seconds(),
            self.validate_swap_due_timestamp_seconds(),
            self.validate_neurons_fund_participation_constraints(false),
            self.validate_neurons_fund_participation(),
            // Obsolete fields are not set
            self.validate_min_icp_e8s(),
            self.validate_max_icp_e8s(),
        ];

        self.join_validation_results(&validation_fns)
    }

    fn join_validation_results(
        &self,
        validation_fns: &[Result<(), String>],
    ) -> Result<Self, String> {
        let mut seen_messages = HashSet::new();
        let defect_messages = validation_fns
            .iter()
            .filter_map(|validation_fn| match validation_fn {
                Err(msg) => Some(msg),
                Ok(_) => None,
            })
            .filter(|&x|
                // returns true iff the set did not already contain the value
                seen_messages.insert(x.clone()))
            .cloned()
            .collect::<Vec<String>>()
            .join("\n");

        if defect_messages.is_empty() {
            Ok(self.clone())
        } else {
            Err(defect_messages)
        }
    }

    fn validate_token_symbol(&self) -> Result<(), String> {
        let token_symbol = self
            .token_symbol
            .as_ref()
            .ok_or_else(|| "Error: token-symbol must be specified".to_string())?;

        // The maximum number of characters allowed for token symbol.
        const MAX_TOKEN_SYMBOL_LENGTH: usize = 10;

        // The minimum number of characters allowed for token symbol.
        const MIN_TOKEN_SYMBOL_LENGTH: usize = 3;

        // Token Symbols that can not be used.
        const BANNED_TOKEN_SYMBOLS: &[&str] = &["ICP", "DFINITY"];

        if token_symbol.len() > MAX_TOKEN_SYMBOL_LENGTH {
            return Err(format!(
                "Error: token-symbol must be fewer than {} characters, given character count: {}",
                MAX_TOKEN_SYMBOL_LENGTH,
                token_symbol.len()
            ));
        }

        if token_symbol.len() < MIN_TOKEN_SYMBOL_LENGTH {
            return Err(format!(
                "Error: token-symbol must be greater than {} characters, given character count: {}",
                MIN_TOKEN_SYMBOL_LENGTH,
                token_symbol.len()
            ));
        }

        if token_symbol != token_symbol.trim() {
            return Err("Token symbol must not have leading or trailing whitespaces".to_string());
        }

        if BANNED_TOKEN_SYMBOLS.contains(&token_symbol.to_uppercase().as_ref()) {
            return Err("Banned token symbol, please chose another one.".to_string());
        }

        Ok(())
    }

    fn validate_token_name(&self) -> Result<(), String> {
        let token_name = self
            .token_name
            .as_ref()
            .ok_or_else(|| "Error: token-name must be specified".to_string())?;

        // The maximum number of characters allowed for token name.
        const MAX_TOKEN_NAME_LENGTH: usize = 255;

        // The minimum number of characters allowed for token name.
        const MIN_TOKEN_NAME_LENGTH: usize = 4;

        // Token Names that can not be used.
        const BANNED_TOKEN_NAMES: &[&str] = &["internetcomputer", "internetcomputerprotocol"];

        if token_name.len() > MAX_TOKEN_NAME_LENGTH {
            return Err(format!(
                "Error: token-name must be fewer than {} characters, given character count: {}",
                MAX_TOKEN_NAME_LENGTH,
                token_name.len()
            ));
        }

        if token_name.len() < MIN_TOKEN_NAME_LENGTH {
            return Err(format!(
                "Error: token-name must be greater than {} characters, given character count: {}",
                MIN_TOKEN_NAME_LENGTH,
                token_name.len()
            ));
        }

        if token_name != token_name.trim() {
            return Err("Token name must not have leading or trailing whitespaces".to_string());
        }

        if BANNED_TOKEN_NAMES.contains(
            &token_name
                .to_lowercase()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .as_ref(),
        ) {
            return Err("Banned token name, please chose another one.".to_string());
        }

        Ok(())
    }

    fn validate_token_logo(&self) -> Result<(), String> {
        let token_logo = self
            .token_logo
            .as_ref()
            .ok_or_else(|| "Error: token_logo must be specified".to_string())?;

        const PREFIX: &str = "data:image/png;base64,";
        // The maximum number of characters allowed for a SNS logo encoding.
        // Roughly 256Kb
        const MAX_LOGO_LENGTH: usize = 341334;

        if token_logo.len() > MAX_LOGO_LENGTH {
            return Err(format!(
                "Error: token_logo must be less than {} characters, roughly 256 Kb",
                MAX_LOGO_LENGTH
            ));
        }

        if !token_logo.starts_with(PREFIX) {
            return Err(format!(
                "Error: token_logo must be a base64 encoded PNG, but the provided \
                string doesn't begin with `{PREFIX}`."
            ));
        }

        // TODO: add b64 validation
        // if base64::decode(&token_logo[PREFIX.len()..]).is_err() {
        //     return Err("Couldn't decode base64 in SnsMetadata.logo".to_string());
        // }

        Ok(())
    }

    fn validate_token_distribution(&self) -> Result<(), String> {
        let initial_token_distribution = self
            .initial_token_distribution
            .as_ref()
            .ok_or_else(|| "Error: initial-token-distribution must be specified".to_string())?;

        let nervous_system_parameters = self.get_nervous_system_parameters();

        match initial_token_distribution {
            InitialTokenDistribution::FractionalDeveloperVotingPower(f) => {
                f.validate(&nervous_system_parameters)?
            }
        }

        Ok(())
    }

    fn validate_transaction_fee_e8s(&self) -> Result<(), String> {
        match self.transaction_fee_e8s {
            Some(_) => Ok(()),
            None => Err("Error: transaction_fee_e8s must be specified.".to_string()),
        }
    }

    fn validate_proposal_reject_cost_e8s(&self) -> Result<(), String> {
        match self.proposal_reject_cost_e8s {
            Some(_) => Ok(()),
            None => Err("Error: proposal_reject_cost_e8s must be specified.".to_string()),
        }
    }

    fn validate_neuron_minimum_stake_e8s(&self) -> Result<(), String> {
        let neuron_minimum_stake_e8s = self
            .neuron_minimum_stake_e8s
            .expect("Error: neuron_minimum_stake_e8s must be specified.");
        let initial_token_distribution = self
            .initial_token_distribution
            .as_ref()
            .ok_or_else(|| "Error: initial-token-distribution must be specified".to_string())?;

        match initial_token_distribution {
            InitialTokenDistribution::FractionalDeveloperVotingPower(f) => {
                let developer_distribution = f
                    .developer_distribution
                    .as_ref()
                    .ok_or_else(|| "Error: developer_distribution must be specified".to_string())?;

                let airdrop_distribution = f
                    .airdrop_distribution
                    .as_ref()
                    .ok_or_else(|| "Error: airdrop_distribution must be specified".to_string())?;

                let min_stake_infringing_developer_neurons: Vec<(Principal, u64)> =
                    developer_distribution
                        .developer_neurons
                        .iter()
                        .filter_map(|neuron_distribution| {
                            if neuron_distribution.stake_e8s < neuron_minimum_stake_e8s {
                                // Safe to unwrap due to the checks done above
                                Some((
                                    neuron_distribution.controller.unwrap(),
                                    neuron_distribution.stake_e8s,
                                ))
                            } else {
                                None
                            }
                        })
                        .collect();

                if !min_stake_infringing_developer_neurons.is_empty() {
                    return Err(format!(
                        "Error: {} developer neurons have a stake below the minimum stake ({} e8s):  \n {:?}",
                        min_stake_infringing_developer_neurons.len(),
                        neuron_minimum_stake_e8s,
                        min_stake_infringing_developer_neurons,
                    ));
                }

                let min_stake_infringing_airdrop_neurons: Vec<(Principal, u64)> =
                    airdrop_distribution
                        .airdrop_neurons
                        .iter()
                        .filter_map(|neuron_distribution| {
                            if neuron_distribution.stake_e8s < neuron_minimum_stake_e8s {
                                // Safe to unwrap due to the checks done above
                                Some((
                                    neuron_distribution.controller.unwrap(),
                                    neuron_distribution.stake_e8s,
                                ))
                            } else {
                                None
                            }
                        })
                        .collect();

                if !min_stake_infringing_airdrop_neurons.is_empty() {
                    return Err(format!(
                        "Error: {} airdrop neurons have a stake below the minimum stake ({} e8s):  \n {:?}",
                        min_stake_infringing_airdrop_neurons.len(),
                        neuron_minimum_stake_e8s,
                        min_stake_infringing_airdrop_neurons,
                    ));
                }
            }
        }

        Ok(())
    }

    fn validate_neuron_minimum_dissolve_delay_to_vote_seconds(&self) -> Result<(), String> {
        // As this is not currently configurable, pull the default value from
        let max_dissolve_delay_seconds = *NervousSystemParameters::with_default_values()
            .max_dissolve_delay_seconds
            .as_ref()
            .unwrap();

        let neuron_minimum_dissolve_delay_to_vote_seconds = self
            .neuron_minimum_dissolve_delay_to_vote_seconds
            .ok_or_else(|| {
                "Error: neuron-minimum-dissolve-delay-to-vote-seconds must be specified".to_string()
            })?;

        if neuron_minimum_dissolve_delay_to_vote_seconds > max_dissolve_delay_seconds {
            return Err(format!(
                "The minimum dissolve delay to vote ({}) cannot be greater than the max \
                dissolve delay ({})",
                neuron_minimum_dissolve_delay_to_vote_seconds, max_dissolve_delay_seconds
            ));
        }

        Ok(())
    }

    fn validate_fallback_controller_principal_ids(&self) -> Result<(), String> {
        if self.fallback_controller_principal_ids.is_empty() {
            return Err(
                "Error: At least one principal ID must be supplied as a fallback controller \
                 in case the initial token swap fails."
                    .to_string(),
            );
        }

        if self.fallback_controller_principal_ids.len()
            > MAX_FALLBACK_CONTROLLER_PRINCIPAL_IDS_COUNT
        {
            return Err(format!(
                "Error: The number of fallback_controller_principal_ids \
                must be less than {}. Current count is {}",
                MAX_FALLBACK_CONTROLLER_PRINCIPAL_IDS_COUNT,
                self.fallback_controller_principal_ids.len()
            ));
        }

        let (valid_principals, invalid_principals): (Vec<_>, Vec<_>) = self
            .fallback_controller_principal_ids
            .iter()
            .map(|principal_id_string| {
                (
                    principal_id_string,
                    Principal::from_str(principal_id_string),
                )
            })
            .partition(|item| item.1.is_ok());

        if !invalid_principals.is_empty() {
            return Err(format!(
                "Error: One or more fallback_controller_principal_ids is not a valid principal id. \
                The follow principals are invalid: {:?}",
                invalid_principals
                    .into_iter()
                    .map(|pair| pair.0)
                    .collect::<Vec<_>>()
            ));
        }

        // At this point, all principals are valid. Dedupe the values
        let unique_principals: BTreeSet<_> = valid_principals
            .iter()
            .filter_map(|pair| pair.1.clone().ok())
            .collect();

        if unique_principals.len() != valid_principals.len() {
            return Err(
                "Error: Duplicate PrincipalIds found in fallback_controller_principal_ids"
                    .to_string(),
            );
        }

        Ok(())
    }

    fn validate_logo(&self) -> Result<(), String> {
        let logo = self
            .logo
            .as_ref()
            .ok_or_else(|| "Error: logo must be specified".to_string())?;

        const PREFIX: &str = "data:image/png;base64,";
        const MAX_LOGO_LENGTH: usize = 341334;

        // TODO: Should we check that it's a valid PNG?
        if logo.len() > MAX_LOGO_LENGTH {
            return Err(format!(
                "SnsMetadata.logo must be less than {} characters, roughly 256 Kb",
                MAX_LOGO_LENGTH
            ));
        }
        if !logo.starts_with(PREFIX) {
            return Err(format!("SnsMetadata.logo must be a base64 encoded PNG, but the provided string does't begin with `{PREFIX}`."));
        }

        // TODO: add b64 validation
        // if base64::decode(&logo[PREFIX.len()..]).is_err() {
        //     return Err("Couldn't decode base64 in SnsMetadata.logo".to_string());
        // }
        Ok(())
    }

    fn validate_url(&self) -> Result<(), String> {
        let url = self.url.as_ref().ok_or("Error: url must be specified")?;
        let field_name = "SnsMetadata.url";
        let max_length = 512;
        let min_length = 10;
        // // Check that the URL is a sensible length
        if url.len() > max_length {
            return Err(format!(
                "{field_name} must be less than {max_length} characters long, but it is {} characters long. (Field was set to `{url}`.)",
                url.len(),
            ));
        }
        if url.len() < min_length {
            return Err(format!(
                "{field_name} must be greater or equal to than {min_length} characters long, but it is {} characters long. (Field was set to `{url}`.)",
                url.len(),
            ));
        }

        //

        if !url.starts_with("https://") {
            return Err(format!(
                "{field_name} must begin with https://. (Field was set to `{url}`.)",
            ));
        }

        let parts_url: Vec<&str> = url.split("://").collect();
        if parts_url.len() > 2 {
            return Err(format!(
                "{field_name} contains an invalid sequence of characters"
            ));
        }

        if parts_url.len() < 2 {
            return Err(format!("{field_name} is missing content after protocol."));
        }

        if url.contains('@') {
            return Err(format!(
                "{field_name} cannot contain authentication information"
            ));
        }

        let parts_past_protocol = parts_url[1].split_once('/');

        let (_domain, _path) = match parts_past_protocol {
            Some((domain, path)) => (domain, Some(path)),
            None => (parts_url[1], None),
        };
        Ok(())
    }

    fn validate_name(&self) -> Result<(), String> {
        // The maximum number of characters allowed for a SNS name.
        const MAX_NAME_LENGTH: usize = 255;

        // The minimum number of characters allowed for a SNS name.
        const MIN_NAME_LENGTH: usize = 4;
        let name = self.name.as_ref().ok_or("Error: name must be specified")?;
        if name.len() > MAX_NAME_LENGTH {
            return Err(format!(
                "SnsMetadata.name must be less than {} characters",
                MAX_NAME_LENGTH
            ));
        } else if name.len() < MIN_NAME_LENGTH {
            return Err(format!(
                "SnsMetadata.name must be greater than {} characters",
                MIN_NAME_LENGTH
            ));
        }
        Ok(())
    }

    fn validate_description(&self) -> Result<(), String> {
        // The maximum number of characters allowed for a SNS description.
        const MAX_DESCRIPTION_LENGTH: usize = 2000;

        // The minimum number of characters allowed for a SNS description.
        const MIN_DESCRIPTION_LENGTH: usize = 10;
        let description = self
            .description
            .as_ref()
            .ok_or("Error: description must be specified")?;

        if description.len() > MAX_DESCRIPTION_LENGTH {
            return Err(format!(
                "SnsMetadata.description must be less than {} characters",
                MAX_DESCRIPTION_LENGTH
            ));
        } else if description.len() < MIN_DESCRIPTION_LENGTH {
            return Err(format!(
                "SnsMetadata.description must be greater than {} characters",
                MIN_DESCRIPTION_LENGTH
            ));
        }
        Ok(())
    }

    fn validate_initial_reward_rate_basis_points(&self) -> Result<(), String> {
        let initial_reward_rate_basis_points = self
            .initial_reward_rate_basis_points
            .ok_or("Error: initial_reward_rate_basis_points must be specified")?;
        if initial_reward_rate_basis_points
            > VotingRewardsParameters::INITIAL_REWARD_RATE_BASIS_POINTS_CEILING
        {
            Err(format!(
                "Error: initial_reward_rate_basis_points must be less than or equal to {}",
                VotingRewardsParameters::INITIAL_REWARD_RATE_BASIS_POINTS_CEILING
            ))
        } else {
            Ok(())
        }
    }

    fn validate_final_reward_rate_basis_points(&self) -> Result<(), String> {
        let initial_reward_rate_basis_points = self
            .initial_reward_rate_basis_points
            .ok_or("Error: initial_reward_rate_basis_points must be specified")?;
        let final_reward_rate_basis_points = self
            .final_reward_rate_basis_points
            .ok_or("Error: final_reward_rate_basis_points must be specified")?;
        if final_reward_rate_basis_points > initial_reward_rate_basis_points {
            Err(
                format!(
                    "Error: final_reward_rate_basis_points ({}) must be less than or equal to initial_reward_rate_basis_points ({})", final_reward_rate_basis_points,
                    initial_reward_rate_basis_points
                )
            )
        } else {
            Ok(())
        }
    }

    fn validate_reward_rate_transition_duration_seconds(&self) -> Result<(), String> {
        let _reward_rate_transition_duration_seconds = self
            .reward_rate_transition_duration_seconds
            .ok_or("Error: reward_rate_transition_duration_seconds must be specified")?;
        Ok(())
    }

    fn validate_max_dissolve_delay_seconds(&self) -> Result<(), String> {
        let _max_dissolve_delay_seconds = self
            .max_dissolve_delay_seconds
            .ok_or("Error: max_dissolve_delay_seconds must be specified")?;
        Ok(())
    }

    fn validate_max_neuron_age_seconds_for_age_bonus(&self) -> Result<(), String> {
        let _max_neuron_age_seconds_for_age_bonus = self
            .max_neuron_age_seconds_for_age_bonus
            .ok_or("Error: max_neuron_age_seconds_for_age_bonus must be specified")?;
        Ok(())
    }

    fn validate_max_dissolve_delay_bonus_percentage(&self) -> Result<(), String> {
        let max_dissolve_delay_bonus_percentage = self
            .max_dissolve_delay_bonus_percentage
            .ok_or("Error: max_dissolve_delay_bonus_percentage must be specified")?;

        if max_dissolve_delay_bonus_percentage
            > NervousSystemParameters::MAX_DISSOLVE_DELAY_BONUS_PERCENTAGE_CEILING
        {
            Err(format!(
                "max_dissolve_delay_bonus_percentage must be less than {}",
                NervousSystemParameters::MAX_DISSOLVE_DELAY_BONUS_PERCENTAGE_CEILING
            ))
        } else {
            Ok(())
        }
    }

    fn validate_max_age_bonus_percentage(&self) -> Result<(), String> {
        let max_age_bonus_percentage = self
            .max_age_bonus_percentage
            .ok_or("Error: max_age_bonus_percentage must be specified")?;
        if max_age_bonus_percentage > NervousSystemParameters::MAX_AGE_BONUS_PERCENTAGE_CEILING {
            Err(format!(
                "max_age_bonus_percentage must be less than {}",
                NervousSystemParameters::MAX_AGE_BONUS_PERCENTAGE_CEILING
            ))
        } else {
            Ok(())
        }
    }

    fn validate_initial_voting_period_seconds(&self) -> Result<(), String> {
        let initial_voting_period_seconds = self
            .initial_voting_period_seconds
            .ok_or("Error: initial_voting_period_seconds must be specified")?;

        if initial_voting_period_seconds
            < NervousSystemParameters::INITIAL_VOTING_PERIOD_SECONDS_FLOOR
        {
            Err(format!(
                "NervousSystemParameters.initial_voting_period_seconds must be greater than {}",
                NervousSystemParameters::INITIAL_VOTING_PERIOD_SECONDS_FLOOR
            ))
        } else if initial_voting_period_seconds
            > NervousSystemParameters::INITIAL_VOTING_PERIOD_SECONDS_CEILING
        {
            Err(format!(
                "NervousSystemParameters.initial_voting_period_seconds must be less than {}",
                NervousSystemParameters::INITIAL_VOTING_PERIOD_SECONDS_CEILING
            ))
        } else {
            Ok(())
        }
    }

    fn validate_wait_for_quiet_deadline_increase_seconds(&self) -> Result<(), String> {
        let wait_for_quiet_deadline_increase_seconds = self
            .wait_for_quiet_deadline_increase_seconds
            .ok_or("Error: wait_for_quiet_deadline_increase_seconds must be specified")?;
        let initial_voting_period_seconds = self
            .initial_voting_period_seconds
            .ok_or("Error: initial_voting_period_seconds must be specified")?;

        if wait_for_quiet_deadline_increase_seconds
            < NervousSystemParameters::WAIT_FOR_QUIET_DEADLINE_INCREASE_SECONDS_FLOOR
        {
            Err(format!(
                "NervousSystemParameters.wait_for_quiet_deadline_increase_seconds must be greater than or equal to {}",
                NervousSystemParameters::WAIT_FOR_QUIET_DEADLINE_INCREASE_SECONDS_FLOOR
            ))
        } else if wait_for_quiet_deadline_increase_seconds
            > NervousSystemParameters::WAIT_FOR_QUIET_DEADLINE_INCREASE_SECONDS_CEILING
        {
            Err(format!(
                "NervousSystemParameters.wait_for_quiet_deadline_increase_seconds must be less than or equal to {}",
                NervousSystemParameters::WAIT_FOR_QUIET_DEADLINE_INCREASE_SECONDS_CEILING
            ))
        // If `wait_for_quiet_deadline_increase_seconds > initial_voting_period_seconds / 2`, any flip (including an initial `yes` vote)
        // will always cause the deadline to be increased. That seems like unreasonable behavior, so we prevent that from being
        // the case.
        } else if wait_for_quiet_deadline_increase_seconds > initial_voting_period_seconds / 2 {
            Err(format!(
                "NervousSystemParameters.wait_for_quiet_deadline_increase_seconds is {}, but must be less than or equal to half the initial voting period, {}",
                initial_voting_period_seconds, initial_voting_period_seconds / 2
            ))
        } else {
            Ok(())
        }
    }

    fn validate_dapp_canisters(&self) -> Result<(), String> {
        let dapp_canisters = match &self.dapp_canisters {
            None => return Ok(()),
            Some(dapp_canisters) => dapp_canisters,
        };

        if dapp_canisters.canisters.len() > MAX_DAPP_CANISTERS_COUNT {
            return Err(format!(
                "Error: The number of dapp_canisters exceeded the maximum allowed canisters at \
                initialization. Count is {}. Maximum allowed is {}.",
                dapp_canisters.canisters.len(),
                MAX_DAPP_CANISTERS_COUNT,
            ));
        }

        for (index, canister) in dapp_canisters.canisters.iter().enumerate() {
            if canister.id.is_none() {
                return Err(format!("Error: dapp_canisters[{}] id field is None", index));
            }
        }

        // Disallow duplicate dapp canisters, because it indicates that
        // the user probably made a mistake (e.g. copy n' paste).
        let unique_dapp_canisters: BTreeSet<_> = dapp_canisters
            .canisters
            .iter()
            .map(|canister| canister.id)
            .collect();
        if unique_dapp_canisters.len() != dapp_canisters.canisters.len() {
            return Err("Error: Duplicate ids found in dapp_canisters".to_string());
        }

        // let nns_canisters = &[
        //     NNS_GOVERNANCE_CANISTER_ID,
        //     ICP_LEDGER_CANISTER_ID,
        //     REGISTRY_CANISTER_ID,
        //     ROOT_CANISTER_ID,
        //     CYCLES_MINTING_CANISTER_ID,
        //     LIFELINE_CANISTER_ID,
        //     GENESIS_TOKEN_CANISTER_ID,
        //     IDENTITY_CANISTER_ID,
        //     NNS_UI_CANISTER_ID,
        //     SNS_WASM_CANISTER_ID,
        //     EXCHANGE_RATE_CANISTER_ID,
        // ]
        // .map(PrincipalId::from);

        // let nns_canisters_listed_as_dapp = dapp_canisters
        //     .canisters
        //     .iter()
        //     .filter_map(|canister| {
        //         // Will not fail because of previous check
        //         let id = canister.id.unwrap();
        //         if nns_canisters.contains(&id) {
        //             Some(id)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect::<Vec<_>>();
        // if !nns_canisters_listed_as_dapp.is_empty() {
        //     return Err(format!(
        //         "Error: The following canisters are listed as dapp canisters, but are \
        //         NNS canisters: {:?}",
        //         nns_canisters_listed_as_dapp
        //     ));
        // }

        Ok(())
    }

    fn validate_confirmation_text(&self) -> Result<(), String> {
        if let Some(confirmation_text) = &self.confirmation_text {
            if MAX_CONFIRMATION_TEXT_BYTES < confirmation_text.len() {
                return Err(
                    format!(
                        "NervousSystemParameters.confirmation_text must be fewer than {} bytes, given bytes: {}",
                        MAX_CONFIRMATION_TEXT_BYTES,
                        confirmation_text.len(),
                    )
                );
            }
            let confirmation_text_length = confirmation_text.chars().count();
            if confirmation_text_length < MIN_CONFIRMATION_TEXT_LENGTH {
                return Err(
                    format!(
                        "NervousSystemParameters.confirmation_text must be greater than {} characters, given character count: {}",
                        MIN_CONFIRMATION_TEXT_LENGTH,
                        confirmation_text_length,
                    )
                );
            }
            if MAX_CONFIRMATION_TEXT_LENGTH < confirmation_text_length {
                return Err(
                    format!(
                        "NervousSystemParameters.confirmation_text must be fewer than {} characters, given character count: {}",
                        MAX_CONFIRMATION_TEXT_LENGTH,
                        confirmation_text_length,
                    )
                );
            }
        }
        Ok(())
    }

    fn validate_restricted_countries(&self) -> Result<(), String> {
        // if let Some(restricted_countries) = &self.restricted_countries {
        //     if restricted_countries.iso_codes.is_empty() {
        //         return RestrictedCountriesValidationError::EmptyList.into();
        //     }
        //     let num_items = restricted_countries.iso_codes.len();
        //     if CountryCode::num_country_codes() < num_items {
        //         return RestrictedCountriesValidationError::TooManyItems(
        //             restricted_countries.iso_codes.len(),
        //         )
        //         .into();
        //     }
        //     let mut unique_iso_codes = BTreeSet::<String>::new();
        //     for item in &restricted_countries.iso_codes {
        //         if CountryCode::for_alpha2(item).is_err() {
        //             return RestrictedCountriesValidationError::NotIsoCompliant(item.clone())
        //                 .into();
        //         }
        //         if !unique_iso_codes.insert(item.clone()) {
        //             return RestrictedCountriesValidationError::ContainsDuplicates(item.clone())
        //                 .into();
        //         }
        //     }
        // }
        Ok(())
    }

    fn validate_neuron_basket_construction_params(&self) -> Result<(), String> {
        let neuron_basket_construction_parameters = self
            .neuron_basket_construction_parameters
            .as_ref()
            .ok_or("Error: neuron_basket_construction_parameters must be specified")?;

        // Check that `NeuronBasket` dissolve delay does not exceed
        // the maximum dissolve delay.
        let max_dissolve_delay_seconds = self
            .max_dissolve_delay_seconds
            .ok_or("Error: max_dissolve_delay_seconds must be specified")?;
        // The maximal dissolve delay of a neuron from a basket created by
        // `NeuronBasketConstructionParameters::generate_vesting_schedule`
        // will equal `(count - 1) * dissolve_delay_interval_seconds`.
        let max_neuron_basket_dissolve_delay = neuron_basket_construction_parameters
            .count
            .saturating_sub(1_u64)
            .checked_mul(neuron_basket_construction_parameters.dissolve_delay_interval_seconds);
        if let Some(max_neuron_basket_dissolve_delay) = max_neuron_basket_dissolve_delay {
            if max_neuron_basket_dissolve_delay > max_dissolve_delay_seconds {
                return NeuronBasketConstructionParametersValidationError::ExceedsMaximalDissolveDelay(max_dissolve_delay_seconds)
                    .into();
            }
        } else {
            return NeuronBasketConstructionParametersValidationError::ExceedsU64.into();
        }
        if neuron_basket_construction_parameters.count < MIN_SNS_NEURONS_PER_BASKET {
            return NeuronBasketConstructionParametersValidationError::BasketSizeTooSmall.into();
        }
        if neuron_basket_construction_parameters.count > MAX_SNS_NEURONS_PER_BASKET {
            return NeuronBasketConstructionParametersValidationError::BasketSizeTooBig.into();
        }
        if neuron_basket_construction_parameters.dissolve_delay_interval_seconds < 1 {
            return NeuronBasketConstructionParametersValidationError::InadequateDissolveDelay
                .into();
        }
        Ok(())
    }

    fn validate_max_icp_e8s(&self) -> Result<(), String> {
        if self.max_icp_e8s.is_some() {
            return Err(
                "Error: max_icp_e8s cannot be specified now that Matched Funding is enabled"
                    .to_string(),
            );
        }

        Ok(())
    }

    fn validate_min_icp_e8s(&self) -> Result<(), String> {
        if self.min_icp_e8s.is_some() {
            return Err(
                "Error: min_icp_e8s cannot be specified now that Matched Funding is enabled"
                    .to_string(),
            );
        };

        Ok(())
    }

    /// Validates that swap participation-related parameters<sup>*</sup> pass the following checks:
    /// (1) All participation-related parameters are set.
    /// (2) All participation-related parameters are within expected constant lower/upper bounds.
    /// (3) Minimum is less than or equal to maximum for the same parameter.
    /// (4) One participation cannot exceed the maximum ICP amount that the swap can obtain.
    /// (5) No more than `MAX_DIRECT_ICP_CONTRIBUTION_TO_SWAP` may be collected from direct swap
    ///     participants.
    /// (6) If the minimum required number of participants participate each with the minimum
    ///     required amount of ICP, the maximum ICP amount that the swap can obtain is not exceeded.
    /// (7) Determines the smallest SNS neuron size is greated than the SNS ledger transaction fee.
    /// (8) Required ICP participation amount is big enough to ensure that all participants will
    ///     end up with enough SNS tokens to form the right number of SNS neurons (after paying for
    ///     the SNS ledger transaction fee to create each such SNS neuron).
    ///
    /// * -- In the context of this function, swap participation-related parameters include:
    /// - `min_direct_participation_icp_e8s` - Required ICP amount for the swap to succeed.
    /// - `max_direct_participation_icp_e8s` - Maximum ICP amount that the swap can obtain.
    /// - `min_participant_icp_e8s`          - Required ICP participation amount.
    /// - `max_participant_icp_e8s`          - Maximum ICP amount from one participant.
    /// - `min_participants`                 - Required number of *direct* participants for the swap
    ///                                        to succeed. This does not restrict the number of
    ///                                        *Neurons' Fund* participants.
    /// - `initial_token_distribution.swap_distribution.initial_swap_amount_e8s`
    ///                                      - How many SNS tokens will be distributed amoung all
    ///                                        the swap participants if the swap succeeds.
    /// - `neuron_basket_construction_parameters`
    ///                                      - How many SNS neurons will be created per participant.
    /// - `neuron_minimum_stake_e8s`         - Determines the smallest SNS neuron size.
    /// - `sns_transaction_fee_e8s`          - SNS ledger transaction fee, in particular, charged
    ///                                        for SNS neuron creation at swap finalization.
    fn validate_participation_constraints(&self) -> Result<(), String> {
        // (1)
        let min_direct_participation_icp_e8s = self
            .min_direct_participation_icp_e8s
            .ok_or("Error: min_direct_participation_icp_e8s must be specified")?;

        let max_direct_participation_icp_e8s = self
            .max_direct_participation_icp_e8s
            .ok_or("Error: max_direct_participation_icp_e8s must be specified")?;

        let min_participant_icp_e8s = self
            .min_participant_icp_e8s
            .ok_or("Error: min_participant_icp_e8s must be specified")?;

        let max_participant_icp_e8s = self
            .max_participant_icp_e8s
            .ok_or("Error: max_participant_icp_e8s must be specified")?;

        let min_participants = self
            .min_participants
            .ok_or("Error: min_participants must be specified")?;

        let initial_swap_amount_e8s = self
            .get_swap_distribution()
            .map_err(|_| "Error: the SwapDistribution must be specified")?
            .initial_swap_amount_e8s;

        let neuron_basket_construction_parameters_count = self
            .neuron_basket_construction_parameters
            .as_ref()
            .ok_or("Error: neuron_basket_construction_parameters must be specified")?
            .count;

        let neuron_minimum_stake_e8s = self
            .neuron_minimum_stake_e8s
            .ok_or("Error: neuron_minimum_stake_e8s must be specified")?;

        let sns_transaction_fee_e8s = self
            .transaction_fee_e8s
            .ok_or("Error: transaction_fee_e8s must be specified")?;

        // (2)
        if min_direct_participation_icp_e8s == 0 {
            return Err("Error: min_direct_participation_icp_e8s must be > 0".to_string());
        }
        if min_participant_icp_e8s == 0 {
            return Err("Error: min_participant_icp_e8s must be > 0".to_string());
        }
        if min_participants == 0 {
            return Err("Error: min_participants must be > 0".to_string());
        }
        // Needed as the SwapInit min_participants field is a `u32`.
        if min_participants > (u32::MAX as u64) {
            return Err(format!(
                "Error: min_participants cannot be greater than {}",
                u32::MAX
            ));
        }

        // (3)
        if max_direct_participation_icp_e8s < min_direct_participation_icp_e8s {
            return Err(format!(
                "Error: max_direct_participation_icp_e8s ({}) \
                 must be >= min_direct_participation_icp_e8s ({})",
                max_direct_participation_icp_e8s, min_direct_participation_icp_e8s
            ));
        }
        if max_participant_icp_e8s < min_participant_icp_e8s {
            return Err(format!(
                "Error: max_participant_icp_e8s ({}) must be >= min_participant_icp_e8s ({})",
                max_participant_icp_e8s, min_participant_icp_e8s
            ));
        }

        // (4)
        if max_participant_icp_e8s > max_direct_participation_icp_e8s {
            return Err(format!(
                "Error: max_participant_icp_e8s ({}) \
                 must be <= max_direct_participation_icp_e8s ({})",
                max_participant_icp_e8s, max_direct_participation_icp_e8s
            ));
        }

        // (5)
        if max_direct_participation_icp_e8s > MAX_DIRECT_ICP_CONTRIBUTION_TO_SWAP {
            return Err(format!(
                "Error: max_direct_participation_icp_e8s ({}) can be at most {} ICP E8s",
                max_direct_participation_icp_e8s, MAX_DIRECT_ICP_CONTRIBUTION_TO_SWAP
            ));
        }

        // (6)
        if max_direct_participation_icp_e8s
            < min_participants.saturating_mul(min_participant_icp_e8s)
        {
            return Err(format!(
                "Error: max_direct_participation_icp_e8s ({}) \
                 must be >= min_participants ({}) * min_participant_icp_e8s ({})",
                max_direct_participation_icp_e8s, min_participants, min_participant_icp_e8s
            ));
        }

        // (7)
        if neuron_minimum_stake_e8s <= sns_transaction_fee_e8s {
            return Err(format!(
                "Error: neuron_minimum_stake_e8s={} is too small. It needs to be \
                 greater than the transaction fee ({} e8s)",
                neuron_minimum_stake_e8s, sns_transaction_fee_e8s
            ));
        }

        // (8)
        let min_participant_sns_e8s = min_participant_icp_e8s as u128
            * initial_swap_amount_e8s as u128
            / max_direct_participation_icp_e8s as u128;

        let min_participant_icp_e8s_big_enough = min_participant_sns_e8s
            >= neuron_basket_construction_parameters_count as u128
                * (neuron_minimum_stake_e8s + sns_transaction_fee_e8s) as u128;

        if !min_participant_icp_e8s_big_enough {
            return Err(format!(
                "Error: min_participant_icp_e8s={} is too small. It needs to be \
                 large enough to ensure that participants will end up with \
                 enough SNS tokens to form {} SNS neurons, each of which \
                 require at least {} SNS e8s, plus {} e8s in transaction \
                 fees. More precisely, the following inequality must hold: \
                 min_participant_icp_e8s >= neuron_basket_count \
                 * (neuron_minimum_stake_e8s + transaction_fee_e8s) \
                 * max_direct_participation_icp_e8s / initial_swap_amount_e8s",
                min_participant_icp_e8s,
                neuron_basket_construction_parameters_count,
                neuron_minimum_stake_e8s,
                sns_transaction_fee_e8s,
            ));
        }

        Ok(())
    }

    fn validate_nns_proposal_id_pre_execution(&self) -> Result<(), String> {
        if self.nns_proposal_id.is_none() {
            Ok(())
        } else {
            Err(format!(
                "Error: nns_proposal_id cannot be specified pre_execution, but was {:?}",
                self.nns_proposal_id
            ))
        }
    }

    fn validate_nns_proposal_id(&self) -> Result<(), String> {
        match self.nns_proposal_id {
            None => Err("Error: nns_proposal_id must be specified".to_string()),
            Some(_) => Ok(()),
        }
    }

    fn validate_swap_start_timestamp_seconds_pre_execution(&self) -> Result<(), String> {
        if self.swap_start_timestamp_seconds.is_none() {
            Ok(())
        } else {
            Err(format!(
                "Error: swap_start_timestamp_seconds cannot be specified pre_execution, but was {:?}",
                self.swap_start_timestamp_seconds
            ))
        }
    }

    fn validate_swap_start_timestamp_seconds(&self) -> Result<(), String> {
        match self.swap_start_timestamp_seconds {
            Some(_) => Ok(()),
            None => Err("Error: swap_start_timestamp_seconds must be specified".to_string()),
        }
    }

    fn validate_swap_due_timestamp_seconds_pre_execution(&self) -> Result<(), String> {
        if self.swap_due_timestamp_seconds.is_none() {
            Ok(())
        } else {
            Err(format!(
                "Error: swap_due_timestamp_seconds cannot be specified pre_execution, but was {:?}",
                self.swap_due_timestamp_seconds
            ))
        }
    }

    fn validate_swap_due_timestamp_seconds(&self) -> Result<(), String> {
        let swap_start_timestamp_seconds = self
            .swap_start_timestamp_seconds
            .ok_or("Error: swap_start_timestamp_seconds must be specified")?;

        let swap_due_timestamp_seconds = self
            .swap_due_timestamp_seconds
            .ok_or("Error: swap_due_timestamp_seconds must be specified")?;

        if swap_due_timestamp_seconds < swap_start_timestamp_seconds {
            return Err(format!(
                "Error: swap_due_timestamp_seconds({}) must be after swap_start_timestamp_seconds({})",
                swap_due_timestamp_seconds, swap_start_timestamp_seconds,
            ));
        }

        Ok(())
    }

    pub fn validate_neurons_fund_participation(&self) -> Result<(), String> {
        if self.neurons_fund_participation.is_none() {
            return Err("SnsInitPayload.neurons_fund_participation must be specified".into());
        }
        Ok(())
    }

    pub fn validate_neurons_fund_participation_constraints(
        &self,
        is_pre_execution: bool,
    ) -> Result<(), String> {
        // This field must be set by NNS Governance at proposal execution time, not before.
        // This check will also catch the situation in which we are in the legacy (pre-1-prop) flow,
        // in which the `neurons_fund_participation_constraints`` field must not be set at all.
        if is_pre_execution && self.neurons_fund_participation_constraints.is_some() {
            return Result::from(
                NeuronsFundParticipationConstraintsValidationError::SetBeforeProposalExecution,
            );
        }

        let Some(ref neurons_fund_participation_constraints) =
            self.neurons_fund_participation_constraints
        else {
            if self.neurons_fund_participation == Some(true) && !is_pre_execution {
                return Result::from(NeuronsFundParticipationConstraintsValidationError::RelatedFieldUnspecified(
                    "neurons_fund_participation requires neurons_fund_participation_constraints"
                    .to_string(),
                ));
            }
            return Ok(());
        };

        // Validate relationship with min_direct_participation_threshold_icp_e8s
        let Some(min_direct_participation_threshold_icp_e8s) =
            neurons_fund_participation_constraints.min_direct_participation_threshold_icp_e8s
        else {
            return Result::from(NeuronsFundParticipationConstraintsValidationError::MinDirectParticipationThresholdValidationError(
                MinDirectParticipationThresholdValidationError::Unspecified
            ));
        };

        let min_direct_participation_icp_e8s =
            self.min_direct_participation_icp_e8s.ok_or_else(|| {
                NeuronsFundParticipationConstraintsValidationError::RelatedFieldUnspecified(
                    "min_direct_participation_icp_e8s".to_string(),
                )
                .to_string()
            })?;
        if min_direct_participation_threshold_icp_e8s < min_direct_participation_icp_e8s {
            return Result::from(NeuronsFundParticipationConstraintsValidationError::MinDirectParticipationThresholdValidationError(
                MinDirectParticipationThresholdValidationError::BelowSwapDirectIcpMin {
                    min_direct_participation_threshold_icp_e8s,
                    min_direct_participation_icp_e8s,
                }
            ));
        }
        let max_direct_participation_icp_e8s =
            self.max_direct_participation_icp_e8s.ok_or_else(|| {
                NeuronsFundParticipationConstraintsValidationError::RelatedFieldUnspecified(
                    "max_direct_participation_icp_e8s".to_string(),
                )
                .to_string()
            })?;
        if min_direct_participation_threshold_icp_e8s > max_direct_participation_icp_e8s {
            return Result::from(NeuronsFundParticipationConstraintsValidationError::MinDirectParticipationThresholdValidationError(
                MinDirectParticipationThresholdValidationError::AboveSwapDirectIcpMax {
                    min_direct_participation_threshold_icp_e8s,
                    max_direct_participation_icp_e8s,
                }
            ));
        }

        // Validate relationship with max_neurons_fund_participation_icp_e8s
        let Some(max_neurons_fund_participation_icp_e8s) =
            neurons_fund_participation_constraints.max_neurons_fund_participation_icp_e8s
        else {
            return Result::from(NeuronsFundParticipationConstraintsValidationError::MaxNeuronsFundParticipationValidationError(
                MaxNeuronsFundParticipationValidationError::Unspecified
            ));
        };

        let min_participant_icp_e8s = self.min_participant_icp_e8s.ok_or_else(|| {
            NeuronsFundParticipationConstraintsValidationError::RelatedFieldUnspecified(
                "min_participant_icp_e8s".to_string(),
            )
            .to_string()
        })?;
        if 0 < max_neurons_fund_participation_icp_e8s
            && max_neurons_fund_participation_icp_e8s < min_participant_icp_e8s
        {
            let max_neurons_fund_participation_icp_e8s =
                NonZeroU64::new(max_neurons_fund_participation_icp_e8s).unwrap();
            return Result::from(NeuronsFundParticipationConstraintsValidationError::MaxNeuronsFundParticipationValidationError(
                MaxNeuronsFundParticipationValidationError::BelowSingleParticipationLimit {
                    max_neurons_fund_participation_icp_e8s,
                    min_participant_icp_e8s,
                }
            ));
        }
        // Not more than 50% of total contributions should come from the Neurons' Fund.
        let max_direct_participation_icp_e8s =
            self.max_direct_participation_icp_e8s.ok_or_else(|| {
                NeuronsFundParticipationConstraintsValidationError::RelatedFieldUnspecified(
                    "max_direct_participation_icp_e8s".to_string(),
                )
                .to_string()
            })?;
        if max_neurons_fund_participation_icp_e8s > max_direct_participation_icp_e8s {
            return Result::from(NeuronsFundParticipationConstraintsValidationError::MaxNeuronsFundParticipationValidationError(
                MaxNeuronsFundParticipationValidationError::AboveSwapMaxDirectIcp {
                    max_neurons_fund_participation_icp_e8s,
                    max_direct_participation_icp_e8s,
                }
            ));
        }

        neurons_fund_participation_constraints
            .validate()
            .map_err(|err| {
                NeuronsFundParticipationConstraintsValidationError::Local(err).to_string()
            })
    }

    /// Checks that all parameters whose values can only be known after the CreateServiceNervousSystem proposal is executed are present.
    pub fn validate_all_post_execution_swap_parameters_are_set(&self) -> Result<(), String> {
        let mut missing_one_proposal_fields = vec![];
        if self.nns_proposal_id.is_none() {
            missing_one_proposal_fields.push("nns_proposal_id")
        }
        if self.swap_start_timestamp_seconds.is_none() {
            missing_one_proposal_fields.push("swap_start_timestamp_seconds")
        }
        if self.swap_due_timestamp_seconds.is_none() {
            missing_one_proposal_fields.push("swap_due_timestamp_seconds")
        }
        if self.min_direct_participation_icp_e8s.is_none() {
            missing_one_proposal_fields.push("min_direct_participation_icp_e8s")
        }
        if self.max_direct_participation_icp_e8s.is_none() {
            missing_one_proposal_fields.push("max_direct_participation_icp_e8s")
        }

        if missing_one_proposal_fields.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "Error in validate_all_post_execution_swap_parameters_are_set: The one-proposal \
                SNS initialization requires some SnsInitPayload parameters to be Some. But the \
                following fields were set to None: {}",
                missing_one_proposal_fields.join(", ")
            ))
        }
    }

    /// Checks that all parameters used by the one-proposal flow are present, except for those whose values can't be known before the CreateServiceNervousSystem proposal is executed.
    pub fn validate_all_non_legacy_pre_execution_swap_parameters_are_set(
        &self,
    ) -> Result<(), String> {
        let mut missing_one_proposal_fields = vec![];
        if self.min_participants.is_none() {
            missing_one_proposal_fields.push("min_participants")
        }

        if self.min_direct_participation_icp_e8s.is_none() {
            missing_one_proposal_fields.push("min_direct_participation_icp_e8s")
        }

        if self.max_direct_participation_icp_e8s.is_none() {
            missing_one_proposal_fields.push("max_direct_participation_icp_e8s")
        }
        if self.min_participant_icp_e8s.is_none() {
            missing_one_proposal_fields.push("min_participant_icp_e8s")
        }
        if self.max_participant_icp_e8s.is_none() {
            missing_one_proposal_fields.push("max_participant_icp_e8s")
        }
        if self.neuron_basket_construction_parameters.is_none() {
            missing_one_proposal_fields.push("neuron_basket_construction_parameters")
        }
        if self.dapp_canisters.is_none() {
            missing_one_proposal_fields.push("dapp_canisters")
        }
        if self.token_logo.is_none() {
            missing_one_proposal_fields.push("token_logo")
        }

        if missing_one_proposal_fields.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "Error in validate_all_non_legacy_pre_execution_swap_parameters_are_set: The one-\
                proposal SNS initialization requires some SnsInitPayload parameters to be Some. \
                But the following fields were set to None: {}",
                missing_one_proposal_fields.join(", ")
            ))
        }
    }
}
