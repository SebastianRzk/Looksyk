use std::collections::HashMap;
use crate::io::fs::media::MediaOnDisk;

pub struct AssetCache {
    asset_cache: HashMap<String, AssetState>,
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct AssetFileContent {
    pub content: String,
}


#[derive(Clone)]
#[derive(PartialEq)]
pub enum AssetState {
    Miss,
    NotFound,
    TooLarge(FileSizeViolation),
    NotText,
    Found(AssetFileContent),
}

#[derive(Clone)]
#[derive(PartialEq)]
pub struct FileSizeViolation{
    pub file_size: u64,
    pub max_size: u64,
}


impl AssetCache {
    pub fn new() -> AssetCache {
        AssetCache {
            asset_cache: HashMap::new(),
        }
    }

    pub fn get(&self, media_on_disk: &MediaOnDisk) -> AssetState {
        if let Some(element) =self.asset_cache.get(media_on_disk.name.as_str()) {
            println!("Cache hit for {}", media_on_disk.name);
            return element.clone();
        }
        println!("Cache miss for {}", media_on_disk.name);
        return AssetState::Miss;
    }

    pub fn insert(&mut self, file_name: &MediaOnDisk, asset_state: AssetState) {
        self.asset_cache.insert(file_name.name.clone(), asset_state);
    }
}