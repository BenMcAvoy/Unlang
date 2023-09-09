use std::process::Command as SystemCommand;
use minifemme::{LevelFilter, LogMode};
use clap::{Arg, Command};
use std::path::PathBuf;
use std::process;
use std::env;
use std::fs;

fn check_file_exists(directory: &std::path::Path, filename: &str) -> (bool, Option<PathBuf>) {
    let mut current_dir = directory;

    loop {
        let file_path = current_dir.join(filename);
        if file_path.exists() {
            return (true, Some(file_path));
        }

        if let Some(parent) = current_dir.parent() {
            current_dir = parent;
        } else {
            break;
        }
    }

    (false, None)
}

fn main() {
    minifemme::start(LevelFilter::Trace, LogMode::Pretty);

    let matches = Command::new("Unpack")
        .about("Unlang package manager")
        .version("0.1.0")
        .subcommand(
            Command::new("project")
                .about("\"Unpack\" a new project.")
                .arg(
                    Arg::new("name")
                        .help("Project name.")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Run `main.ul`")
        )
        .get_matches();

    if let Some(project_matches) = matches.subcommand_matches("project") {
        let project_name = project_matches.get_one::<String>("name").unwrap();

        let path = PathBuf::from(project_name);

        if path.exists() {
            log::error!("Project directory already exists!");
            process::exit(-1);
        }

        if fs::create_dir_all(path.clone()).is_err() {
            log::error!("Failed to create the project. Do you have the correct permissions?");
            process::exit(-1);
        }

        if fs::create_dir_all(path.clone().join("src")).is_err() {
            log::error!("Failed to create the project. Do you have the correct permissions?");
            process::exit(-1);
        }

        let config = path.join("Unpack.toml");
        let main = path.join("src/main.ul");

        if fs::write(main, "return 0;\n").is_err() {
            log::error!("Failed to create `main.ul`");
        }

        if fs::write(config, "[package]\n").is_err() {
            log::error!("Failed to create `Unpack.toml`");
        }

        log::info!("Created new project.");
    }

    if let Some(_) = matches.subcommand_matches("run") {
        let current_dir = env::current_dir().expect("failed to get current directory");
        let result = check_file_exists(&current_dir, "Unpack.toml");

        if result.0 {
            let dir = match result.1 {
                Some(v) => v,
                None => {
                    log::error!("Failed to get directory of the config.");
                    process::exit(-1);
                },
            };

            let dir = match dir.parent() {
                Some(v) => v,
                None => {
                    log::error!("Failed to get directory of the config.");
                    process::exit(-1);
                }
            };

            let mut compiler = SystemCommand::new("unlang");
            compiler.arg("src/main.ul");
            compiler.current_dir(dir);

            let output = compiler.output().expect("Failed to run compiler.");

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("Compilation succeeded:\n{}", stdout);
            }
        }
    }
}

