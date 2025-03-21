use crate::io::fs::basic_file::{is_text_file, read_file};
use crate::io::fs::media::MediaState::Found;
use crate::io::fs::media::{create_absolute_media_path, read_media_state, MediaOnDisk};
use crate::looksyk::data::config::startup_configuration::MAX_INLINE_FILESIZE;
use crate::state::application_state::GraphRootLocation;
use crate::state::asset_cache::{AssetFileContent, AssetState, FileSizeViolation};

pub fn load_cachable_asset(
    media_on_disk: &MediaOnDisk,
    data_root_location: &GraphRootLocation,
) -> AssetState {
    if let Found(file_info) = read_media_state(media_on_disk, data_root_location) {
        let abs_path = create_absolute_media_path(media_on_disk, data_root_location);
        if file_info.size > MAX_INLINE_FILESIZE {
            return AssetState::TooLarge(FileSizeViolation {
                max_size: MAX_INLINE_FILESIZE,
                file_size: file_info.size,
            });
        }
        if !is_text_file(abs_path.clone()) {
            return AssetState::NotText;
        }

        return AssetState::Found(AssetFileContent {
            content: read_file(abs_path),
        });
    }
    AssetState::NotFound
}
