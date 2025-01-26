use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    pub home_path: PathBuf,
    pub managed_paths: Vec<LinkedFile>,
    pub remote_url: Option<String>,
    pub git_init: bool,
}

impl Default for ConfigFile {
    fn default() -> Self {
        let data_dir = directories::BaseDirs::new()
            .expect("ERROR: Could not find home or base directory")
            .data_local_dir()
            .to_path_buf();

        Self {
            home_path: data_dir,
            managed_paths: Vec::new(),
            remote_url: None,
            git_init: false,
        }
    }
}

impl ConfigFile {
    pub fn paths_iter(&self) -> impl Iterator<Item = (PathBuf, Option<PathBuf>)> + '_ {
        self.managed_paths
            .iter()
            .map(|x| (x.source.clone(), x.destination.clone()))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LinkedFile {
    pub source: PathBuf,
    pub destination: Option<PathBuf>,
}

impl LinkedFile {
    pub fn new<P: Into<PathBuf>>(source: P, destination: Option<P>) -> Self {
        let source = source.into();
        let destination = match destination {
            Some(x) => Some(x.into()),
            None => None,
        };

        Self {
            source,
            destination,
        }
    }
}
