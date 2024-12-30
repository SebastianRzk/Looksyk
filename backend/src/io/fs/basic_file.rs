use std::fs;
use std::path::PathBuf;

const NULL_BYTE: &str = "\0";

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

pub fn folder_empty(path: PathBuf) -> bool {
    fs::metadata(path.clone()).is_ok()
        && fs::metadata(path.clone()).unwrap().is_dir()
        && fs::read_dir(path).unwrap().count() == 0
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

pub fn is_text_file(path: PathBuf) -> bool {
    let file_content = read_binary_file(path);
    !file_content.contains(&NULL_BYTE.as_bytes()[0])
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_null_byte_is_len_1() {
        let null_byte = "\0";
        assert_eq!(null_byte.as_bytes().len(), 1);
    }
}
