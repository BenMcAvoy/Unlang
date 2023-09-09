use minifemme::{LevelFilter, LogMode};

use std::env;
use std::fs;
use std::path::Path;
use std::process;

use crate::asm::tokens_to_asm;
use crate::lexer::Lexer;

mod query;
mod lexer;
mod asm;

/// Print out the program usage
fn usage() {
    log::error!("Incorrect usage. Correct usage is:");
    log::error!("unlang <input.ul>");

    process::exit(-1);
}

fn main() {
    // Start logging
    minifemme::start(LevelFilter::Trace, LogMode::Pretty);

    // Collect arguments
    let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();

    // Check if right amount of arguments were provided
    if args.len() != 2 {
        usage();
    }

    // Create a path from arguments
    let path = Path::new(&args[1]);

    // Check if it ends in `ul`.
    if let Some(extension) = path.extension() {
        if extension != "ul" {
            log::error!("File must end in `.ul`");
            usage();
        }
    }

    // Read file contents
    let contents = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error reading file: {e}");
            process::exit(-1);
        }
    };

    let mut lexer = Lexer::new(contents);
    let asm = tokens_to_asm(lexer.tokenize());

    println!("{}", asm.unwrap());
}
