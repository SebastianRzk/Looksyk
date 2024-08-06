use serde::{Deserialize, Serialize};

pub fn find_file(file_name: &String, media_index: &MediaIndex)-> Option<IndexedMedia> {
    for media in &media_index.media {
        if &media.file_name == file_name {
            return Some(media.clone());
        }
    }
    return None;
}

pub fn find_file_by_hash(file_hash: &String, media_index: &MediaIndex)-> Option<IndexedMedia> {
    for media in &media_index.media {
        if &media.sha3 == file_hash {
            return Some(media.clone());
        }
    }
    return None;
}


#[derive(Deserialize, Serialize, Clone)]
pub struct IndexedMedia {
    pub file_name: String,
    pub sha3: String,
}


pub struct MediaIndex {
    pub media: Vec<IndexedMedia>,
}