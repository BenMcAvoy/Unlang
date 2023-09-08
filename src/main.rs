use kv_log_macro as log;

use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), i32> {
    femme::with_level(femme::LevelFilter::Trace);

    let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();

    if args.len() != 2 {
        log::error!("Incorrect usage. Correct usage is:");
        log::error!("unlang <input.ul>");

        return Err(-1);
    }

    let path = Path::new(&args[1]);
    let contents = fs::read_to_string(path);

    Ok(())
}
