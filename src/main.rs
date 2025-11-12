// Tell Rust to load the code from `src/models.rs` and `src/parser.rs`
mod models;
mod parser;

use std::env;
use std::fs;
use std::process;

use parser::parse_changelog;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: No changelog file provided.");
        eprintln!("Usage: {} <path/to/CHANGELOG.md>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    let content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };

    match parse_changelog(&content) {
        Ok(changelog) => {
            println!("Successfully parsed changelog!");
            println!("---");
            println!("{:#?}", changelog);
        }
        Err(e) => {
            eprintln!("Failed to parse changelog:");
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
