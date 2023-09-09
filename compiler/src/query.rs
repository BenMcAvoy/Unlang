use std::io::{self, Write};

// TODO: Cleanup
pub fn query(message: &str) -> bool {
    let mut done = false;
    let mut accept = true;

    while !done {
        log::info!("{message} ");
        let mut buf = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();
        match buf.trim().to_lowercase().as_str() {
            "true" => done = true,
            "yes" => done = true,
            "y" => done = true,
            "false" => { done = true; accept = false; },
            "no" => { done = true; accept = false; },
            "n" => { done = true; accept = false; },
            _ => continue
        }
    }

    accept
}
