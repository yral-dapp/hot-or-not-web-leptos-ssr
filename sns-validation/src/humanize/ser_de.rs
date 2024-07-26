use super::*;
use crate::pbs::nns_pb;
use serde::{ser::Error, Deserialize, Deserializer, Serializer};

pub mod tokens {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<nns_pb::Tokens, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: String = Deserialize::deserialize(deserializer)?;
        parse_tokens(&string).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(tokens: &nns_pb::Tokens, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if tokens.e8s.is_none() {
            return Err(S::Error::custom(
                "Unable to format Tokens, because e8s is blank.",
            ));
        }
        serializer.serialize_str(&format_tokens(tokens))
    }
}

pub mod duration {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<nns_pb::Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: String = Deserialize::deserialize(deserializer)?;
        parse_duration(&string).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(duration: &nns_pb::Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if duration.seconds.is_none() {
            return Err(S::Error::custom(
                "Unable to format Duration, because seconds is blank.",
            ));
        }
        serializer.serialize_str(&format_duration(duration))
    }
}

pub mod percentage {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<nns_pb::Percentage, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: String = Deserialize::deserialize(deserializer)?;
        parse_percentage(&string).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(percentage: &nns_pb::Percentage, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if percentage.basis_points.is_none() {
            return Err(S::Error::custom(
                "Unable to format Percentage, because basis_points is blank.",
            ));
        }
        serializer.serialize_str(&format_percentage(percentage))
    }
}

pub mod optional_tokens {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<nns_pb::Tokens>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: Option<String> = Deserialize::deserialize(deserializer)?;
        string
            .map(|string| parse_tokens(&string).map_err(serde::de::Error::custom))
            .transpose()
    }

    pub fn serialize<S>(tokens: &Option<nns_pb::Tokens>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match tokens {
            None => serializer.serialize_none(),
            Some(tokens) => tokens::serialize(tokens, serializer),
        }
    }
}

pub mod optional_time_of_day {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<nns_pb::GlobalTimeOfDay>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string: Option<String> = Deserialize::deserialize(deserializer)?;

        let string = match string {
            None => return Ok(None),
            Some(string) => string,
        };

        let global_time_of_day = parse_time_of_day(&string).map_err(serde::de::Error::custom)?;
        Ok(Some(global_time_of_day))
    }

    pub fn serialize<S>(
        time_of_day: &Option<nns_pb::GlobalTimeOfDay>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let time_of_day = match time_of_day.as_ref() {
            None => return serializer.serialize_none(),
            Some(time_of_day) => time_of_day,
        };

        // Input was Some -> format it (i.e. convert to String).
        if time_of_day.seconds_after_utc_midnight.is_none() {
            return Err(S::Error::custom(
                "Unable to format TimeOfDay, because seconds_after_utc_midnight is blank.",
            ));
        }
        let string = format_time_of_day(time_of_day);

        // The string needs to be wrapped in Some. Otherwise, the round trip is
        // going to be missing an Option layer: look at the first line of
        // deserialize: we try to get an Option<String> from deserializer.
        serializer.serialize_some(&Some(string))
    }
}
