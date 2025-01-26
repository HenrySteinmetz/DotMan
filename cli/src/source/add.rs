use std::{path::PathBuf, process::exit};

use clap::ArgMatches;

use crate::{
    config::LinkedFile,
    get_config_file_content,
    utils::{has_unique_elements, write_config},
};

pub fn add(sub_matches: &ArgMatches) {
    let path = sub_matches
        .get_one::<PathBuf>("path")
        .expect("ERROR: Invalid path supplied.");

    if !path.exists() {
        eprintln!("ERROR: Provided path does not exist.");
        exit(1);
    }

    let mut config_struct = get_config_file_content();
    config_struct
        .managed_paths
        .push(LinkedFile::new(path, None));

    if !has_unique_elements(config_struct.managed_paths.iter()) {
        eprintln!("ERROR: A config with the same path is already managed by DotMan.\nYou can print all managed configs by using the `source list` command.");
        exit(1);
    }

    write_config(&config_struct);
}
