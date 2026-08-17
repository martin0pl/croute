use chrono::{DateTime, Local, Duration};
use serde::{Serialize, Deserialize};
use colored::Colorize;

use crate::utils::format_duration;

#[derive(Serialize, Deserialize)]
pub struct Countdown {
    title: String,
    date: DateTime<Local>
}

impl Countdown {
    pub fn new(title: String, date: DateTime<Local>) -> Countdown {
        Self {
            title : title,
            date : date
        }
    }

    pub fn get_time_left(&self) -> Duration {
        return self.date - Local::now();
    }

    pub fn to_string(&self) -> String {
        if self.get_time_left() < Duration::zero() {
            return format!("{} {} ago", self.title.bold(), format_duration(self.get_time_left()))
        }
        else {
            return format!("{} in {}", self.title.bold(), format_duration(self.get_time_left()))
        }
    }

    pub fn get_date(&self) -> &DateTime<Local> {
        &self.date
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}
