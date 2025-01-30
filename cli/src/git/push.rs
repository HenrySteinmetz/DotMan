use std::process::{exit, Command};

use crate::home_dir;

pub fn push() {
    match Command::new("git")
        .current_dir(home_dir())
        .arg("push origin master")
        .spawn()
    {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("ERROR: Encountered the following error while trying to push local git changes to repository:\n{}", e.to_string());
            exit(1);
        }
    }
}
