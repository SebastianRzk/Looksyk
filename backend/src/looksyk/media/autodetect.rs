use std::path::PathBuf;

use crate::looksyk::datatypes::AssetDescriptor;
use crate::looksyk::markdown::{render_as_image, render_asset_as_link};
use crate::looksyk::media::media_type::{get_media_type_from_extension, MediaType};
use crate::looksyk::queries::insert_file_content::{query_insert_file_content_as_audio, query_insert_file_content_as_code, query_insert_file_content_as_text, query_insert_file_content_as_video};

pub fn inver_markdown_media_link(index_filename: &String) -> String {
    let filename = PathBuf::from(index_filename).file_name().unwrap().to_str().unwrap().to_string();
    let asset_descriptor = AssetDescriptor::new(filename.to_string());
    let media_type = get_media_type_from_extension(&asset_descriptor);

    return match media_type {
        MediaType::Other => {
            render_asset_as_link(&asset_descriptor)
        }
        MediaType::Video => {
            query_insert_file_content_as_video(&asset_descriptor)
        }
        MediaType::Audio => {
            query_insert_file_content_as_audio(&asset_descriptor)
        }
        MediaType::Code => {
            query_insert_file_content_as_code(&asset_descriptor)
        }
        MediaType::Text => {
            query_insert_file_content_as_text(&asset_descriptor)
        }
        MediaType::Image => {
            render_as_image(&asset_descriptor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inver_markdown_media_link_with_no_extension_should_render_link() {
        let filename = String::from("filename");

        let result = inver_markdown_media_link(&filename);

        assert_eq!(result, "[filename](/assets/filename)");
    }

    #[test]
    fn test_inver_markdown_media_link_with_video_extension_should_render_video_query() {
        let filename = String::from("filename.mp4");

        let result = inver_markdown_media_link(&filename);

        assert_eq!(result, "{query: insert-file-content target-file:\"filename.mp4\" display:\"video\" }");
    }

    #[test]
    fn test_inver_markdown_media_link_with_audio_extension_should_render_audio_query() {
        let filename = String::from("filename.mp3");

        let result = inver_markdown_media_link(&filename);

        assert_eq!(result, "{query: insert-file-content target-file:\"filename.mp3\" display:\"audio\" }");
    }

    #[test]
    fn test_inver_markdown_media_link_with_text_extension_should_render_text_query() {
        let filename = String::from("filename.txt");

        let result = inver_markdown_media_link(&filename);

        assert_eq!(result, "{query: insert-file-content target-file:\"filename.txt\" display:\"inline-text\" }");
    }

    #[test]
    fn test_inver_markdown_media_link_with_code_extension_should_render_code_query() {
        let filename = String::from("filename.rs");

        let result = inver_markdown_media_link(&filename);

        assert_eq!(result, "{query: insert-file-content target-file:\"filename.rs\" display:\"code-block\" }");
    }
}