use std::path::PathBuf;

use clap::ArgMatches;

use crate::{get_config_file_content, write_config};

pub fn remove(sub_matches: &ArgMatches) {
    let path = sub_matches
        .get_one::<PathBuf>("path")
        .expect("ERROR: Invalid path supplied.");

    let mut config_struct = get_config_file_content();

    let index = config_struct
        .managed_paths
        .iter()
        .enumerate()
        .find(|x| x.1.source == *path);

    match index {
        Some((index, _)) => {
            config_struct.managed_paths.remove(index);
            write_config(&config_struct);
        }
        None => println!("WARNING: Could not find the requested path in config."),
    }
}
