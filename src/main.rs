mod countdown;
mod utils;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use chrono::{DateTime, Utc};

use countdown::Countdown;
use utils::str_to_datetime;

const SAVE_FILE: &str = ".croute-save.json";

fn load_countdowns() -> Vec<Countdown> {
    if let Ok(mut file) = File::open(SAVE_FILE) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(data) = serde_json::from_str(&contents) {
                return data;
            }
        }
    }
    Vec::new()
}

fn save_countdowns(countdowns: &Vec<Countdown>) {
    if let Ok(json) = serde_json::to_string_pretty(countdowns) {
        if let Ok(mut file) = File::create(SAVE_FILE) {
            let _ = file.write_all(json.as_bytes());
        }
    }
}

fn main() {
    const VERSION:&str = "0.1";

    let args: Vec<String> = env::args().skip(1).collect();

    let mut countdowns: Vec<Countdown> = load_countdowns();

    // If there is no command
    if args.len() == 0
    {
        println!("Croute");
        println!("Developper : martin0pl");
        println!("Programming language : Rust");
        println!("Version : {}",VERSION);
        println!("Github repository : https://github.com/martin0pl/croute");
    }
    else
    {
        // croute version
        if args.len() == 1 && args[0] == "version" {
            println!("Version : {}",VERSION);
        }
        // croute list
        if args.len() == 1 && args[0] == "list" {
            for countdown in countdowns {
                println!("{}", countdown.to_string());
            }
        }
        // croute new "countdown name" YYYY-MM-DD
        else if args.len() == 3 && args[0] == "new" {
            let title: String = args[1].clone();

            let date_str: String = args[2].clone() + " " + "00:00:00";
            let date: DateTime<Utc> = str_to_datetime(date_str);

            countdowns.push(Countdown::new(title, date));
            save_countdowns(&countdowns);
            println!("Countdown added!");
        }
        // croute new "countdown name" YYYY-MM-DD HH:MM:SS
        else if args.len() == 4 && args[0] == "new" {
            let title: String = args[1].clone();

            let date_str: String = args[2].clone() + " " + &args[3];
            let date: DateTime<Utc> = str_to_datetime(date_str);

            countdowns.push(Countdown::new(title, date));
            save_countdowns(&countdowns);
            println!("Countdown added!");
        }
    }
}
