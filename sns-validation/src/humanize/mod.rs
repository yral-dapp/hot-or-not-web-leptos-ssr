pub mod ser_de;

use std::{collections::VecDeque, fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;

use crate::pbs::nns_pb;

// Normally, we would import this from ic_nervous_system_common, but we'd be
// dragging in lots of stuff along with it. The main problem with that is that
// any random change that requires ic_nervous_system_common to be rebuilt will
// also trigger a rebuild here. This gives us a "fire wall" to prevent fires
// from spreading.
//
// TODO(NNS1-2284): Move E8, and other such things to their own tiny library to
// avoid triggering mass rebuilds.
pub(crate) const E8: u64 = 100_000_000;

pub fn parse_tokens(s: &str) -> Result<nns_pb::Tokens, String> {
    let e8s = if let Some(s) = s.strip_suffix("tokens").map(|s| s.trim()) {
        parse_fixed_point_decimal(s, /* decimal_places = */ 8)?
    } else if let Some(s) = s.strip_suffix("token").map(|s| s.trim()) {
        parse_fixed_point_decimal(s, /* decimal_places = */ 8)?
    } else if let Some(s) = s.strip_suffix("e8s").map(|s| s.trim()) {
        u64::from_str(&s.replace('_', "")).map_err(|err| err.to_string())?
    } else {
        return Err(format!("Invalid tokens input string: {}", s));
    };
    let e8s = Some(e8s);

    Ok(nns_pb::Tokens { e8s })
}

pub fn parse_duration(s: &str) -> Result<nns_pb::Duration, String> {
    humantime::parse_duration(s)
        .map(|d| nns_pb::Duration {
            seconds: Some(d.as_secs()),
        })
        .map_err(|err| err.to_string())
}

pub fn parse_percentage(s: &str) -> Result<nns_pb::Percentage, String> {
    let number = s
        .strip_suffix('%')
        .ok_or_else(|| format!("Input string must end with a percent sign: {}", s))?;

    let basis_points = Some(parse_fixed_point_decimal(
        number, /* decimal_places = */ 2,
    )?);
    Ok(nns_pb::Percentage { basis_points })
}

pub fn parse_time_of_day(s: &str) -> Result<nns_pb::GlobalTimeOfDay, String> {
    const FORMAT: &str = "hh:mm UTC";
    let error = format!("Unable to parse time of day \"{s}\". Format should be \"{FORMAT}\"",);

    // decompose "hh:mm UTC" into ["hh:mm", "UTC"]
    let parts = s.split_whitespace().collect::<Vec<_>>();
    let [hh_mm, "UTC"] = &parts[..] else {
        return Err(error);
    };

    // decompose "hh:mm" into ["hh", "mm"]
    let parts = hh_mm.split(':').collect::<Vec<_>>();
    let [hh, mm] = &parts[..] else {
        return Err(error);
    };
    if hh.len() != 2 || mm.len() != 2 {
        return Err(error);
    }

    // convert ["hh", "mm"] into hh, mm
    let Ok(hh) = u64::from_str(hh) else {
        return Err(error);
    };
    let Ok(mm) = u64::from_str(mm) else {
        return Err(error);
    };

    nns_pb::GlobalTimeOfDay::from_hh_mm(hh, mm)
}

static FP_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x) # Verbose (ignore white space, and comments, like this).
    ^  # begin
    (?P<whole>[\d_]+)  # Digit or underscores (for grouping digits).
    (  # The dot + fractional part...
        [.]  # dot
        (?P<fractional>[\d_]+)
    )?  # ... is optional.
    $  # end
",
    )
    .unwrap()
});

fn parse_fixed_point_decimal(s: &str, decimal_places: usize) -> Result<u64, String> {
    let found = FP_REGEX
        .captures(s)
        .ok_or_else(|| format!("Not a number: {}", s))?;

    let whole = u64::from_str(
        &found
            .name("whole")
            .expect("Missing capture group?!")
            .as_str()
            .replace('_', ""),
    )
    .map_err(|err| err.to_string())?;

    let fractional = format!(
        // Pad so that fractional ends up being of length (at least) decimal_places.
        "{:0<decimal_places$}",
        found
            .name("fractional")
            .map(|m| m.as_str())
            .unwrap_or("0")
            .replace('_', ""),
    );
    if fractional.len() > decimal_places {
        return Err(format!("Too many digits after the decimal place: {}", s));
    }
    let fractional = u64::from_str(&fractional).map_err(|err| err.to_string())?;

    Ok(shift_decimal_right(whole, decimal_places)? + fractional)
}

fn shift_decimal_right<I>(n: u64, count: I) -> Result<u64, String>
where
    u32: TryFrom<I>,
    <u32 as TryFrom<I>>::Error: Display,
    I: Display + Copy,
{
    let count = u32::try_from(count)
        .map_err(|err| format!("Unable to convert {} to u32. Reason: {}", count, err))?;

    let boost = 10_u64
        .checked_pow(count)
        .ok_or_else(|| format!("Too large of an exponent: {}", count))?;

    n.checked_mul(boost)
        .ok_or_else(|| format!("Too large of a decimal shift: {} >> {}", n, count))
}

pub fn format_tokens(tokens: &nns_pb::Tokens) -> String {
    let nns_pb::Tokens { e8s } = tokens;
    let e8s = e8s.unwrap_or(0);

    if 0 < e8s && e8s < 1_000_000 {
        return format!("{} e8s", group_digits(e8s));
    }

    // TODO: format_fixed_point_decimal. parse_fixed_point_decimal seems
    // lonesome. But seriously, it can also be used in format_percentage.

    let whole = e8s / E8;
    let fractional = e8s % E8;

    let fractional = if fractional == 0 {
        "".to_string()
    } else {
        // TODO: Group.
        format!(".{:08}", fractional).trim_matches('0').to_string()
    };

    let units = if e8s == E8 { "token" } else { "tokens" };

    format!("{}{} {}", group_digits(whole), fractional, units)
}

pub fn format_duration(duration: &nns_pb::Duration) -> String {
    let nns_pb::Duration { seconds } = duration;
    let seconds = seconds.unwrap_or(0);

    humantime::format_duration(std::time::Duration::from_secs(seconds)).to_string()
}

pub fn format_percentage(percentage: &nns_pb::Percentage) -> String {
    let nns_pb::Percentage { basis_points } = percentage;
    let basis_points = basis_points.unwrap_or(0);

    let whole = basis_points / 100;
    let fractional = basis_points % 100;

    let fractional = if fractional == 0 {
        "".to_string()
    } else {
        format!(".{:02}", fractional).trim_matches('0').to_string()
    };

    format!("{}{}%", group_digits(whole), fractional)
}

pub fn format_time_of_day(time_of_day: &nns_pb::GlobalTimeOfDay) -> String {
    let (hours, minutes) = time_of_day.as_hh_mm().unwrap_or((0, 0));

    format!("{hours:02}:{minutes:02} UTC")
}

pub(crate) fn group_digits(n: u64) -> String {
    let mut left_todo = n;
    let mut groups = VecDeque::new();

    while left_todo > 0 {
        let group = left_todo % 1000;
        left_todo /= 1000;

        let group = if left_todo == 0 {
            format!("{}", group)
        } else {
            format!("{:03}", group)
        };

        groups.push_front(group);
    }

    if groups.is_empty() {
        return "0".to_string();
    }

    Vec::from(groups).join("_")
}
