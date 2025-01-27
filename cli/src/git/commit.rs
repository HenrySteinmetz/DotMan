use crate::home_dir;

use std::process::{exit, Command};

use clap::ArgMatches;

pub fn commit(sub_matches: &ArgMatches) {
    let message = sub_matches
        .get_one::<String>("message")
        .expect("ERROR: Invalid commit message supplied.");

    match Command::new("git")
        .current_dir(home_dir())
        .arg("commit")
        .arg("-m")
        .arg(message)
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
