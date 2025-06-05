pub struct Config {
    pub input_file: String,
    pub output_file: Option<String>,
    pub terminal: bool,
}

pub fn check(args: &[String]) -> Result<Config, &'static str> {
    const VERSION: &str = "0.0.1";

    if args.is_empty() {
        return Err("No arguments provided. Use --help for usage.");
    }

    let mut terminal = false;
    let mut input_file = None;
    let mut output_file = None;

    for command in args {
        if command == "--help" || command == "-h" || command == "-?" {
            println!(
                "
Usage: cargo run -- [OPTIONS] [FILE]

Arguments:
    [FILE]
        Image file to convert to ASCII

    [OPTIONS]
        -h, --help             Print this help menu
        -t, --terminal         Print ASCII in terminal
        -f=FILE, --file=FILE   Output ASCII art to specified text file
        -p=PATH, --path=PATH   Image path
        -V, --version          Print version

Examples:
    cargo run --file=ascii.txt --path=image.png
    cargo run -t --path=image.png

See github.com/rth1894/ascii-cli for details."
            );
            std::process::exit(0);
        } else if command == "-t" || command == "--terminal" {
            terminal = true;
        } else if command.starts_with("-f=") || command.starts_with("--file=") {
            let parts: Vec<&str> = command.split('=').collect();
            if parts.len() == 2 && !parts[1].is_empty() {
                output_file = Some(parts[1].to_string());
            } else {
                return Err("Output file path is missing after -f or --file.");
            }
        } else if command.starts_with("-p=") || command.starts_with("--path=") {
            let parts: Vec<&str> = command.split('=').collect();
            if parts.len() == 2 && !parts[1].is_empty() {
                input_file = Some(parts[1].to_string());
            } else {
                return Err("Input file path is missing after -p or --path.");
            }
        } else if command == "-V" || command == "--version" {
            println!("ascii-cli version {}", VERSION);
            std::process::exit(0);
        } else {
            return Err("Unknown argument provided. Use --help for usage.");
        }
    }

    if input_file.is_none() {
        return Err("Input file path not specified. Use -p=PATH or --path=PATH.");
    }

    Ok(Config {
        input_file: input_file.unwrap(),
        output_file,
        terminal,
    })
}
