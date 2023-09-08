// use kv_log_macro as log;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn usage() {
    log::error!("Incorrect usage. Correct usage is:");
    log::error!("unlang <input.ul>");

    process::exit(-1);
}

fn main() {
    minifemme::start(minifemme::LevelFilter::Trace, minifemme::LogMode::Pretty);

    let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();

    if args.len() != 2 {
        usage();
    }

    let path = Path::new(&args[1]);

    if let Some(extension) = path.extension() {
        if extension != "ul" {
            log::error!("File must end in `.ul`");
            usage();
        }
    }

    let contents = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error reading file: {e}");
            process::exit(-1);
        }
    };

    log::info!("Read file contents.");

    println!("{contents}");
}
