use crate::home_dir;

use std::process::{exit, Command};

use clap::ArgMatches;

pub fn set_remote_url(sub_matches: &ArgMatches) {
    let remote_url = sub_matches
        .get_one::<String>("url")
        .expect("ERROR: Invalid url supplied.");

    match Command::new("git")
        .current_dir(home_dir())
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(remote_url)
        .spawn()
    {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!(
                "ERROR: Encountered the following error while trying to commit\n{}",
                e.to_string()
            );
            exit(1);
        }
    }
}
