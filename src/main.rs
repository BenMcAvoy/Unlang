use kv_log_macro as log;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    femme::with_level(femme::LevelFilter::Trace);

    let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();

    if args.len() != 2 {
        log::error!("Incorrect usage. Correct usage is:");
        log::error!("unlang <input.ul>");

        process::exit(-1);
    }

    let path = Path::new(&args[1]);
    let contents = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error reading file: {e}");
            process::exit(-1);
        }
    };

    println!("{contents}");
}
