use serde::{Serialize, Deserialize};
use crate::package::{ArchiveType, EncryptionType};
use std::fs::{File, read_dir};
use std::io::prelude::*;

static REPO_CONFIG_FILE: &str = ".snake_repo";

#[derive(Serialize, Deserialize)]
struct RepoStructure {
    repo_name: String,
    repo_addr: String,
    repo_encryption: EncryptionType,
    repo_archive: ArchiveType
}

impl RepoStructure {
    fn new() -> RepoStructure{
        RepoStructure {
            repo_name: String::from(""),
            repo_addr: String::from(""),
            repo_encryption: EncryptionType::Rsa,
            repo_archive: ArchiveType::Zip
        }
    }

    fn scan_repo_config() -> Option<File> {
        let home_path = home::home_dir()
            .unwrap();
        let repo_conf = format!("{}/{REPO_CONFIG_FILE}", home_path.display());

        match File::open(repo_conf) {
            Ok(f) => Some(f),
            Err(_) => None
        }
    }

    pub fn read_repo_structure() -> RepoStructure {
        let mut repo_file = match RepoStructure::scan_repo_config() {
            Some(f) => f,
            None => panic!("Configuration file can't be found!")
        };
        let mut repo_body = String::new();
        let mut repo_struct = RepoStructure::new();

        repo_file.read_to_string(&mut repo_body).unwrap();
        repo_struct = serde_json::from_str(&repo_body).unwrap();

        repo_struct
    }
}