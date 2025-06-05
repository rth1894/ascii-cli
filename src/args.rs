use std::process;

const VERSION: &str = "1.0.0";

pub struct Config {
    pub file: String,
    pub terminal: bool,
}

pub fn check(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
        return Err("In sufficient parameters!");
    }

    let command: String = args[1].clone();
    let mut terminal: bool = true;
    let mut file = String::new();

    if command == "--help" || command == "-h" || command == "-?"  {
        println!(
            "
            \rUsage: cargo run -- [OPTIONS] [FILE]

            \rArguments:
            \r\t[FILE]
            \r\t\tImage file to convert to ascii

            \r\t[OPTIONS]
            \r\t\t-h, --help\t\tPrint this help menu
            \r\t\t-t, --terminal\t\tPrint ASCII in terminal
            \r\t\t-f=PATH, --path=PATH\tPrint ASCII in specified PATH
            \r\t\t-V, --version\t\tPrint version

            \rExamples:
            \r\tcargo run --path=\"images/ascii-text.txt\" image.png
            \r\tcargo run --t image.png

            \rSee github.com/rth1894/ascii-cli for details");
        process::exit(0);
    }

    else if command.starts_with("-f=")|| command.starts_with("--path=") {
        let parts: Vec<&str> = command.split("=").collect();
        if parts.len() == 2 {
            file = parts[1].to_string();
            terminal = false;
        }
        else {
            return Err("No path specified!");
        }
    }

    else if command == "-V" || command == "--version" {
        println!("ascii-cli {}", VERSION);
    }

    Ok(Config { file, terminal }) 
}
