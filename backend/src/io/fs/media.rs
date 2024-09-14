use std::collections::HashMap;
use std::path::PathBuf;

use actix_files::NamedFile;
use chrono::Utc;

use crate::io::fs::basic_file::{exists_file, get_file_size, read_binary_file, read_file};
use crate::io::fs::paths::{REL_MEDIA_CONFIG_PATH, REL_MEDIA_LOCATION};
use crate::io::hash::hash_file_content;
use crate::looksyk::datatypes::AssetDescriptor;
use crate::looksyk::index::media::{find_file, IndexedMedia, MediaIndex};
use crate::state::state::DataRootLocation;

pub fn read_media_config(data_root_location: &DataRootLocation) -> MediaIndex {
    let media_config_path = media_config_path(data_root_location);
    let config_file_content_as_str = read_file(media_config_path);
    let json: Vec<IndexedMedia> = serde_json::from_str(config_file_content_as_str.as_str()).unwrap();
    MediaIndex {
        media: json
    }
}

pub fn write_media_config(data_root_location: &DataRootLocation, media_index: &MediaIndex) {
    let config_file_content_as_str = serde_json::to_string(&media_index.media).unwrap();
    std::fs::write(media_config_path(&data_root_location), config_file_content_as_str).unwrap();
}

fn media_config_path(data_path: &DataRootLocation) -> PathBuf {
    data_path.path.clone().join(REL_MEDIA_CONFIG_PATH)
}

pub fn init_media(data_root_location: &DataRootLocation, current_media_index: &MediaIndex) -> MediaIndex {
    let all_files_in_folder = read_all_media_files(data_root_location);
    let mut result_index: Vec<IndexedMedia> = vec![];

    for file in all_files_in_folder {
        let file_in_index = find_file(&file.name, current_media_index);
        if file_in_index.is_some() {
            result_index.push(file_in_index.clone().unwrap().clone());
        } else {
            println!("Add media object to index: {}", &file.name.as_str());
            result_index.push(IndexedMedia {
                file_name: file.name.clone(),
                sha3: create_hash(file, data_root_location),
            });
        }
    }

    MediaIndex {
        media: result_index,
    }
}

pub fn read_file_sizes(data_root_location: &DataRootLocation) -> HashMap<String, u64> {
    let all_files_in_folder = read_all_media_files(data_root_location);

    let mut result: HashMap<String, u64> = HashMap::new();

    for file in all_files_in_folder {
        let file_path = create_absolute_media_path(&file, data_root_location);
        let size = get_file_size(file_path);
        result.insert(file.name.clone(), size);
    }

    result
}


pub fn destination_path(filename: &str, data_root_location: &DataRootLocation) -> PathBuf {
    let timestamp = Utc::now().format("%Y_%m_%d_%H_%M_%S").to_string();

    let (filestem, file_ending) = parse_name(filename);

    let filename = format!("{}_{}.{}", filestem, timestamp, file_ending);
    create_absolute_media_path(&MediaOnDisk {
        name: filename
    }, data_root_location)
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

pub fn create_absolute_media_path(file: &MediaOnDisk, data_root_location: &DataRootLocation) -> PathBuf {
    data_root_location.path.clone().join(REL_MEDIA_LOCATION).join(file.name.clone())
}

pub fn create_hash(file: MediaOnDisk, data_root_location: &DataRootLocation) -> String {
    let file_conent = read_binary_file(create_absolute_media_path(&file, data_root_location));
    hash_file_content(LoadedMedia {
        content: file_conent,
    })
}

pub fn read_media_file(name: &String, location: &DataRootLocation) -> std::io::Result<NamedFile> {
    NamedFile::open(create_absolute_media_path(&MediaOnDisk {
        name: name.clone()
    }, location))
}

pub fn read_media_state(media_on_disk: &MediaOnDisk, location: &DataRootLocation) -> MediaState {
    let media_path = create_absolute_media_path(media_on_disk, location);
    println!("Checking media path: {}", media_path.to_str().unwrap());
    if ! exists_file(media_path.clone()) {
        return MediaState::NotFound;
    }

    let size = get_file_size(media_path);
    MediaState::Found(MediaSize {
        size
    })
}


pub fn read_all_media_files(data_root_location: &DataRootLocation) -> Vec<MediaOnDisk> {
    let media_path = data_root_location.path.clone().join(REL_MEDIA_LOCATION);
    let mut result = vec![];
    for file in media_path.read_dir().unwrap() {
        let location = file.unwrap().file_name().to_str().unwrap().to_string();
        result.push(MediaOnDisk {
            name: location
        });
    }
    return result;
}


pub struct MediaOnDisk {
    pub name: String,
}

impl MediaOnDisk {
    pub fn as_asset_descriptor(&self) -> AssetDescriptor {
        AssetDescriptor::new(self.name.clone())
    }
}


pub struct LoadedMedia {
    pub content: Vec<u8>,
}


pub enum MediaState {
    Found(MediaSize),
    NotFound
}

pub struct MediaSize{
    pub size: u64
}