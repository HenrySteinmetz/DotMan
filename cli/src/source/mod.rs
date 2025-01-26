use std::process::exit;

use clap::ArgMatches;

mod add;
mod link;
mod list;
mod remove;
mod unlink;

pub fn source(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("list", _sub_matches)) => list::list(),
        Some(("add", sub_matches)) => add::add(sub_matches),
        Some(("remove", sub_matches)) => remove::remove(sub_matches),
        Some(("link", sub_matches)) => link::link(sub_matches),
        Some(("unlink", sub_matches)) => unlink::unlink(sub_matches),
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
