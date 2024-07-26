/// Represents a Neurons' Fund participant, possibly with several neurons.
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct CfParticipant {
    /// The principal that can manage the NNS neuron that participated in the Neurons' Fund.
    pub controller: ::core::option::Option<candid::Principal>,
    /// Information about the participating neurons. Must not be empty.
    pub cf_neurons: Vec<CfNeuron>,
    /// The principal that can vote on behalf of these Neurons' Fund neurons.
    /// Deprecated. Please use `controller` instead (not `hotkeys`!)
    /// TODO(NNS1-3198): Remove
    #[deprecated]
    pub hotkey_principal: String,
}

/// Represents one NNS neuron from the Neurons' Fund participating in this swap.
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct CfNeuron {
    /// The NNS neuron ID of the participating neuron.
    pub nns_neuron_id: u64,
    /// The amount of ICP that the Neurons' Fund invests associated
    /// with this neuron.
    pub amount_icp_e8s: u64,
    /// The principals that can vote, propose, and follow on behalf of this neuron.
    pub hotkeys: ::core::option::Option<crate::pbs::nns_pb::Principals>,
    /// Idempotency flag indicating whether the neuron recipes have been created for
    /// the CfNeuron. When set to true, it signifies that the action of creating neuron
    /// recipes has been performed on this structure. If the action is retried, this flag
    /// can be checked to avoid duplicate operations.
    pub has_created_neuron_recipes: ::core::option::Option<bool>,
}

/// The construction parameters for the basket of neurons created for all
/// investors in the decentralization swap.
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NeuronBasketConstructionParameters {
    /// The number of neurons each investor will receive after the
    /// decentralization swap. The total tokens swapped for will be
    /// evenly distributed across the `count` neurons.
    pub count: u64,
    /// The amount of additional time it takes for the next neuron to dissolve.
    pub dissolve_delay_interval_seconds: u64,
}

/// Constraints for the Neurons' Fund participation in an SNS swap.
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct NeuronsFundParticipationConstraints {
    /// The Neurons' Fund will not participate in this swap unless the direct
    /// contributions reach this threshold (in ICP e8s).
    pub min_direct_participation_threshold_icp_e8s: ::core::option::Option<u64>,
    /// Maximum amount (in ICP e8s) of contributions from the Neurons' Fund to this swap.
    pub max_neurons_fund_participation_icp_e8s: ::core::option::Option<u64>,
    /// List of intervals in which the given linear coefficients apply for scaling the
    /// ideal Neurons' Fund participation amount (down) to the effective Neurons' Fund
    /// participation amount.
    pub coefficient_intervals: Vec<LinearScalingCoefficient>,
    /// The function used in the implementation of Matched Funding for mapping amounts of direct
    /// participation to "ideal" Neurons' Fund participation amounts. The value needs to be adjusted
    /// to a potentially smaller value due to SNS-specific participation constraints and
    /// the configuration of the Neurons' Fund at the time of the CreateServiceNervousSystem proposal
    /// execution.
    pub ideal_matched_participation_function:
        ::core::option::Option<IdealMatchedParticipationFunction>,
}

/// This function is called "ideal" because it serves as the guideline that the Neurons' Fund will
/// try to follow, but may deviate from in order to satisfy SNS-specific participation constraints
/// while allocating its overall participation amount among its neurons' maturity. In contrast,
/// The "effective" matched participation function `crate::neurons_fund::MatchedParticipationFunction`
/// is computed *based* on this one.
/// TODO(NNS1-1589): Until the Jira ticket gets solved, this definition needs to be synchronized with
/// that from nns/governance/proto/ic_nns_governance/pb/v1/governance.proto.
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct IdealMatchedParticipationFunction {
    /// The encoding of the "ideal" matched participation function is defined in `crate::neurons_fund`.
    /// In the future, we could change this message to represent full abstract syntactic trees
    /// comprised of elementary mathematical operators, with literals and variables as tree leaves.
    pub serialized_representation: ::core::option::Option<String>,
}
/// Some Neurons' Fund neurons might be too small, and some might be too large to participate in a
/// given SNS swap. This causes the need to adjust Neurons' Fund participation from an "ideal" amount
/// to an "effective" amount.
/// * The ideal-participation of the Neurons' Fund refers to the value dictated by some curve that
///    specifies how direct contributions should be matched with Neurons' Fund maturity.
/// * The effective-participation of the Neurons' Fund refers to the value that the NNS Governance
///    can actually allocate, given (1) the configuration of the Neurons' Fund at the time of
///    execution of the corresponding CreateServiceNervousSystem proposal and (2) the amount of direct
///    participation.
///
/// This structure represents the coefficients of a linear transformation used for
/// mapping the Neurons' Fund ideal-participation to effective-participation on a given
/// linear (semi-open) interval. Say we have the following function for matching direct
/// participants' contributions: `f: ICP e8s -> ICP e8s`; then the *ideal* Neuron's Fund
/// participation amount corresponding to the direct participation of `x` ICP e8s is
/// `f(x)`, while the Neuron's Fund *effective* participation amount is:
/// ```
/// g(x) = (c.slope_numerator / c.slope_denominator) * f(x) + c.intercept
/// ```
/// where `c: LinearScalingCoefficient` with
/// `c.from_direct_participation_icp_e8s <= x < c.to_direct_participation_icp_e8s`.
/// Note that we represent the slope as a rational number (as opposed to floating point),
/// enabling equality comparison between two instances of this structure.
#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct LinearScalingCoefficient {
    /// (Included) lower bound on the amount of direct participation (in ICP e8s) at which
    /// these coefficients apply.
    pub from_direct_participation_icp_e8s: ::core::option::Option<u64>,
    /// (Excluded) upper bound on the amount of direct participation (in ICP e8s) at which
    /// these coefficients apply.
    pub to_direct_participation_icp_e8s: ::core::option::Option<u64>,
    /// Numerator or the slope of the linear transformation.
    pub slope_numerator: ::core::option::Option<u64>,
    /// Denominator or the slope of the linear transformation.
    pub slope_denominator: ::core::option::Option<u64>,
    /// Intercept of the linear transformation (in ICP e8s).
    pub intercept_icp_e8s: ::core::option::Option<u64>,
}
