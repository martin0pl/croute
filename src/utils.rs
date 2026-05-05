use chrono::{DateTime, Utc, TimeZone};

pub fn str_to_datetime(date_str: String) -> DateTime<Utc> {
    let format = "%Y-%m-%d %H:%M:%S";

    let naive_datetime = chrono::NaiveDateTime::parse_from_str(&date_str, format).expect("Format invalide");

    let datetime_utc: DateTime<Utc> = Utc.from_local_datetime(&naive_datetime).unwrap();

    return datetime_utc;
}
