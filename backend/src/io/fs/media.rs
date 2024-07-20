use std::path::{Path, PathBuf};
use chrono::Utc;
use crate::io::fs::basic_file::{read_binary_file, read_file};
use crate::io::fs::paths::{REL_MEDIA_CONFIG_PATH, REL_MEDIA_LOCATION};
use crate::io::hash::hash_file_content;
use crate::looksyk::index::media::{find_file, IndexedMedia, MediaIndex};
use crate::state::DataRootLocation;

pub fn read_media_config(data_root_location: &DataRootLocation) -> MediaIndex {
    let config_file_content_as_str = read_file(media_config_path(data_root_location));
    let json: Vec<IndexedMedia> = serde_json::from_str(config_file_content_as_str.as_str()).unwrap();
    return MediaIndex {
        media: json
    };
}

pub fn write_media_config(data_root_location: &DataRootLocation, media_index: &MediaIndex) {
    let config_file_content_as_str = serde_json::to_string(&media_index.media).unwrap();
    std::fs::write(media_config_path(&data_root_location), config_file_content_as_str).unwrap();
}

fn media_config_path(data_path: &DataRootLocation) -> PathBuf {
    Path::new(data_path.path.as_str()).join(REL_MEDIA_CONFIG_PATH)
}

pub fn init_media(data_root_location: &DataRootLocation, current_media_index: &MediaIndex) -> MediaIndex {
    let all_files_in_folder = read_all_media_files(data_root_location);
    let mut result_index: Vec<IndexedMedia> = vec![];

    for file in all_files_in_folder {
        let media_name = create_media_name(&file);
        let file_in_index = find_file(&media_name, current_media_index);
        if file_in_index.is_some() {
            result_index.push(file_in_index.clone().unwrap().clone());
        } else {
            println!("Add media object to index: {}", media_name.as_str());
            result_index.push(IndexedMedia {
                relative_path: media_name,
                sha3: create_hash(file),
            });
        }
    }

    MediaIndex {
        media: result_index,
    }
}

fn create_media_name(file: &MediaOnDisk) -> String {
    file.location.to_str().unwrap().to_string()
}

pub fn destination_path(filename: &str, data_root_location: &DataRootLocation) -> PathBuf {
    let timestamp = Utc::now().format("%Y_%m_%d_%H_%M_%S").to_string();

    let (filestem, file_ending) = parse_name(filename);

    let filename = format!("{}_{}.{}", filestem, timestamp, file_ending);
    Path::new(data_root_location.path.as_str()).join(REL_MEDIA_LOCATION).join(filename)
}

fn parse_name(filename: &str) -> (String, String) {
    if !filename.contains(".") {
        return (filename.to_string(), "".to_string())
    }

    let parsed_filename = filename.split(".").collect::<Vec<&str>>();
    let filestem = &parsed_filename[0..parsed_filename.len() - 1].join(".").to_string();

    let last_index = parsed_filename.len() - 1;
    let file_ending = &parsed_filename[last_index];

    (filestem.clone(), file_ending.to_string())
}

pub fn create_hash(file: MediaOnDisk) -> String {
    let file_conent = read_binary_file(file.location);
    hash_file_content(LoadedMedia {
        content: file_conent,
    })
}


pub fn read_all_media_files(data_root_location: &DataRootLocation) -> Vec<MediaOnDisk> {
    let media_path = Path::new(data_root_location.path.as_str()).join(REL_MEDIA_LOCATION);
    let mut result = vec![];
    for file in media_path.read_dir().unwrap() {
        let location = file.unwrap().path();
        result.push(MediaOnDisk {
            location
        });
    }
    return result;
}


pub struct MediaOnDisk {
    pub location: PathBuf,
}


pub struct LoadedMedia {
    pub content: Vec<u8>,
}
