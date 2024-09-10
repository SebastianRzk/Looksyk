use crate::looksyk::datatypes::AssetDescriptor;

#[derive(Debug, PartialEq)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Other,
    Code,
    Text
}

pub fn get_media_type_from_extension(asset_descriptor: &AssetDescriptor) -> MediaType {
    let extension = asset_descriptor.find_extension();
    if extension.is_none() {
        return MediaType::Other;
    }

    match extension.unwrap().to_lowercase().as_str() {
        "mp4" | "webm" | "ogm" => MediaType::Video,
        "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" => MediaType::Image,
        "mp3" | "wav" | "flac" | "ogg" => MediaType::Audio,
        "txt"  => MediaType::Text,
        "rs" | "py" | "js" | "css" | "java" | "kt" | "c" | "sql" | "go" | "php"=> MediaType::Code,
        _ => MediaType::Other
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::asset_descriptor;
    use super::*;

    #[test]
    fn test_get_media_type_from_extension_with_video_extension_should_return_video() {
        let result = get_media_type_from_extension(&asset_descriptor("file.mp4"));

        assert_eq!(result, MediaType::Video);
    }

    #[test]
    fn test_get_media_type_from_extension_with_audio_extension_should_return_audio() {
        let result = get_media_type_from_extension(&asset_descriptor("file.mp3"));

        assert_eq!(result, MediaType::Audio);
    }

    #[test]
    fn test_get_media_type_from_extension_with_text_extension_should_return_text() {
        let result = get_media_type_from_extension(&asset_descriptor("file.txt"));

        assert_eq!(result, MediaType::Text);
    }

    #[test]
    fn test_get_media_type_from_extension_with_code_extension_should_return_code() {
        let result = get_media_type_from_extension(&asset_descriptor("file.rs"));

        assert_eq!(result, MediaType::Code);
    }

    #[test]
    fn test_get_media_type_from_extension_with_unknown_extension_should_return_other() {
        let result = get_media_type_from_extension(&asset_descriptor("file.unknown"));

        assert_eq!(result, MediaType::Other);
    }

    #[test]
    fn test_get_media_type_from_extension_with_no_extension_should_return_other() {
        let result = get_media_type_from_extension(&asset_descriptor("file"));

        assert_eq!(result, MediaType::Other);
    }
}