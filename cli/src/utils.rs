use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    hash::Hash,
    io::{Read, Write},
    path::PathBuf,
};

use crate::config::ConfigFile;

pub fn config_file_path() -> PathBuf {
    let base_dirs =
        directories::BaseDirs::new().expect("ERROR: Could not find home or base directory");

    let config_dir = base_dirs.config_local_dir();

    let mut config_file_path = config_dir.to_path_buf();
    config_file_path.push("dotman.toml");

    config_file_path
}

pub fn get_config_file_content() -> ConfigFile {
    let mut file = File::open(config_file_path()).expect("ERROR: Failed to open config file.");
    let mut content = String::new();

    file.read_to_string(&mut content).expect("");

    toml::from_str(content.as_str()).expect("ERROR: Failed to parse config file.")
}

pub fn write_config(content: &ConfigFile) {
    let mut config_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_file_path())
        .expect("ERROR: Failed to open config file.");

    let new_content = toml::to_string(content).expect("ERROR: Failed to serialize config.");

    config_file
        .write_all(new_content.as_bytes())
        .expect("ERROR: Failed to write config to disk.");
}

pub fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
