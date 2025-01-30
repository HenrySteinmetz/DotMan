use clap::ArgMatches;
use std::{path::PathBuf, process::exit};

use crate::{get_config_file_content, utils::write_config};

pub fn link(sub_matches: &ArgMatches) {
    let source_path = sub_matches
        .get_one::<PathBuf>("source_path")
        .expect("ERROR: Invalid path supplied.");

    let destination_path = sub_matches
        .get_one::<PathBuf>("destination_path")
        .expect("ERROR: Invalid path supplied.");

    if !source_path.exists() {
        eprintln!("ERROR: Source path does not exist.");
        exit(1);
    }

    if destination_path.exists() {
        eprintln!("ERROR: Destination path already exists.");
        exit(1);
    }

    let mut config_struct = get_config_file_content();

    let mut was_modified = false;

    config_struct.managed_paths = config_struct
        .managed_paths
        .into_iter()
        .filter_map(|mut linked_file| {
            if linked_file.source == *source_path {
                if was_modified {
                    eprintln!("ERROR: Source path `{:#?}` is used mulitple times.\nPlease remove the duplicates.\nThe first occurence of the path was linked.", source_path);
                    exit(1);
                }
                if linked_file.destination.is_some() {
                    println!("WARNING: Source file already had a link  that now changed.");
                }
                was_modified = true;
                linked_file.destination = Some(destination_path.to_path_buf());
            }

            Some(linked_file)
        })
        .collect();

    if was_modified == false {
        println!("WARNING: The config was not changed.\nThe provided path is not managed by DotMan.\nYou can add it by using the `source add` subcommand.");
    } else {
        write_config(&config_struct);
    }
}
