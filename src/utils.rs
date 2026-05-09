use chrono::{DateTime, Utc, TimeZone, Duration};
use crate::countdown::Countdown;

pub fn str_to_datetime(date_str: String) -> DateTime<Utc> {
    let format = "%Y-%m-%d %H:%M:%S";

    let naive_datetime = chrono::NaiveDateTime::parse_from_str(&date_str, format).expect("Invalid format");

    let datetime_utc: DateTime<Utc> = Utc.from_local_datetime(&naive_datetime).unwrap();

    return datetime_utc;
}

pub fn format_duration(d: Duration) -> String {
    let mut remaining_seconds = d.num_seconds().abs();

    let sec_per_year = 365 * 24 * 3600;
    let sec_per_month = 30 * 24 * 3600;
    let sec_per_day = 24 * 3600;
    let sec_per_hour = 3600;
    let sec_per_minute = 60;

    let years = remaining_seconds / sec_per_year;
    remaining_seconds %= sec_per_year;

    let months = remaining_seconds / sec_per_month;
    remaining_seconds %= sec_per_month;

    let days = remaining_seconds / sec_per_day;
    remaining_seconds %= sec_per_day;

    let hours = remaining_seconds / sec_per_hour;
    remaining_seconds %= sec_per_hour;

    let minutes = remaining_seconds / sec_per_minute;
    let seconds = remaining_seconds % sec_per_minute;

    let mut parts = Vec::new();

    if years > 0 { parts.push(format!("{}year{}", years, if years > 1 { "s" } else { "" })); }
    if months > 0 { parts.push(format!("{}month{}", months, if months > 1 { "s" } else { "" })); }
    if days > 0 { parts.push(format!("{}d", days)); }
    if hours > 0 { parts.push(format!("{}h", hours)); }
    if minutes > 0 { parts.push(format!("{}min", minutes)); }
    if seconds > 0 || parts.is_empty() { parts.push(format!("{}s", seconds)); }

    format!("{}", parts.join(" "))
}

pub fn sort_countdowns_by_date(countdowns: &mut Vec<Countdown>) {
    countdowns.sort_by_key(|c| *c.get_date());
}
