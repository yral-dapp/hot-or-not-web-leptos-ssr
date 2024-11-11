use candid::Principal;
use std::{fmt::Debug, str::FromStr};

use crate::{
    humanize,
    pbs::{
        gov_pb::CreateServiceNervousSystem,
        nns_pb::{self, Image},
        sns_pb::SnsInitPayload,
    },
};

// Alias CreateServiceNervousSystem-related types, but since we have many
// related types in this module, put these aliases in their own module to avoid
// getting mixed up.
mod nns_governance_pb {
    pub use crate::pbs::gov_pb::create_sns::{
        governance_parameters::VotingRewardParameters,
        initial_token_distribution::{
            developer_distribution::NeuronDistribution, DeveloperDistribution, SwapDistribution,
            TreasuryDistribution,
        },
        swap_parameters::NeuronBasketConstructionParameters,
        GovernanceParameters, InitialTokenDistribution, LedgerParameters, SwapParameters,
    };
}

// Implements the format used by test_sns_init_v2.yaml in the root of this
// package. Studying that is a much more ergonomic way of becoming familiar with
// the format that we are trying to implement here.
//
// (Thanks to the magic of serde, all the code here is declarative.)
#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnsConfigurationFile {
    pub name: String,
    pub description: String,
    pub logo_b64: String,
    pub url: String,

    #[serde(rename = "Principals", default)]
    pub principals: Vec<PrincipalAlias>,

    pub fallback_controller_principals: Vec<String>, // Principal (alias)
    pub dapp_canisters: Vec<String>,                 // Principal (alias)

    #[serde(rename = "Token")]
    pub token: Token,

    #[serde(rename = "Proposals")]
    pub proposals: Proposals,

    #[serde(rename = "Neurons")]
    pub neurons: Neurons,

    #[serde(rename = "Voting")]
    pub voting: Voting,

    #[serde(rename = "Distribution")]
    pub distribution: Distribution,

    #[serde(rename = "Swap")]
    pub swap: Swap,

    #[serde(rename = "NnsProposal")]
    pub nns_proposal: NnsProposal,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PrincipalAlias {
    id: String, // PrincipalId
    name: Option<String>,
    email: Option<String>,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    #[serde(with = "humanize::ser_de::tokens")]
    pub transaction_fee: nns_pb::Tokens,
    pub logo_b64: String,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Proposals {
    #[serde(with = "humanize::ser_de::tokens")]
    pub rejection_fee: nns_pb::Tokens,

    #[serde(with = "humanize::ser_de::duration")]
    pub initial_voting_period: nns_pb::Duration,

    #[serde(with = "humanize::ser_de::duration")]
    pub maximum_wait_for_quiet_deadline_extension: nns_pb::Duration,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Neurons {
    #[serde(with = "humanize::ser_de::tokens")]
    pub minimum_creation_stake: nns_pb::Tokens,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Voting {
    #[serde(with = "humanize::ser_de::duration")]
    pub minimum_dissolve_delay: nns_pb::Duration,

    #[serde(rename = "MaximumVotingPowerBonuses")]
    pub maximum_voting_power_bonuses: MaximumVotingPowerBonuses,

    #[serde(rename = "RewardRate")]
    pub reward_rate: RewardRate,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct MaximumVotingPowerBonuses {
    #[serde(rename = "DissolveDelay")]
    pub dissolve_delay: Bonus,

    #[serde(rename = "Age")]
    pub age: Bonus,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Bonus {
    #[serde(with = "humanize::ser_de::duration")]
    pub duration: nns_pb::Duration,

    #[serde(with = "humanize::ser_de::percentage")]
    pub bonus: nns_pb::Percentage,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RewardRate {
    #[serde(with = "humanize::ser_de::percentage")]
    pub initial: nns_pb::Percentage,

    #[serde(with = "humanize::ser_de::percentage")]
    pub r#final: nns_pb::Percentage,

    #[serde(with = "humanize::ser_de::duration")]
    pub transition_duration: nns_pb::Duration,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Swap {
    pub minimum_participants: u64,

    #[serde(default)]
    #[serde(with = "humanize::ser_de::optional_tokens")]
    pub minimum_icp: Option<nns_pb::Tokens>,
    #[serde(default)]
    #[serde(with = "humanize::ser_de::optional_tokens")]
    pub maximum_icp: Option<nns_pb::Tokens>,

    #[serde(default)]
    #[serde(with = "humanize::ser_de::optional_tokens")]
    pub minimum_direct_participation_icp: Option<nns_pb::Tokens>,
    #[serde(default)]
    #[serde(with = "humanize::ser_de::optional_tokens")]
    pub maximum_direct_participation_icp: Option<nns_pb::Tokens>,

    #[serde(with = "humanize::ser_de::tokens")]
    pub minimum_participant_icp: nns_pb::Tokens,
    #[serde(with = "humanize::ser_de::tokens")]
    pub maximum_participant_icp: nns_pb::Tokens,

    pub confirmation_text: Option<String>,
    pub restricted_countries: Option<Vec<String>>,

    #[serde(rename = "VestingSchedule")]
    pub vesting_schedule: VestingSchedule,

    #[serde(default)]
    #[serde(with = "humanize::ser_de::optional_time_of_day")]
    pub start_time: Option<nns_pb::GlobalTimeOfDay>,
    #[serde(with = "humanize::ser_de::duration")]
    pub duration: nns_pb::Duration,

    #[serde(default)]
    #[serde(with = "humanize::ser_de::optional_tokens")]
    pub neurons_fund_investment_icp: Option<nns_pb::Tokens>,

    #[serde(default)]
    pub neurons_fund_participation: Option<bool>,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct VestingSchedule {
    pub events: u64,

    #[serde(with = "humanize::ser_de::duration")]
    pub interval: nns_pb::Duration,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Distribution {
    #[serde(rename = "Neurons")]
    pub neurons: Vec<Neuron>,

    #[serde(rename = "InitialBalances")]
    pub initial_balances: InitialBalances,

    #[serde(with = "humanize::ser_de::tokens")]
    pub total: nns_pb::Tokens,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Neuron {
    pub principal: String, // Principal (alias)

    #[serde(with = "humanize::ser_de::tokens")]
    pub stake: nns_pb::Tokens,

    #[serde(default)]
    pub memo: u64,

    #[serde(with = "humanize::ser_de::duration")]
    pub dissolve_delay: nns_pb::Duration,

    #[serde(with = "humanize::ser_de::duration")]
    pub vesting_period: nns_pb::Duration,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct InitialBalances {
    #[serde(with = "humanize::ser_de::tokens")]
    pub governance: nns_pb::Tokens,

    #[serde(with = "humanize::ser_de::tokens")]
    pub swap: nns_pb::Tokens,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct NnsProposal {
    pub title: String,
    pub summary: String,
    pub url: Option<String>,
}

struct AliasToPrincipalId<'a> {
    #[allow(unused)]
    source: &'a Vec<PrincipalAlias>,
    /* TODO
    #[derive(Eq, PartialEq, Hash, Debug)]
    enum Key { // TODO: This name is just a placeholder.
        Name(String),
        Email(String),
    }

        alias_to_principal_id: HashMap<Key, PrincipalId>,
        */
}

impl<'a> AliasToPrincipalId<'a> {
    fn new(source: &'a Vec<PrincipalAlias>) -> Self {
        Self { source }
    }

    /// TODO: Currently, this just does PrincipalId::from_str, but real alias
    /// substitution is planned for a future MR.
    fn unalias(
        &self,
        field_name: &str,
        principals: &[String],
    ) -> Result<Vec<Principal>, Vec<String>> {
        let mut defects = vec![];

        let result = principals
            .iter()
            .map(|string| {
                Principal::from_str(string)
                    .map_err(|err| {
                        defects.push(format!(
                            "Unable to parse PrincipalId ({:?}) in {}. Reason: {}",
                            string, field_name, err,
                        ))
                    })
                    .unwrap_or(Principal::anonymous())
            })
            .collect();

        if !defects.is_empty() {
            return Err(defects);
        }

        Ok(result)
    }
}

impl SnsConfigurationFile {
    pub fn try_convert_to_create_service_nervous_system(
        &self,
    ) -> Result<CreateServiceNervousSystem, String> {
        // Step 1: Unpack.
        let SnsConfigurationFile {
            name,
            description,
            logo_b64,
            url,
            principals,
            fallback_controller_principals,
            dapp_canisters,
            token,
            proposals,
            neurons,
            voting,
            distribution,
            swap,
            nns_proposal: _, // We ignore the NNS Proposal fields
        } = self;

        // Step 2: Convert components.
        //
        // (This is the main section, where the "real" work takes place.)
        let alias_to_principal_id = AliasToPrincipalId::new(principals);
        let mut defects = vec![];

        // 2.1: Convert "primitive" typed fields.

        let name = Some(name.clone());
        let description = Some(description.clone());
        let url = Some(url.clone());

        // 2.2: Convert Vec fields.

        let fallback_controller_principal_ids = alias_to_principal_id
            .unalias(
                "fallback_controller_principals",
                fallback_controller_principals,
            )
            .map_err(|inner_defects| defects.extend(inner_defects))
            .unwrap_or_default();

        let dapp_canisters = alias_to_principal_id
            .unalias("dapp_canisters", dapp_canisters)
            .map_err(|inner_defects| defects.extend(inner_defects))
            .unwrap_or_default();

        // Wrap in Canister.
        let dapp_canisters = dapp_canisters
            .into_iter()
            .map(|principal_id| {
                let id = Some(principal_id);
                nns_pb::Canister { id }
            })
            .collect();

        // 2.3: Convert composite fields.
        let initial_token_distribution = Some(
            distribution
                .try_convert_to_initial_token_distribution()
                .map_err(|inner_defects| defects.extend(inner_defects))
                .unwrap_or_default(),
        );
        let swap_parameters = Some(swap.convert_to_swap_parameters());
        let ledger_parameters = Some(token.convert_to_ledger_parameters());
        let governance_parameters =
            Some(convert_to_governance_parameters(proposals, neurons, voting));

        // Step 3: Repackage.
        let result = CreateServiceNervousSystem {
            name,
            description,
            url,
            logo: Some(Image {
                base64_encoding: Some(logo_b64.clone()),
            }),

            fallback_controller_principal_ids,
            dapp_canisters,

            initial_token_distribution,
            swap_parameters,
            ledger_parameters,
            governance_parameters,
        };

        // Step 4: Validate.
        if !defects.is_empty() {
            return Err(format!(
                "Unable to convert configuration file to proposal for the following \
                 reason(s):\n  -{}",
                defects.join("\n  -"),
            ));
        }
        if let Err(err) = SnsInitPayload::try_from(result.clone()) {
            return Err(format!(
                "Unable to convert configuration file to proposal: {}",
                err,
            ));
        }

        // Step 5: Ship it!
        Ok(result)
    }

    pub fn try_convert_to_sns_init_payload(&self) -> Result<SnsInitPayload, String> {
        let create_nervous_system = self.try_convert_to_create_service_nervous_system()?;
        let now = web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut sns_init = SnsInitPayload::try_from(create_nervous_system)?;
        sns_init.nns_proposal_id = Some(1);
        sns_init.swap_start_timestamp_seconds = Some(now - 1000);
        sns_init.swap_due_timestamp_seconds = Some(now + 300);
        sns_init.validate_post_execution()?;

        Ok(sns_init)
    }
}

impl Distribution {
    fn try_convert_to_initial_token_distribution(
        &self,
    ) -> Result<nns_governance_pb::InitialTokenDistribution, Vec<String>> {
        let Distribution {
            neurons,
            initial_balances,
            total,
        } = self;

        let mut defects = vec![];
        // IDEALLY: Make Tokens support operators like +, -, and *. Ditto for
        // Duration, Percentage.
        let mut observed_total_e8s = 0;

        let developer_distribution =
            try_convert_from_neuron_vec_to_developer_distribution_and_total_stake(neurons)
                .map_err(|inner_defects| defects.extend(inner_defects))
                .unwrap_or_default();
        observed_total_e8s += developer_distribution
            .developer_neurons
            .iter()
            .map(|developer_neuron| {
                developer_neuron
                    .stake
                    .unwrap_or_default()
                    .e8s
                    .unwrap_or_default()
            })
            .sum::<u64>();
        let developer_distribution = Some(developer_distribution);

        let (treasury_distribution, swap_distribution) = {
            let InitialBalances { governance, swap } = initial_balances;

            observed_total_e8s += governance.e8s.unwrap_or_default();
            observed_total_e8s += swap.e8s.unwrap_or_default();

            (
                Some(nns_governance_pb::TreasuryDistribution {
                    total: Some(*governance),
                }),
                Some(nns_governance_pb::SwapDistribution { total: Some(*swap) }),
            )
        };

        // Validate total SNS tokens.
        if observed_total_e8s != total.e8s.unwrap_or_default() {
            defects.push(format!(
                "The total amount of SNS tokens was expected to be {}, but was instead {}.",
                humanize::format_tokens(total),
                humanize::format_tokens(&nns_pb::Tokens {
                    e8s: Some(observed_total_e8s),
                }),
            ));
        }

        if !defects.is_empty() {
            return Err(defects);
        }

        Ok(nns_governance_pb::InitialTokenDistribution {
            developer_distribution,
            treasury_distribution,
            swap_distribution,
        })
    }
}

fn try_convert_from_neuron_vec_to_developer_distribution_and_total_stake(
    original: &[Neuron],
) -> Result<nns_governance_pb::DeveloperDistribution, Vec<String>> {
    let mut defects = vec![];

    let developer_neurons = original
        .iter()
        .map(|neuron| {
            neuron
                .try_convert_to_neuron_distribution()
                .map_err(|inner_defects| defects.extend(inner_defects))
                .unwrap_or_default()
        })
        .collect();

    if !defects.is_empty() {
        return Err(defects);
    }

    Ok(nns_governance_pb::DeveloperDistribution { developer_neurons })
}

impl Neuron {
    fn try_convert_to_neuron_distribution(
        &self,
    ) -> Result<nns_governance_pb::NeuronDistribution, Vec<String>> {
        let Neuron {
            principal,
            stake,
            memo,
            dissolve_delay,
            vesting_period,
        } = self;

        let mut defects = vec![];

        let controller = Principal::from_str(principal)
            .map_err(|err| {
                defects.push(format!(
                    "Unable to parse PrincipalId in distribution.neurons ({:?}). \
                     err: {:#?}",
                    principal, err,
                ))
            })
            .unwrap_or(Principal::anonymous());
        let controller = Some(controller);

        let dissolve_delay = Some(*dissolve_delay);
        let memo = Some(*memo);
        let stake = Some(*stake);

        let vesting_period = Some(*vesting_period);

        if !defects.is_empty() {
            return Err(defects);
        }

        Ok(nns_governance_pb::NeuronDistribution {
            controller,
            dissolve_delay,
            memo,
            stake,
            vesting_period,
        })
    }
}

impl Token {
    fn convert_to_ledger_parameters(&self) -> nns_governance_pb::LedgerParameters {
        let Token {
            name,
            symbol,
            transaction_fee,
            logo_b64,
        } = self;

        let token_name = Some(name.clone());
        let token_symbol = Some(symbol.clone());
        let transaction_fee = Some(*transaction_fee);

        nns_governance_pb::LedgerParameters {
            token_name,
            token_symbol,
            transaction_fee,
            token_logo: Some(Image {
                base64_encoding: Some(logo_b64.clone()),
            }),
        }
    }
}

fn convert_to_governance_parameters(
    proposals: &Proposals,
    neurons: &Neurons,
    voting: &Voting,
) -> nns_governance_pb::GovernanceParameters {
    let Proposals {
        rejection_fee,
        initial_voting_period,
        maximum_wait_for_quiet_deadline_extension,
    } = proposals;
    let Neurons {
        minimum_creation_stake,
    } = neurons;
    let Voting {
        minimum_dissolve_delay,
        maximum_voting_power_bonuses,
        reward_rate,
    } = voting;
    let MaximumVotingPowerBonuses {
        dissolve_delay,
        age,
    } = maximum_voting_power_bonuses;

    let proposal_rejection_fee = Some(*rejection_fee);
    let proposal_initial_voting_period = Some(*initial_voting_period);
    let proposal_wait_for_quiet_deadline_increase =
        Some(*maximum_wait_for_quiet_deadline_extension);

    let neuron_minimum_stake = Some(*minimum_creation_stake);
    let neuron_minimum_dissolve_delay_to_vote = Some(*minimum_dissolve_delay);

    let (neuron_maximum_dissolve_delay, neuron_maximum_dissolve_delay_bonus) = {
        let Bonus { duration, bonus } = dissolve_delay;

        (Some(*duration), Some(*bonus))
    };

    let (neuron_maximum_age_for_age_bonus, neuron_maximum_age_bonus) = {
        let Bonus { duration, bonus } = age;

        (Some(*duration), Some(*bonus))
    };

    let voting_reward_parameters = Some(reward_rate.convert_to_voting_reward_parameters());

    nns_governance_pb::GovernanceParameters {
        proposal_rejection_fee,
        proposal_initial_voting_period,
        proposal_wait_for_quiet_deadline_increase,

        neuron_minimum_stake,

        neuron_minimum_dissolve_delay_to_vote,
        neuron_maximum_dissolve_delay,
        neuron_maximum_dissolve_delay_bonus,

        neuron_maximum_age_for_age_bonus,
        neuron_maximum_age_bonus,

        voting_reward_parameters,
    }
}

impl RewardRate {
    fn convert_to_voting_reward_parameters(&self) -> nns_governance_pb::VotingRewardParameters {
        let RewardRate {
            initial,
            r#final,
            transition_duration,
        } = self;

        let initial_reward_rate = Some(*initial);
        let final_reward_rate = Some(*r#final);
        let reward_rate_transition_duration = Some(*transition_duration);

        nns_governance_pb::VotingRewardParameters {
            initial_reward_rate,
            final_reward_rate,
            reward_rate_transition_duration,
        }
    }
}

impl Swap {
    fn convert_to_swap_parameters(&self) -> nns_governance_pb::SwapParameters {
        let Swap {
            minimum_participants,

            minimum_icp,
            maximum_icp,

            minimum_direct_participation_icp,
            maximum_direct_participation_icp,

            maximum_participant_icp,
            minimum_participant_icp,

            confirmation_text,
            restricted_countries,

            vesting_schedule,

            start_time,
            duration,
            neurons_fund_investment_icp,
            neurons_fund_participation,
        } = self;

        let minimum_participants = Some(*minimum_participants);

        let minimum_icp = *minimum_icp;
        let maximum_icp = *maximum_icp;

        let minimum_direct_participation_icp = minimum_direct_participation_icp
            .or_else(|| minimum_icp?.checked_sub(&neurons_fund_investment_icp.unwrap_or_default()));
        let maximum_direct_participation_icp = maximum_direct_participation_icp
            .or_else(|| maximum_icp?.checked_sub(&neurons_fund_investment_icp.unwrap_or_default()));

        let maximum_participant_icp = Some(*maximum_participant_icp);
        let minimum_participant_icp = Some(*minimum_participant_icp);

        let confirmation_text = confirmation_text.clone();
        let restricted_countries =
            restricted_countries
                .as_ref()
                .map(|restricted_countries| nns_pb::Countries {
                    iso_codes: restricted_countries.clone(),
                });

        let neuron_basket_construction_parameters =
            Some(vesting_schedule.convert_to_neuron_basket_construction_parameters());

        let start_time = *start_time;
        let duration = Some(*duration);

        let neurons_fund_participation = *neurons_fund_participation;

        nns_governance_pb::SwapParameters {
            minimum_participants,

            minimum_icp,
            maximum_icp,

            minimum_direct_participation_icp,
            maximum_direct_participation_icp,

            maximum_participant_icp,
            minimum_participant_icp,

            neuron_basket_construction_parameters,

            confirmation_text,
            restricted_countries,

            start_time,
            duration,

            neurons_fund_investment_icp: *neurons_fund_investment_icp,
            neurons_fund_participation,
        }
    }
}

impl VestingSchedule {
    fn convert_to_neuron_basket_construction_parameters(
        &self,
    ) -> nns_governance_pb::NeuronBasketConstructionParameters {
        let VestingSchedule { events, interval } = self;

        let count = Some(*events);
        let dissolve_delay_interval = Some(*interval);

        nns_governance_pb::NeuronBasketConstructionParameters {
            count,
            dissolve_delay_interval,
        }
    }
}
