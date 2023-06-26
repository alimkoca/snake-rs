use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub enum EncryptionType {
    Aes,
    TripleDes,
    Blowfish,
    Rsa
}

#[derive(Serialize, Deserialize)]
pub enum ArchiveType {
    Zip,
    Rar,
    Tar,
    SevenZip,
    GunZip
}

#[derive(Serialize, Deserialize)]
struct PackageStructure {
    package_name: String,
    package_version: String,
    package_addr: HashMap<String, String>,
    encryption: EncryptionType,
    archiving: ArchiveType
}

impl PackageStructure {
    fn new() -> PackageStructure {
        PackageStructure {
            package_name: String::from(""),
            package_version: String::from(""),
            package_addr: HashMap::new(),
            encryption: EncryptionType::Rsa,
            archiving: ArchiveType::Zip
        }
    }
}