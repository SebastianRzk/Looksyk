use std::path::PathBuf;
use crate::looksyk::markdown::{render_as_image, render_as_link};
use crate::looksyk::media::media_type::{get_media_type_from_extension, MediaType};
use crate::looksyk::queries::insert_file_content::{query_insert_file_content_as_audio, query_insert_file_content_as_code, query_insert_file_content_as_text, query_insert_file_content_as_video};

pub fn inver_markdown_media_link(index_filename: &String, path: &String) -> String {
    let media_type = get_media_type_from_extension(index_filename);
    let filename = PathBuf::from(index_filename).file_name().unwrap().to_str().unwrap().to_string();
    match media_type {
        MediaType::Other => {
            return render_as_link(index_filename, path);
        }
        MediaType::Video => {
            return query_insert_file_content_as_video(&filename);
        }
        MediaType::Audio => {
            return query_insert_file_content_as_audio(&filename);
        }
        MediaType::Code => {
            return query_insert_file_content_as_code(&filename);
        }
        MediaType::Text => {
            return query_insert_file_content_as_text(&filename);
        }
        MediaType::Image => {
            return render_as_image(index_filename, path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inver_markdown_media_link_with_no_extension_should_render_link() {
        let filename = String::from("filename");
        let path = String::from("path");

        let result = inver_markdown_media_link(&filename, &path);

        assert_eq!(result, "[filename](path)");
    }

    #[test]
    fn test_inver_markdown_media_link_with_video_extension_should_render_video_query() {
        let filename = String::from("filename.mp4");
        let path = String::from("path");

        let result = inver_markdown_media_link(&filename, &path);

        assert_eq!(result, "{query: insert-file-content target-file:\"filename.mp4\" display:\"video\" }");
    }

    #[test]
    fn test_inver_markdown_media_link_with_audio_extension_should_render_audio_query() {
        let filename = String::from("filename.mp3");
        let path = String::from("path");

        let result = inver_markdown_media_link(&filename, &path);

        assert_eq!(result, "{query: insert-file-content target-file:\"filename.mp3\" display:\"audio\" }");
    }

    #[test]
    fn test_inver_markdown_media_link_with_text_extension_should_render_text_query() {
        let filename = String::from("filename.txt");
        let path = String::from("path");

        let result = inver_markdown_media_link(&filename, &path);

        assert_eq!(result, "{query: insert-file-content target-file:\"filename.txt\" display:\"inline-text\" }");
    }

    #[test]
    fn test_inver_markdown_media_link_with_code_extension_should_render_code_query() {
        let filename = String::from("filename.rs");
        let path = String::from("path");

        let result = inver_markdown_media_link(&filename, &path);

        assert_eq!(result, "{query: insert-file-content target-file:\"filename.rs\" display:\"code-block\" }");
    }
}