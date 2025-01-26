use std::process::{exit, Command};

use clap::ArgMatches;

mod clone_from;
mod commit;
mod init;
mod pull;
mod push;
mod set_remote_url;

pub fn check_git() {
    match Command::new("git").spawn() {
        Ok(_) => (),
        Err(e) => {
            if let std::io::ErrorKind::NotFound = e.kind() {
                eprintln!("`git` command not found. Please install it");
                exit(1);
            } else {
                eprintln!(
                    "Encountered the following error while trying to execute git:\n{}",
                    e.to_string()
                );
                exit(1);
            }
        }
    }
}

pub fn git(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("clone_from", sub_matches)) => clone_from::clone_from(sub_matches),
        Some(("commit", sub_matches)) => commit::commit(sub_matches),
        Some(("init", sub_matches)) => init::init(sub_matches),
        Some(("pull", sub_matches)) => pull::pull(sub_matches),
        Some(("push", sub_matches)) => push::push(sub_matches),
        Some(("set_remote_url", sub_matches)) => set_remote_url::set_remote_url(sub_matches),
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
