use crate::pbs::sns_swap_pb::{LinearScalingCoefficient, NeuronsFundParticipationConstraints};

// The maximum number of bytes that a serialized representation of an ideal matching function
// `IdealMatchedParticipationFunction` may have.
pub const MAX_MATCHING_FUNCTION_SERIALIZED_REPRESENTATION_SIZE_BYTES: usize = 1_000;

// The maximum number of intervals for scaling ideal Neurons' Fund participation down to effective
// participation. Theoretically, this number should be greater than double the number of neurons
// participating in the Neurons' Fund. Although the currently chosen value is quite high, it is
// still significantly smaller than `usize::MAX`, allowing to reject an misformed
// SnsInitPayload.coefficient_intervals structure with obviously too many elements.
pub const MAX_LINEAR_SCALING_COEFFICIENT_VEC_LEN: usize = 100_000;

#[derive(Debug)]
pub enum LinearScalingCoefficientValidationError {
    // All fields are mandatory.
    UnspecifiedField(String),
    EmptyInterval {
        from_direct_participation_icp_e8s: u64,
        to_direct_participation_icp_e8s: u64,
    },
    DenominatorIsZero,
    // The slope should be between 0.0 and 1.0.
    NumeratorGreaterThanDenominator {
        slope_numerator: u64,
        slope_denominator: u64,
    },
}

impl std::fmt::Display for LinearScalingCoefficientValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "LinearScalingCoefficientValidationError: ";
        match self {
            Self::UnspecifiedField(field_name) => {
                write!(f, "{prefix}Field `{}` must be specified.", field_name)
            }
            Self::EmptyInterval {
                from_direct_participation_icp_e8s,
                to_direct_participation_icp_e8s,
            } => {
                write!(
                    f,
                    "{prefix}from_direct_participation_icp_e8s ({}) must be strictly less that \
                    to_direct_participation_icp_e8s ({})).",
                    from_direct_participation_icp_e8s, to_direct_participation_icp_e8s,
                )
            }
            Self::DenominatorIsZero => {
                write!(f, "{prefix}slope_denominator must not equal zero.")
            }
            Self::NumeratorGreaterThanDenominator {
                slope_numerator,
                slope_denominator,
            } => {
                write!(
                    f,
                    "{prefix}slope_numerator ({}) must be less than or equal \
                    slope_denominator ({})",
                    slope_numerator, slope_denominator,
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedLinearScalingCoefficient {
    pub from_direct_participation_icp_e8s: u64,
    pub to_direct_participation_icp_e8s: u64,
    pub slope_numerator: u64,
    pub slope_denominator: u64,
    pub intercept_icp_e8s: u64,
}

#[derive(Debug)]
pub enum LinearScalingCoefficientVecValidationError {
    LinearScalingCoefficientsOutOfRange(usize),
    LinearScalingCoefficientsUnordered(
        ValidatedLinearScalingCoefficient,
        ValidatedLinearScalingCoefficient,
    ),
    IrregularLinearScalingCoefficients(ValidatedLinearScalingCoefficient),
    LinearScalingCoefficientValidationError(LinearScalingCoefficientValidationError),
}

impl std::fmt::Display for LinearScalingCoefficientVecValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "LinearScalingCoefficientVecValidationError: ";
        match self {
            Self::LinearScalingCoefficientsOutOfRange(num_elements) => {
                write!(
                    f,
                    "{}coefficient_intervals (len={}) must contain at least 1 and at most {} elements.",
                    prefix, num_elements, MAX_LINEAR_SCALING_COEFFICIENT_VEC_LEN,
                )
            }
            Self::LinearScalingCoefficientsUnordered(left, right) => {
                write!(
                    f,
                    "{}The intervals {:?} and {:?} are ordered incorrectly.",
                    prefix, left, right
                )
            }
            Self::IrregularLinearScalingCoefficients(interval) => {
                write!(
                    f,
                    "{}The first interval {:?} does not start from 0.",
                    prefix, interval,
                )
            }
            Self::LinearScalingCoefficientValidationError(error) => {
                write!(f, "{}{}", prefix, error)
            }
        }
    }
}

impl From<LinearScalingCoefficientVecValidationError> for Result<(), String> {
    fn from(value: LinearScalingCoefficientVecValidationError) -> Self {
        Err(value.to_string())
    }
}

#[derive(Debug)]
pub enum IdealMatchedParticipationFunctionValidationError {
    TooManyBytes(usize),
    DeserializationError {
        /// Value that could not be deserialized.
        input: String,
        /// Why deserialization did not work.
        err: String,
    },
}

impl std::fmt::Display for IdealMatchedParticipationFunctionValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "IdealMatchedParticipationFunctionValidationError: ";
        match self {
            Self::TooManyBytes(num_bytes) => {
                write!(
                    f,
                    "{prefix} serialized representation has {} bytes; the maximum is {} bytes.",
                    num_bytes, MAX_MATCHING_FUNCTION_SERIALIZED_REPRESENTATION_SIZE_BYTES,
                )
            }
            Self::DeserializationError { input, err } => {
                write!(
                    f,
                    "{prefix} deserialization failed: {}; input: `{}`.",
                    err, input
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum NeuronsFundParticipationConstraintsValidationError {
    RelatedFieldUnspecified(String),
    LinearScalingCoefficientVecValidationError(LinearScalingCoefficientVecValidationError),
    IdealMatchedParticipationFunctionValidationError(
        IdealMatchedParticipationFunctionValidationError,
    ),
}

impl std::fmt::Display for NeuronsFundParticipationConstraintsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "NeuronsFundParticipationConstraintsValidationError: ";
        match self {
            Self::RelatedFieldUnspecified(related_field_name) => {
                write!(f, "{}{} must be specified.", prefix, related_field_name,)
            }
            Self::LinearScalingCoefficientVecValidationError(error) => {
                write!(f, "{}{}", prefix, error)
            }
            Self::IdealMatchedParticipationFunctionValidationError(error) => {
                write!(f, "{prefix}{}", error)
            }
        }
    }
}

impl From<NeuronsFundParticipationConstraintsValidationError> for Result<(), String> {
    fn from(value: NeuronsFundParticipationConstraintsValidationError) -> Self {
        Err(value.to_string())
    }
}

#[derive(Clone, Debug)]
pub struct ValidatedNeuronsFundParticipationConstraints {
    // pub min_direct_participation_threshold_icp_e8s: u64,
    // pub max_neurons_fund_participation_icp_e8s: u64,
    // pub coefficient_intervals: Vec<ValidatedLinearScalingCoefficient>,
    // pub ideal_matched_participation_function: Box<F>,
}

impl NeuronsFundParticipationConstraints {
    /// Make the validation function available to crates that do not import
    /// `ValidatedNeuronsFundParticipationConstraints` directly, e.g., `rs/sns/init`.
    pub fn validate(&self) -> Result<(), NeuronsFundParticipationConstraintsValidationError> {
        ValidatedNeuronsFundParticipationConstraints::try_from(self).map(|_| ())
    }
}

impl TryFrom<&NeuronsFundParticipationConstraints>
    for ValidatedNeuronsFundParticipationConstraints
{
    type Error = NeuronsFundParticipationConstraintsValidationError;

    fn try_from(value: &NeuronsFundParticipationConstraints) -> Result<Self, Self::Error> {
        // Validate min_direct_participation_threshold_icp_e8s
        let _min_direct_participation_threshold_icp_e8s = value
            .min_direct_participation_threshold_icp_e8s
            .ok_or_else(|| {
                Self::Error::RelatedFieldUnspecified(
                    "min_direct_participation_threshold_icp_e8s".to_string(),
                )
            })?;

        // Validate max_neurons_fund_participation_icp_e8s
        let _max_neurons_fund_participation_icp_e8s = value
            .max_neurons_fund_participation_icp_e8s
            .ok_or_else(|| {
                Self::Error::RelatedFieldUnspecified(
                    "max_neurons_fund_participation_icp_e8s".to_string(),
                )
            })?;

        // Validate coefficient_intervals length.
        if !(1..MAX_LINEAR_SCALING_COEFFICIENT_VEC_LEN + 1)
            .contains(&value.coefficient_intervals.len())
        {
            return Err(Self::Error::LinearScalingCoefficientVecValidationError(
                LinearScalingCoefficientVecValidationError::LinearScalingCoefficientsOutOfRange(
                    value.coefficient_intervals.len(),
                ),
            ));
        }

        // Validate individual coefficient_intervals elements, consuming value.
        let coefficient_intervals: Vec<ValidatedLinearScalingCoefficient> = value
            .coefficient_intervals
            .iter()
            .map(ValidatedLinearScalingCoefficient::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| {
                Self::Error::LinearScalingCoefficientVecValidationError(
                LinearScalingCoefficientVecValidationError::LinearScalingCoefficientValidationError(err)
            )
            })?;

        // Validate that coefficient_intervals forms a partitioning.
        let intervals = &coefficient_intervals;
        intervals
            .iter()
            .zip(intervals.iter().skip(1))
            .find(|(prev, this)| {
                prev.to_direct_participation_icp_e8s != this.from_direct_participation_icp_e8s
            })
            .map_or(Ok(()), |(prev, this)| {
                Err(Self::Error::LinearScalingCoefficientVecValidationError(
                    LinearScalingCoefficientVecValidationError::LinearScalingCoefficientsUnordered(
                        prev.clone(),
                        this.clone(),
                    ),
                ))
            })?;

        // Validate that coefficient_intervals starts from 0.
        if let Some(first_interval) = intervals.first() {
            if first_interval.from_direct_participation_icp_e8s != 0 {
                return Err(Self::Error::LinearScalingCoefficientVecValidationError(
                    LinearScalingCoefficientVecValidationError::IrregularLinearScalingCoefficients(
                        first_interval.clone(),
                    ),
                ));
            }
        }

        let matching_function_serialized_representation = value
            .ideal_matched_participation_function
            .as_ref()
            .ok_or_else(|| {
                Self::Error::RelatedFieldUnspecified(
                    "ideal_matched_participation_function".to_string(),
                )
            })?
            .serialized_representation
            .as_ref()
            .ok_or_else(|| {
                Self::Error::RelatedFieldUnspecified(
                    "ideal_matched_participation_function.serialized_representation".to_string(),
                )
            })?;
        if matching_function_serialized_representation.len()
            > MAX_MATCHING_FUNCTION_SERIALIZED_REPRESENTATION_SIZE_BYTES
        {
            return Err(
                Self::Error::IdealMatchedParticipationFunctionValidationError(
                    IdealMatchedParticipationFunctionValidationError::TooManyBytes(
                        matching_function_serialized_representation.len(),
                    ),
                ),
            );
        }

        // TODO: add proper validation here
        // let ideal_matched_participation_function =
        //     F::from_repr(matching_function_serialized_representation).map_err(|err| {
        //         Self::Error::IdealMatchedParticipationFunctionValidationError(
        //             IdealMatchedParticipationFunctionValidationError::DeserializationError {
        //                 input: matching_function_serialized_representation.clone(),
        //                 err,
        //             },
        //         )
        //     })?;

        Ok(Self {
            // min_direct_participation_threshold_icp_e8s,
            // max_neurons_fund_participation_icp_e8s,
            // coefficient_intervals,
            // ideal_matched_participation_function,
        })
    }
}

impl TryFrom<&LinearScalingCoefficient> for ValidatedLinearScalingCoefficient {
    type Error = LinearScalingCoefficientValidationError;

    fn try_from(value: &LinearScalingCoefficient) -> Result<Self, Self::Error> {
        let from_direct_participation_icp_e8s =
            value.from_direct_participation_icp_e8s.ok_or_else(|| {
                LinearScalingCoefficientValidationError::UnspecifiedField(
                    "from_direct_participation_icp_e8s".to_string(),
                )
            })?;
        let to_direct_participation_icp_e8s =
            value.to_direct_participation_icp_e8s.ok_or_else(|| {
                LinearScalingCoefficientValidationError::UnspecifiedField(
                    "to_direct_participation_icp_e8s".to_string(),
                )
            })?;
        let slope_numerator = value.slope_numerator.ok_or_else(|| {
            LinearScalingCoefficientValidationError::UnspecifiedField("slope_numerator".to_string())
        })?;
        let slope_denominator = value.slope_denominator.ok_or_else(|| {
            LinearScalingCoefficientValidationError::UnspecifiedField(
                "slope_denominator".to_string(),
            )
        })?;
        // Currently we only check that `intercept_icp_e8s` is specified, so the actual field value
        // is unchecked.
        let intercept_icp_e8s = value.intercept_icp_e8s.ok_or_else(|| {
            LinearScalingCoefficientValidationError::UnspecifiedField(
                "intercept_icp_e8s".to_string(),
            )
        })?;
        if to_direct_participation_icp_e8s <= from_direct_participation_icp_e8s {
            return Err(LinearScalingCoefficientValidationError::EmptyInterval {
                from_direct_participation_icp_e8s,
                to_direct_participation_icp_e8s,
            });
        }
        if slope_denominator == 0 {
            return Err(LinearScalingCoefficientValidationError::DenominatorIsZero);
        }
        if slope_numerator > slope_denominator {
            return Err(
                LinearScalingCoefficientValidationError::NumeratorGreaterThanDenominator {
                    slope_numerator,
                    slope_denominator,
                },
            );
        }
        Ok(Self {
            from_direct_participation_icp_e8s,
            to_direct_participation_icp_e8s,
            slope_numerator,
            slope_denominator,
            intercept_icp_e8s,
        })
    }
}
