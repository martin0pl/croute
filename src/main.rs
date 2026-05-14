mod countdown;
mod utils;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use chrono::{DateTime, Utc, Duration};

use countdown::Countdown;
use utils::{str_to_datetime, sort_countdowns_by_date};

fn load_countdowns(save_file: &str) -> Vec<Countdown> {
    if let Ok(mut file) = File::open(save_file) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(data) = serde_json::from_str(&contents) {
                return data;
            }
        }
    }
    Vec::new()
}

fn save_countdowns(countdowns: &Vec<Countdown>, save_file: &str) {
    if let Ok(json) = serde_json::to_string_pretty(countdowns) {
        if let Ok(mut file) = File::create(save_file) {
            let _ = file.write_all(json.as_bytes());
        }
    }
}

fn main() {
    let home_dir = env::var("HOME").expect("Impossible to reach HOME directory");
    let save_file = format!("{}/.croute-save.json", home_dir);

    let mut countdowns: Vec<Countdown> = load_countdowns(&save_file);
}
