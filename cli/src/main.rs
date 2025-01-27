use clap::ArgMatches;

use std::{fs::File, io::Write, path::PathBuf, process::exit};

mod cli;
mod config;
mod git;
mod source;
mod utils;

use cli::cli;
use config::ConfigFile;
use git::git;
use source::source;
use utils::*;

fn main() {
    let config_file_path = config_file_path();

    if !config_file_path.exists() {
        println!("WARNING: No DotMan config found; creating default config.");

        let mut config_file = File::create_new(config_file_path)
            .expect("ERROR: Failed to create default config file. Possibly missing permissions.");
        // Safe unwrap: Data is known
        let default_config_string = toml::to_string(&ConfigFile::default()).unwrap();

        config_file
            .write_all(default_config_string.as_bytes())
            .expect("ERROR: Failed to write default config to file.");
    } else {
        let config_struct = get_config_file_content();

        if !config_struct.home_path.exists() {
            println!("WARNING: Configured home location does not exist.");
        }
    }

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("set_home", sub_matches)) => set_home(sub_matches),
        Some(("source", sub_matches)) => source(sub_matches),
        Some(("git", sub_matches)) => git(sub_matches),
        Some((subcommand, _)) => {
            eprintln!("Unknown subcommand {}", subcommand);
            exit(1);
        }
        None => {
            eprintln!("No subcommand provided");
            exit(1);
        }
    }
}

fn set_home(matches: &ArgMatches) {
    let path = matches
        .get_one::<PathBuf>("path")
        .expect("ERROR: Invalid path supplied");

    if !path.exists() {
        println!("WARNING: Provided home location does not exist.")
    }

    let mut config_struct = get_config_file_content();

    config_struct.home_path = path.to_path_buf();
    config_struct.git_init = false;

    write_config(&config_struct);

    println!("Succesfully changed the dotman home directory.");
    exit(0);
}
