use crate::io::fs::media::LoadedMedia;
use sha3::{Digest, Sha3_256};
use std::io::Write;

pub fn hash_file_content(loaded_media: LoadedMedia) -> String {
    let mut hasher = Sha3_256::new();
    hasher.write_all(&loaded_media.content).unwrap();
    let mut result = String::new();

    hasher.finalize()[..]
        .to_ascii_lowercase()
        .iter()
        .for_each(|b| result.push_str(&format!("{b:02x}")));
    result
}
