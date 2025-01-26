use std::path::PathBuf;

use clap::ArgMatches;

use crate::{get_config_file_content, write_config};

pub fn unlink(sub_matches: &ArgMatches) {
    let path = sub_matches
        .get_one::<PathBuf>("path")
        .expect("ERROR: Invalid path supplied.");

    let mut config_struct = get_config_file_content();

    config_struct.managed_paths = config_struct
        .managed_paths
        .into_iter()
        .map(|mut linked_file| {
            if linked_file.source == *path {
                linked_file.destination = None;
            }
            linked_file
        })
        .collect();

    write_config(&config_struct);
}
