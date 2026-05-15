mod countdown;
mod utils;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};

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

const DESCRIPTION: &str = "CLI tool to to manage countdowns anywhere on your device\nMade by martin0pl\nGithub : https://github.com/martin0pl/croute";

#[derive(Parser)]
#[command(name = "croute")]
#[command(version)]
#[command(about = DESCRIPTION, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new countdown
    New {
        /// Title of the countdown
        title: String,

        /// Date of the countdown
        date: String,

        /// Hour of the countdown
        #[arg(long, default_value_t = String::from("00:00:00"))]
        hour: String
    }
}

fn main() {
    let home_dir = env::var("HOME").expect("Impossible to reach HOME directory");
    let save_file = format!("{}/.croute-save.json", home_dir);

    let mut countdowns: Vec<Countdown> = load_countdowns(&save_file);

    let cli = Cli::parse();

    match &cli.command {
        Commands::New { title, date, hour } => {
            let date_str: String = date.to_owned() + " " + hour;
            let date_clean: DateTime<Utc> = str_to_datetime(date_str);

            countdowns.push(Countdown::new(title.to_string(), date_clean));

            sort_countdowns_by_date(&mut countdowns);

            save_countdowns(&countdowns, &save_file);

            println!("Countdown added!");
        },
    }
}
