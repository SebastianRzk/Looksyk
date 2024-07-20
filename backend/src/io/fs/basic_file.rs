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

