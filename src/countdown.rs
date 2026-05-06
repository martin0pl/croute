use chrono::{DateTime, Utc, Duration};

pub struct Countdown {
    title: String,
    date: DateTime<Utc>
}

impl Countdown {
    pub fn new(title: String, date: DateTime<Utc>) -> Countdown {
        Self {
            title : title,
            date : date
        }
    }

    pub fn get_time_left(&self) -> Duration {
        return self.date - Utc::now();
    }
}
