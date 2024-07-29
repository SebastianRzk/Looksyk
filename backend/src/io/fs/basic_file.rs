use std::fs;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> String {
    println!("loading file {}", path.to_str().unwrap());
    fs::read_to_string::<PathBuf>(path).unwrap()
}


pub fn read_binary_file(path: PathBuf) -> Vec<u8> {
    println!("loading file {}", path.to_str().unwrap());
    fs::read::<PathBuf>(path).unwrap()
}


pub fn exists_folder(path: PathBuf) -> bool {
    fs::metadata(path.clone()).is_ok() && fs::metadata(path).unwrap().is_dir()
}


pub fn exists_file(path: PathBuf) -> bool {
    fs::metadata(path.clone()).is_ok() && fs::metadata(path).unwrap().is_dir()
}

pub fn create_folder(path: PathBuf) {
    fs::create_dir_all(path).unwrap();
}