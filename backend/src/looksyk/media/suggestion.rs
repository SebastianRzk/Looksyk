use crate::io::http::media::config::pad_url_media_location;
use crate::looksyk::markdown::{render_as_audio, render_as_image, render_as_link, render_as_video};
use crate::looksyk::media::media_type::{get_media_type_from_extension, MediaType};
use crate::looksyk::queries::insert_file_content::{query_insert_file_content_as_audio, query_insert_file_content_as_code, query_insert_file_content_as_text, query_insert_file_content_as_video};

pub struct Suggestions {
    pub suggestions: Vec<Suggestion>,
}

pub struct Suggestion {
    pub explanation: String,
    pub inplace_markdown: String,
}


pub fn get_suggestion_for_file(file_name: &String) -> Suggestions {
    let file_type = get_media_type_from_extension(file_name);

    match file_type {
        MediaType::Image => {
            Suggestions {
                suggestions: vec![
                    suggest_as_markdown_preview_supported(file_name),
                    suggest_as_link(file_name),
                ],
            }
        }
        MediaType::Video => {
            Suggestions {
                suggestions: vec![
                    suggest_as_video_query(file_name),
                    suggest_as_video_html(file_name),
                    suggest_as_link(file_name),
                    suggest_as_markdown_preview_not_supported(file_name),
                ]
            }
        }
        MediaType::Audio => {
            Suggestions {
                suggestions: vec![
                    suggest_as_audio_query(file_name),
                    suggest_as_audio_html(file_name),
                    suggest_as_link(file_name),
                    suggest_as_markdown_preview_not_supported(file_name)
                ]
            }
        }
        MediaType::Other => {
            Suggestions {
                suggestions: vec![
                    suggest_as_link(file_name),
                ]
            }
        }
        MediaType::Code => {
            Suggestions {
                suggestions: vec![
                    suggest_as_code_query(file_name),
                    suggest_as_text_query(file_name),
                    suggest_as_link(file_name),
                ]
            }
        }
        MediaType::Text => {
            Suggestions {
                suggestions: vec![
                    suggest_as_text_query(file_name),
                    suggest_as_link(file_name),
                ]
            }
        }
    }
}

const INSERT_LINK_TEXT: &str = "Link: Insert the file as a markdown link (no preview).";
const INSERT_MARKDOWN_PREVIEW: &str = "Markdown Preview: Insert the file into the markdown like an image (if filetype is supported).";
const INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED: &str = "Markdown Preview (currently not supported by markdown.js)";
const INSERT_QUERY_VIDEO: &str = "Query: Show video (renders a video player)";
const INSERT_HTML_VIDEO: &str = "Html-Code video player";
const INSERT_QUERY_AUDIO: &str = "Query: Show audio (renders a audio player)";
const INSERT_HTML_AUDIO: &str = "Html-Code audio player";
const INSERT_QUERY_TEXT: &str = "Query: Insert text as text block";
const INSERT_QUERY_CODE: &str = "Query: Insert text as code block with code highlighting";

fn suggest_as_link(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_LINK_TEXT.to_string(),
        inplace_markdown: render_as_link(file_name, &pad_url_media_location(file_name)),
    }
}

fn suggest_as_markdown_preview_supported(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_MARKDOWN_PREVIEW.to_string(),
        inplace_markdown: render_as_image(file_name, &pad_url_media_location(file_name)),
    }
}

fn suggest_as_markdown_preview_not_supported(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED.to_string(),
        inplace_markdown: render_as_image(file_name, &pad_url_media_location(file_name)),
    }
}

fn suggest_as_video_query(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_QUERY_VIDEO.to_string(),
        inplace_markdown: query_insert_file_content_as_video(file_name),
    }
}

fn suggest_as_video_html(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_HTML_VIDEO.to_string(),
        inplace_markdown: render_as_video(file_name),
    }
}


fn suggest_as_audio_query(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_QUERY_AUDIO.to_string(),
        inplace_markdown: query_insert_file_content_as_audio(file_name),
    }
}

fn suggest_as_audio_html(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_HTML_AUDIO.to_string(),
        inplace_markdown: render_as_audio(file_name),
    }
}

fn suggest_as_text_query(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_QUERY_TEXT.to_string(),
        inplace_markdown: query_insert_file_content_as_text(file_name),
    }
}

fn suggest_as_code_query(file_name: &String) -> Suggestion {
    Suggestion {
        explanation: INSERT_QUERY_CODE.to_string(),
        inplace_markdown: query_insert_file_content_as_code(file_name),
    }
}


#[cfg(test)]
mod tests {
    use crate::looksyk::media::suggestion::{get_suggestion_for_file, INSERT_HTML_AUDIO, INSERT_HTML_VIDEO, INSERT_LINK_TEXT, INSERT_MARKDOWN_PREVIEW, INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED, INSERT_QUERY_AUDIO, INSERT_QUERY_CODE, INSERT_QUERY_TEXT, INSERT_QUERY_VIDEO};

    #[test]
    fn test_get_suggestion_for_image() {
        let suggestions = get_suggestion_for_file(&"test.jpg".to_string());

        assert_eq!(suggestions.suggestions.len(), 2);
        assert_eq!(suggestions.suggestions[0].explanation, INSERT_MARKDOWN_PREVIEW);
        assert_eq!(suggestions.suggestions[0].inplace_markdown, "![test.jpg](/assets/test.jpg)");
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_LINK_TEXT);
        assert_eq!(suggestions.suggestions[1].inplace_markdown, "[test.jpg](/assets/test.jpg)");
    }

    #[test]
    fn test_get_suggestion_for_video() {
        let suggestions = get_suggestion_for_file(&"test.mp4".to_string());

        assert_eq!(suggestions.suggestions.len(), 4);

        assert_eq!(suggestions.suggestions[0].explanation, INSERT_QUERY_VIDEO);
        assert_eq!(suggestions.suggestions[0].inplace_markdown, "{query: insert-file-content target-file:\"test.mp4\" display:\"video\" }");
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_HTML_VIDEO);
        assert_eq!(suggestions.suggestions[1].inplace_markdown, "<video width=\"720\" controls>\n<source src=\"/assets/test.mp4\" type=\"video/mp4\">\n</video>");
        assert_eq!(suggestions.suggestions[2].explanation, INSERT_LINK_TEXT);
        assert_eq!(suggestions.suggestions[2].inplace_markdown, "[test.mp4](/assets/test.mp4)");
        assert_eq!(suggestions.suggestions[3].explanation, INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED);
        assert_eq!(suggestions.suggestions[3].inplace_markdown, "![test.mp4](/assets/test.mp4)");
    }

    #[test]
    fn test_get_suggestion_for_audio() {
        let suggestions = get_suggestion_for_file(&"test.mp3".to_string());

        assert_eq!(suggestions.suggestions.len(), 4);
        assert_eq!(suggestions.suggestions[0].explanation, INSERT_QUERY_AUDIO);
        assert_eq!(suggestions.suggestions[0].inplace_markdown, "{query: insert-file-content target-file:\"test.mp3\" display:\"audio\" }");
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_HTML_AUDIO);
        assert_eq!(suggestions.suggestions[1].inplace_markdown, "<audio controls>\n<source src=\"/assets/test.mp3\" type=\"audio/mp3\">\n</audio>");
        assert_eq!(suggestions.suggestions[2].explanation, INSERT_LINK_TEXT);
        assert_eq!(suggestions.suggestions[2].inplace_markdown, "[test.mp3](/assets/test.mp3)");
        assert_eq!(suggestions.suggestions[3].explanation, INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED);
        assert_eq!(suggestions.suggestions[3].inplace_markdown, "![test.mp3](/assets/test.mp3)");
    }

    #[test]
    fn test_get_suggestion_for_text() {
        let suggestions = get_suggestion_for_file(&"test.txt".to_string());

        assert_eq!(suggestions.suggestions.len(), 2);
        assert_eq!(suggestions.suggestions[0].explanation, INSERT_QUERY_TEXT);
        assert_eq!(suggestions.suggestions[0].inplace_markdown, "{query: insert-file-content target-file:\"test.txt\" display:\"inline-text\" }");
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_LINK_TEXT);
        assert_eq!(suggestions.suggestions[1].inplace_markdown, "[test.txt](/assets/test.txt)");
    }

    #[test]
    fn test_get_suggestion_for_file_code() {
        let suggestions = get_suggestion_for_file(&"test.rs".to_string());

        assert_eq!(suggestions.suggestions.len(), 3);
        assert_eq!(suggestions.suggestions[0].explanation, INSERT_QUERY_CODE);
        assert_eq!(suggestions.suggestions[0].inplace_markdown, "{query: insert-file-content target-file:\"test.rs\" display:\"code-block\" }");
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_QUERY_TEXT);
        assert_eq!(suggestions.suggestions[1].inplace_markdown, "{query: insert-file-content target-file:\"test.rs\" display:\"inline-text\" }");
        assert_eq!(suggestions.suggestions[2].explanation, INSERT_LINK_TEXT);
        assert_eq!(suggestions.suggestions[2].inplace_markdown, "[test.rs](/assets/test.rs)");
    }
}