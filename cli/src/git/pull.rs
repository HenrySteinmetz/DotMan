use std::process::{exit, Command};

use crate::home_dir;

pub fn pull() {
    match Command::new("git")
        .current_dir(home_dir())
        .arg("pull")
        .spawn()
    {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("ERROR: Encountered the following error while trying to initialize the git repository:\n{}", e.to_string());
            exit(1);
        }
    }
}
