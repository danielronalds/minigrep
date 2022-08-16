use std::{env, process};

use minigrep::Config;

fn main() {
    // Collecting the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Assigning arguments to variables
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");

        process::exit(1);
    }
}
