// From https://github.com/dfinity/ic/blob/master/rs/nervous_system/proto/src/gen/ic_nervous_system.pb.v1.rs

#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Duration {
    pub seconds: ::core::option::Option<u64>,
}
#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct GlobalTimeOfDay {
    pub seconds_after_utc_midnight: ::core::option::Option<u64>,
}
#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Tokens {
    pub e8s: ::core::option::Option<u64>,
}
#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Image {
    /// A data URI of a png. E.g.
    /// data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAAD0lEQVQIHQEEAPv/AAD/DwIRAQ8HgT3GAAAAAElFTkSuQmCC
    /// ^ 1 pixel containing the color #00FF0F.
    pub base64_encoding: ::core::option::Option<String>,
}
#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize, Copy, PartialOrd, Ord)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Percentage {
    pub basis_points: ::core::option::Option<u64>,
}
/// A list of principals.
/// Needed to allow prost to generate the equivalent of Optional<Vec<PrincipalId>>.
#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Principals {
    pub principals: Vec<candid::Principal>,
}
/// A Canister that will be transferred to an SNS.
#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize, Ord, PartialOrd, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Canister {
    /// The id of the canister.
    pub id: ::core::option::Option<candid::Principal>,
}
/// Represents a set of countries. To be used in country-specific configurations,
/// e.g., to restrict the geography of an SNS swap.
#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Countries {
    /// ISO 3166-1 alpha-2 codes
    pub iso_codes: Vec<String>,
}
/// Features:
///    1. Sign ('+' is optional).
///    2. Smallest positive value: 10^-28.
///    3. 96 bits of significand.
///    4. Decimal point character: '.' (dot/period).
#[derive(Eq, candid::CandidType, candid::Deserialize, serde::Serialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Decimal {
    /// E.g. "3.14".
    pub human_readable: ::core::option::Option<String>,
}

impl GlobalTimeOfDay {
    pub fn from_hh_mm(hh: u64, mm: u64) -> Result<Self, String> {
        if hh >= 23 || mm >= 60 {
            return Err(format!("invalid time of day ({}:{})", hh, mm));
        }
        let seconds_after_utc_midnight = Some(hh * 3600 + mm * 60);
        Ok(Self {
            seconds_after_utc_midnight,
        })
    }

    pub fn as_hh_mm(&self) -> Option<(u64, u64)> {
        let hh = self.seconds_after_utc_midnight? / 3600;
        let mm = (self.seconds_after_utc_midnight? % 3600) / 60;
        Some((hh, mm))
    }
}

impl Tokens {
    pub fn checked_add(&self, rhs: &Tokens) -> Option<Tokens> {
        let e8s = self.e8s?.checked_add(rhs.e8s?)?;
        Some(Tokens { e8s: Some(e8s) })
    }

    pub fn checked_sub(&self, rhs: &Tokens) -> Option<Tokens> {
        let e8s = self.e8s?.checked_sub(rhs.e8s?)?;
        Some(Tokens { e8s: Some(e8s) })
    }
}

impl Percentage {
    pub fn from_percentage(percentage: f64) -> Percentage {
        assert!(
            !percentage.is_sign_negative(),
            "percentage must be non-negative"
        );
        Percentage {
            basis_points: Some((percentage * 100.0).round() as u64),
        }
    }

    pub const fn from_basis_points(basis_points: u64) -> Percentage {
        Percentage {
            basis_points: Some(basis_points),
        }
    }
}
