mod countdown;
mod utils;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use chrono::{DateTime, Utc, Duration};
use clap::{Parser, Subcommand};
use colored::Colorize;

use countdown::Countdown;
use utils::{str_to_datetime, sort_countdowns_by_date, format_duration};

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

        /// Date of the countdown (YYYY-MM-DD)
        date: String,

        /// Hour of the countdown (HH:MM:SS)
        #[arg(long, default_value_t = String::from("00:00:00"))]
        hour: String
    },
    /// List all your countdowns
    List {
        /// Show the index of the countdown
        #[arg(short, long)]
        index: bool
    },
    /// Delete countdown(s)
    #[command(group(
            clap::ArgGroup::new("delete_target")
                .required(true)
                .multiple(false)
    ))]
    Delete {
        /// Index of the countdown to delete
        #[arg(short, long, group = "delete_target")]
        index: Option<usize>,

        /// Delete all passed countdowns
        #[arg(short, long, group = "delete_target")]
        passed: bool
    },
    Info {
        index: usize
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
        Commands::List { index } => {
            if *index {
                for i in 0..countdowns.len() {
                    println!("{} - {}", i, countdowns[i].to_string());
                }
            }
            else {
                for countdown in countdowns {
                    println!("{}", countdown.to_string());
                }      
            }
        },
        Commands::Delete { index, passed } => {
            if *passed {
                let initial_len = countdowns.len();
    
                countdowns.retain(|c| c.get_time_left() >= Duration::zero());
    
                if countdowns.len() < initial_len {
                    save_countdowns(&countdowns, &save_file);
                    println!("{} passed countdowns deleted!", initial_len - countdowns.len());
                } else {
                    println!("No passed countdowns to delete.");
                }
            } 
            else if let Some(target_index) = index {

                if *target_index < countdowns.len() {
                    let title = countdowns[*target_index].get_title();
                    countdowns.remove(*target_index);
                    println!("Countdown '{}' deleted!", title.bold());
                    save_countdowns(&countdowns, &save_file);
                } else {
                    println!("Index out of range");
                }
                
            }
        },
        Commands::Info { index } => {
            if *index < countdowns.len() {
                println!("Title : {}", countdowns[*index].get_title());
                println!("Date : {}", countdowns[*index].get_date());
                println!("Time left : {}", format_duration(countdowns[*index].get_time_left()));
            } else {
                println!("Index out of range");
            }
        }
    }
}
