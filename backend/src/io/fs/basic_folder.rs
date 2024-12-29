use dirs::home_dir;
use std::path::PathBuf;

pub fn home_directory() -> PathBuf {
    home_dir().unwrap()
}
