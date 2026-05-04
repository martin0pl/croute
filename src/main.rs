mod countdown;

use std::env;

use countdown::Countdown;

fn main() {
    const VERSION:&str = "0.1";

    let args: Vec<String> = env::args().skip(1).collect();

    let countdowns: Vec<Countdown> = Vec::new();

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
    }
}
