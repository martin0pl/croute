use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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

    pub fn to_string(&self) -> String {
        return format!("{} : {}", self.title, self.get_time_left())
    }
}
