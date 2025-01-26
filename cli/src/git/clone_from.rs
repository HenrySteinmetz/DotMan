use clap::ArgMatches;
use std::process::{exit, Command};

pub fn clone_from(sub_matches: &ArgMatches) {
    let url = sub_matches
        .get_one::<String>("url")
        .expect("ERROR: Invalid URL supplied.");

    super::check_git();

    match Command::new("git").arg("clone").arg(url).spawn() {
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
