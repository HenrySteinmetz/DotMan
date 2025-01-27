use crate::{get_config_file_content, home_dir, write_config};

use std::process::{exit, Command};

pub fn init() {
    let mut config_struct = get_config_file_content();

    if config_struct.git_init == true {
        eprintln!("ERROR: Git repository is already initialized.\nINFO: This can be the result of manual configuration.\nFIX: If you are sure that your dotman home folder is not a git repo then set the value of `git_init` to `false` in your dotman.toml file");
        exit(1);
    }

    config_struct.git_init = true;

    write_config(&config_struct);

    match Command::new("git")
        .current_dir(home_dir())
        .arg("init")
        .spawn()
    {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("ERROR: Encountered the following error while trying to initialize the git repository:\n{}", e.to_string());
            exit(1);
        }
    }
}
