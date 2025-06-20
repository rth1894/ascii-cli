# ascii-cli

- Fast Rust CLI that converts images to ASCII art
- File export and future support for colorized output.

---

## Features

- Print help menu (`--help`, `-h`, `-?`)  
- Print version information (`--version`, `-V`)  
- Parse the path of the input file to be converted  
- Implemented `lib.rs` to handle ASCII conversion
- Error handling
- Clean interface with help/version flags.
- Converts images into text-based ASCII representation.
- Export to a file (`--file`) or display directly in the terminal.
- Ensures readable output regardless of input image size.
- Handles failures for invalid paths, unsupported formats, and missing args.

- `lib.rs`  - handles image to ascii conversion
- `main.rs` - handles errors
- `args.rs` - handles arguments and errors

---

## Usage

### Run with Cargo (development mode)
```bash
cargo run -- [OPTIONS] [FILE]
cargo run -- --help
cargo run -- --file=ascii.txt --path=image.png
cargo run -t --path=image.png
```

- `--path` - path to your input image
- `--file` - path where the output should be saved (required only if terminal flag not set)
- `--t`    - used to print ascii to terminal (`--file` not needed here)

---

## Installation

### Install with Cargo
After installing Rust, run:
```bash
cargo install --path .
```
This compiles and installs the binaries locally.

### Cargo Binary Path
- Linux/MacOS: `~/.cargo/bin/ascii-cli`
- Windows:     `%USERPROFILE%\.cargo\bin\ascii-cli.exe`

Add the directory to your system's `PATH` so you can use `ascii-cli` globally.
