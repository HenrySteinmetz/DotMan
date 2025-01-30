use std::{
    collections::HashSet,
    fs::{File, OpenOptions},
    hash::Hash,
    io::{Read, Write},
    path::PathBuf,
};

use walkdir::WalkDir;

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

pub fn home_dir() -> PathBuf {
    get_config_file_content().home_path
}

pub fn flat_file_array(path: PathBuf) -> Vec<PathBuf> {
    let mut ret = Vec::new();

    if path.is_dir() {
        for entry in WalkDir::new(path) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!(
                        "WARING: Failed to add file with error `{}. Skipping...`",
                        e.to_string()
                    );
                    continue;
                }
            };

            if entry.path().is_dir() {
                ret.extend(flat_file_array(entry.path().to_path_buf()).into_iter());
            } else {
                ret.push(entry.path().to_path_buf());
            }
        }
    }

    ret
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
