use std::io::Write;
use sha3::{Digest, Sha3_256};
use crate::io::fs::media::LoadedMedia;

pub fn hash_file_content(loaded_media: LoadedMedia) -> String{
    let mut hasher = Sha3_256::new();
    hasher.write(&loaded_media.content).unwrap();
    hasher.finalize()[..].to_ascii_lowercase().iter().map(|b| format!("{:02x}", b)).collect::<String>()
}