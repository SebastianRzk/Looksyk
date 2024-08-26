#[derive(Debug, PartialEq)]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Other,
    Code,
    Text
}

pub fn get_media_type_from_extension(file_name: &String) -> MediaType {
    let extension = file_name.split('.').last();
    if extension.is_none() {
        return MediaType::Other;
    }

    match extension.clone().unwrap().to_lowercase().as_str() {
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
    use super::*;

    #[test]
    fn test_get_media_type_from_extension_with_video_extension_should_return_video() {
        let file_name = String::from("file.mp4");

        let result = get_media_type_from_extension(&file_name);

        assert_eq!(result, MediaType::Video);
    }

    #[test]
    fn test_get_media_type_from_extension_with_audio_extension_should_return_audio() {
        let file_name = String::from("file.mp3");

        let result = get_media_type_from_extension(&file_name);

        assert_eq!(result, MediaType::Audio);
    }

    #[test]
    fn test_get_media_type_from_extension_with_text_extension_should_return_text() {
        let file_name = String::from("file.txt");

        let result = get_media_type_from_extension(&file_name);

        assert_eq!(result, MediaType::Text);
    }

    #[test]
    fn test_get_media_type_from_extension_with_code_extension_should_return_code() {
        let file_name = String::from("file.rs");

        let result = get_media_type_from_extension(&file_name);

        assert_eq!(result, MediaType::Code);
    }

    #[test]
    fn test_get_media_type_from_extension_with_unknown_extension_should_return_other() {
        let file_name = String::from("file.unknown");

        let result = get_media_type_from_extension(&file_name);

        assert_eq!(result, MediaType::Other);
    }

    #[test]
    fn test_get_media_type_from_extension_with_no_extension_should_return_other() {
        let file_name = String::from("file");

        let result = get_media_type_from_extension(&file_name);

        assert_eq!(result, MediaType::Other);
    }
}