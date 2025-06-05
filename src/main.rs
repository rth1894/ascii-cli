mod args;
use std::{env, process};
use args::{Config, check};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = check(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    println!("File: {}", config.file);
    println!("Terminal: {}", config.terminal);
}
