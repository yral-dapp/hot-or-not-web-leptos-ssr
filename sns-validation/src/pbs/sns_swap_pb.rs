#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct CfParticipant {
    pub controller: ::core::option::Option<candid::Principal>,

    pub cf_neurons: Vec<CfNeuron>,

    #[deprecated]
    pub hotkey_principal: String,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct CfNeuron {
    pub nns_neuron_id: u64,

    pub amount_icp_e8s: u64,

    pub hotkeys: ::core::option::Option<crate::pbs::nns_pb::Principals>,

    pub has_created_neuron_recipes: ::core::option::Option<bool>,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct NeuronBasketConstructionParameters {
    pub count: u64,

    pub dissolve_delay_interval_seconds: u64,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct NeuronsFundParticipationConstraints {
    pub min_direct_participation_threshold_icp_e8s: ::core::option::Option<u64>,

    pub max_neurons_fund_participation_icp_e8s: ::core::option::Option<u64>,

    pub coefficient_intervals: Vec<LinearScalingCoefficient>,

    pub ideal_matched_participation_function:
        ::core::option::Option<IdealMatchedParticipationFunction>,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct IdealMatchedParticipationFunction {
    pub serialized_representation: ::core::option::Option<String>,
}

#[derive(candid::CandidType, candid::Deserialize, serde::Serialize, Eq)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug)]
pub struct LinearScalingCoefficient {
    pub from_direct_participation_icp_e8s: ::core::option::Option<u64>,

    pub to_direct_participation_icp_e8s: ::core::option::Option<u64>,

    pub slope_numerator: ::core::option::Option<u64>,

    pub slope_denominator: ::core::option::Option<u64>,

    pub intercept_icp_e8s: ::core::option::Option<u64>,
}
