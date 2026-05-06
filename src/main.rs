mod countdown;
mod utils;

use std::env;
use chrono::{DateTime, Utc};

use countdown::Countdown;
use utils::str_to_datetime;

fn main() {
    const VERSION:&str = "0.1";

    let args: Vec<String> = env::args().skip(1).collect();

    let mut countdowns: Vec<Countdown> = Vec::new();

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
        // croute new "countdown name" YYYY-MM-DD
        else if args.len() == 3 && args[0] == "new" {
            let title: String = args[1].clone();

            let date_str: String = args[2].clone() + " " + "00:00:00";
            let date: DateTime<Utc> = str_to_datetime(date_str);

            countdowns.push(Countdown::new(title, date));
        }
        // croute new "countdown name" YYYY-MM-DD HH:MM:SS
        else if args.len() == 4 && args[0] == "new" {
            let title: String = args[1].clone();

            let date_str: String = args[2].clone() + " " + &args[3];
            let date: DateTime<Utc> = str_to_datetime(date_str);

            countdowns.push(Countdown::new(title, date));
        }
    }
}
