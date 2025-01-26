use std::process::exit;

use crate::utils::get_config_file_content;

pub fn list() {
    let config_content = get_config_file_content();

    if config_content.managed_paths.len() == 0 {
        println!("INFO: There are currently no configs managed by DotMan.\nYou can add a config by using the `source add` command.");
        exit(0);
    }

    for (location, destination) in config_content.paths_iter() {
        match destination {
            Some(destination) => {
                println!(
                    "Location: {:#?} -> Destination: {:#?}",
                    location, destination
                )
            }
            None => println!("Location: {:#?}", location),
        }
    }
}
