use std::{
    ffi::OsStr,
    fs::{create_dir, File},
    io::{Read, Write},
    path::PathBuf,
    process::exit,
};

use directories::BaseDirs;
use template_engine::TemplateEngine;

pub fn apply() {
    // Create list of output paths
    let mut destination_array: Vec<Option<PathBuf>> = Vec::new();

    // Create array of file paths and contents
    let mut content_array: Vec<(String, bool)> = Vec::new();

    let mut config_struct = crate::get_config_file_content();

    for (source, destination) in config_struct.paths_iter() {
        if !source.exists() {
            println!(
                "WARNING: Source file with path `{:#?}` does not exist. Skipping...",
                source
            );
            continue;
        }

        let mut source_content = String::new();

        let mut source_handle =
            File::open(&source).expect("ERROR: Failed to open source file, but path exists.");

        match source_handle.read_to_string(&mut source_content) {
            Ok(_) => (),
            Err(_) => {
                eprintln!(
                    "ERROR: Failed to read source file with path `{:#?}`, but path exists.",
                    source
                );
                exit(1);
            }
        }

        if source_content.is_empty() {
            println!(
                "WARNING: Source file at path `{:#?}` is empty. Skipping...",
                source
            );
            continue;
        }

        match destination {
            Some(destination) => {
                if destination.exists() {
                    println!(
                        "WARNING: Link destination with path `{:#?}` already exists. Skipping...",
                        destination
                    );
                    continue;
                }

                content_array.push((source_content, false));
                destination_array.push(Some(destination));
            }
            None => {
                if source.extension() != Some(OsStr::new("te")) {
                    eprintln!(
                        "WARNING: Config with path `{:#?}` has no link location. Skipping...",
                        source
                    );
                    continue;
                }

                content_array.push((source_content, true));
                destination_array.push(None);
            }
        }
    }
    // Parse all source files through the template engine
    let compiled_configs = match TemplateEngine::parse_files(content_array) {
        Ok(x) => x,
        Err(e) => {
            eprintln!(
                "ERROR: Received the following error while parsing a config:\n{}",
                e.to_string()
            );
            exit(1);
        }
    };

    if compiled_configs.len() != destination_array.len() {
        eprintln!("ERROR: Number of configs and destinations is not equal after templating.");
        exit(1);
    }

    // Check if data directory exists
    let base_dirs = match BaseDirs::new() {
        Some(x) => x,
        None => {
            eprintln!("ERROR: Failed to find local data directory.");
            exit(1);
        }
    };

    let data_dir = base_dirs.data_dir();

    let mut config_data_dir = data_dir.to_path_buf();
    config_data_dir.push("dotman");

    if !config_data_dir.exists() {
        match create_dir(&config_data_dir) {
            Ok(_) => println!(
                "WARNING: Data directory was not found. Created new directory at `{:#?}`",
                config_data_dir
            ),

            Err(e) => {
                eprintln!("ERROR: Data directory was not found. Failed to create a new directory at `{:#?}` with error `{:#?}`.", config_data_dir, e);
                exit(1);
            }
        }
    }

    // Create a file for each 'compiled' config at the desired location and write the content to it
    for (content, location) in compiled_configs
        .into_iter()
        .zip(destination_array.clone().into_iter())
    {
        if location.is_none() {
            continue;
        }

        let location = location.unwrap();

        if location.exists() {
            eprintln!("ERROR: Destination `{:#?} already exists.`", location);
            exit(1);
        }

        let mut file_handle = match File::create(&location) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "ERROR: Failed to create file at `{:#?}`, because of the following error:\n{}",
                    location,
                    e.to_string()
                );
                exit(1);
            }
        };

        match file_handle.write_all(content.as_bytes()) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("ERROR: Failed to write content to file at `{:#?}`, because of the following error:\n{}", location, e.to_string());
                exit(1);
            }
        }
    }

    config_struct
        .applied_paths
        .extend(destination_array.into_iter().filter_map(|x| x));

    println!("INFO: Succesfully applied your configs.");
}
