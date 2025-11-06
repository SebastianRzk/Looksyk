use crate::io::fs::media::MediaOnDisk;
use std::collections::HashMap;

pub struct AssetCache {
    asset_cache: HashMap<String, AssetState>,
}

#[derive(Clone, PartialEq)]
pub struct AssetFileContent {
    pub content: String,
}

#[derive(Clone, PartialEq)]
pub enum AssetState {
    Miss,
    NotFound,
    TooLarge(FileSizeViolation),
    NotText,
    Found(AssetFileContent),
}

#[derive(Clone, PartialEq)]
pub struct FileSizeViolation {
    pub file_size: u64,
    pub max_size: u64,
}

impl Default for AssetCache {
    fn default() -> Self {
        Self::new()
    }
}

impl AssetCache {
    pub fn new() -> AssetCache {
        AssetCache {
            asset_cache: HashMap::new(),
        }
    }

    pub fn get(&self, media_on_disk: &MediaOnDisk) -> AssetState {
        if let Some(element) = self.asset_cache.get(media_on_disk.name.as_str()) {
            println!("Cache hit for {}", media_on_disk.name);
            return element.clone();
        }
        println!("Cache miss for {}", media_on_disk.name);
        AssetState::Miss
    }

    pub fn insert(&mut self, file_name: &MediaOnDisk, asset_state: AssetState) {
        self.asset_cache.insert(file_name.name.clone(), asset_state);
    }
}
