use uts2ts::uts2ts;

/// Get day & month -> DD MMM format
/// where DD -> 2 digits
/// MMM -> String representing the month. i.e AUG for august
pub fn get_day_month(epoch_secs: u64) -> String {
    let ts = uts2ts(epoch_secs as i64);
    let month = match ts.month {
        1 => "JAN",
        2 => "FEB",
        3 => "MARCH",
        4 => "APRIL",
        5 => "MAY",
        6 => "JUNE",
        7 => "JULY",
        8 => "AUG",
        9 => "SEPT",
        10 => "OCT",
        11 => "NOV",
        12 => "DEC",
        _ => unreachable!(),
    };
    format!("{:02} {month}", ts.day)
}
