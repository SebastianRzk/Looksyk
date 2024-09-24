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
    fs::metadata(path.clone()).is_ok() && fs::metadata(path).unwrap().is_file()
}

pub fn create_folder(path: PathBuf) {
    fs::create_dir_all(path).unwrap();
}


pub fn get_file_size(path: PathBuf) -> u64 {
    fs::metadata(path).unwrap().len()
}

pub fn delete_file(path: PathBuf) {
    fs::remove_file(path).unwrap();
}


pub fn is_text_file(path: PathBuf)-> bool {
    let file_content = read_binary_file(path);
    let mut is_text = true;
    for byte in file_content {
        if byte > 127 && (byte < 161 || byte > 191) && byte != 195 {
            println!("Found non text byte: {}", byte);
            is_text = false;
            break;
        }
    }
    is_text
}