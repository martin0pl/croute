use crate::utils::format_duration;

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
        if self.get_time_left() < Duration::zero() {
            return format!("{} {} ago", self.title, format_duration(self.get_time_left()))
        } 
        else {
            return format!("{} in {}", self.title, format_duration(self.get_time_left()))
        }
    }

    pub fn get_date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}
