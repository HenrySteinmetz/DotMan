use std::{
    path::PathBuf,
    process::{exit, Command},
};

use clap::ArgMatches;

pub fn clone_from(sub_matches: &ArgMatches) {
    let url = sub_matches
        .get_one::<String>("url")
        .expect("ERROR: Invalid URL supplied.");

    let path = sub_matches
        .get_one::<PathBuf>("path")
        .expect("ERROR: Invalid URL supplied.");

    match Command::new("git").arg("clone").arg(url).arg(path).spawn() {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!(
                "Encountered the following error while trying to clone the repository:\n{}",
                e.to_string()
            );
            exit(1);
        }
    }
}
