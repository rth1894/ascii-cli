# ascii-cli

Rust cli to convert images to ASCII art.

---

## Features Implemented (So Far)

- Print help menu (`--help`, `-h`, `-?`)  
- Print version information (`--version`, `-V`)  
- Parse the path of the input file to be converted  
- Implemented `lib.rs` to handle ASCII conversion
- Set a minimum width for ASCII art
- Error handling

---

## TODO (not in order)

- Tests
- Colored ASCII
- Package the cli
- Multithreading

---

## Usage
```bash
cargo run -- [OPTIONS] [FILE]
