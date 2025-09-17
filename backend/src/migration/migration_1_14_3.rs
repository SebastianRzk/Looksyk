use crate::io::fs::basic_folder::{config_directory, home_directory};
use std::fs::{copy, create_dir_all};
use std::path::Path;

pub fn migriere_1_14_3() {
    let old_path = home_directory()
        .join(".local")
        .join("share")
        .join("looksyk")
        .join("config.json");
    let new_path = config_directory().join("config.json");
    if Path::new(&old_path).exists() && !Path::new(&new_path).exists() {
        create_dir_all(config_directory()).unwrap();
        copy(&old_path, &new_path).unwrap();
    }
}
