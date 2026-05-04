use std::env;

fn main() {
    const VERSION:&str = "0.1";

    let args: Vec<String> = env::args().skip(1).collect();
}
