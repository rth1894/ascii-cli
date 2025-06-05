use ascii_cli::{check, get_info};

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = match check(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = get_info(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
