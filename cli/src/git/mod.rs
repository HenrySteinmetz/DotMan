mod clone_from;
mod commit;
mod init;
mod pull;
mod push;
mod set_remote_url;

use std::process::{exit, Command};

use clap::ArgMatches;

fn check_git() {
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
    check_git();
    match matches.subcommand() {
        Some(("clone_from", sub_matches)) => clone_from::clone_from(sub_matches),
        Some(("commit", sub_matches)) => commit::commit(sub_matches),
        Some(("init", _sub_matches)) => init::init(),
        Some(("pull", _sub_matches)) => pull::pull(),
        Some(("push", _sub_matches)) => push::push(),
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
