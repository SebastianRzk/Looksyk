use std::fs;
use std::fs::Metadata;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> String {
    println!("loading file {}", path.to_str().unwrap());
    fs::read_to_string::<PathBuf>(path).unwrap()
}

pub fn read_binary_file(path: PathBuf) -> Vec<u8> {
    println!("loading file {}", path.to_str().unwrap());
    fs::read::<PathBuf>(path).unwrap()
}

pub fn read_metadata(path: PathBuf) -> Metadata {
    fs::metadata(path).unwrap()
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
    !(file_content.contains(&b'\x00') || file_content.contains(&b'\xff'))
}

pub fn delete_all_forbidden_chars_in_filename(filename: String) -> String {
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || c == &' ' || c == &'.' || c == &'-')
        .collect()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_null_byte_is_len_1() {
        let null_byte = "\0";
        assert_eq!(null_byte.as_bytes().len(), 1);
    }

    #[test]
    fn test_delete_all_forbidden_chars_in_filename() {
        assert_eq!(
            super::delete_all_forbidden_chars_in_filename("test\0".to_string()),
            "test"
        );
        assert_eq!(
            super::delete_all_forbidden_chars_in_filename("test\t".to_string()),
            "test"
        );
        assert_eq!(
            super::delete_all_forbidden_chars_in_filename("test.html".to_string()),
            "test.html"
        );
        assert_eq!(
            super::delete_all_forbidden_chars_in_filename("test-1".to_string()),
            "test-1"
        );
        assert_eq!(
            super::delete_all_forbidden_chars_in_filename("test 1".to_string()),
            "test 1"
        );
        assert_eq!(
            super::delete_all_forbidden_chars_in_filename("test[[1".to_string()),
            "test1"
        );
    }
}
