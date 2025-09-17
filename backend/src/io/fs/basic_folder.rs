use dirs::{config_dir, home_dir};
use std::path::PathBuf;

pub fn home_directory() -> PathBuf {
    home_dir().unwrap()
}

pub fn config_directory() -> PathBuf {
    let path = config_dir();
    if let Some(path) = path {
        path.join("looksyk")
    } else {
        home_directory()
            .join(".local")
            .join("share")
            .join("looksyk")
    }
}

pub fn documents_directory() -> PathBuf {
    let path = dirs::document_dir();
    if let Some(path) = path {
        path
    } else {
        home_directory().join("Documents")
    }
}
