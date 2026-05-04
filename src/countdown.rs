use chrono::{DateTime, Utc, Duration};

pub struct Countdown {
    title: String,
    date: Option<DateTime<Utc>>
}

impl Countdown {
    pub fn new(title: String, date: Option<DateTime<Utc>>) -> Countdown {
        Self {
            title : title,
            date : date
        }
    }
}
