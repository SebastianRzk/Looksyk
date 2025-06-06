use crate::looksyk::datatypes::AssetDescriptor;
use crate::looksyk::media::media_type::{get_media_type_from_extension, MediaType};
use crate::looksyk::queries::insert_file_content::{
    query_insert_file_content_as_audio, query_insert_file_content_as_code,
    query_insert_file_content_as_text, query_insert_file_content_as_video,
};
use crate::looksyk::syntax::markdown::{
    render_as_audio, render_as_image, render_as_video, render_asset_as_link,
};

pub struct Suggestions {
    pub suggestions: Vec<Suggestion>,
}

pub struct Suggestion {
    pub explanation: String,
    pub inplace_markdown: String,
}

pub fn get_suggestion_for_file(asset_descriptor: &AssetDescriptor) -> Suggestions {
    let file_type = get_media_type_from_extension(asset_descriptor);

    match file_type {
        MediaType::Image => Suggestions {
            suggestions: vec![
                suggest_as_markdown_preview_supported(asset_descriptor),
                suggest_as_link(asset_descriptor),
            ],
        },
        MediaType::Video => Suggestions {
            suggestions: vec![
                suggest_as_video_query(asset_descriptor),
                suggest_as_video_html(asset_descriptor),
                suggest_as_link(asset_descriptor),
                suggest_as_markdown_preview_not_supported(asset_descriptor),
            ],
        },
        MediaType::Audio => Suggestions {
            suggestions: vec![
                suggest_as_audio_query(asset_descriptor),
                suggest_as_audio_html(asset_descriptor),
                suggest_as_link(asset_descriptor),
                suggest_as_markdown_preview_not_supported(asset_descriptor),
            ],
        },
        MediaType::Other | MediaType::Html | MediaType::Pdf => Suggestions {
            suggestions: vec![suggest_as_link(asset_descriptor)],
        },
        MediaType::Code => Suggestions {
            suggestions: vec![
                suggest_as_code_query(asset_descriptor),
                suggest_as_text_query(asset_descriptor),
                suggest_as_link(asset_descriptor),
            ],
        },
        MediaType::Text => Suggestions {
            suggestions: vec![
                suggest_as_text_query(asset_descriptor),
                suggest_as_link(asset_descriptor),
            ],
        },
    }
}

const INSERT_LINK_TEXT: &str = "Link: Insert the file as a markdown link (no preview).";
const INSERT_MARKDOWN_PREVIEW: &str =
    "Markdown Preview: Insert the file into the markdown like an image (if filetype is supported).";
const INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED: &str =
    "Markdown Preview (currently not supported by markdown.js)";
const INSERT_QUERY_VIDEO: &str = "Query: Show video (renders a video player)";
const INSERT_HTML_VIDEO: &str = "Html-Code video player";
const INSERT_QUERY_AUDIO: &str = "Query: Show audio (renders a audio player)";
const INSERT_HTML_AUDIO: &str = "Html-Code audio player";
const INSERT_QUERY_TEXT: &str = "Query: Insert text as text block";
const INSERT_QUERY_CODE: &str = "Query: Insert text as code block with code highlighting";

fn suggest_as_link(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_LINK_TEXT.to_string(),
        inplace_markdown: render_asset_as_link(asset_descriptor),
    }
}

fn suggest_as_markdown_preview_supported(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_MARKDOWN_PREVIEW.to_string(),
        inplace_markdown: render_as_image(asset_descriptor),
    }
}

fn suggest_as_markdown_preview_not_supported(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED.to_string(),
        inplace_markdown: render_as_image(asset_descriptor),
    }
}

fn suggest_as_video_query(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_QUERY_VIDEO.to_string(),
        inplace_markdown: query_insert_file_content_as_video(asset_descriptor),
    }
}

fn suggest_as_video_html(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_HTML_VIDEO.to_string(),
        inplace_markdown: render_as_video(asset_descriptor),
    }
}

fn suggest_as_audio_query(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_QUERY_AUDIO.to_string(),
        inplace_markdown: query_insert_file_content_as_audio(asset_descriptor),
    }
}

fn suggest_as_audio_html(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_HTML_AUDIO.to_string(),
        inplace_markdown: render_as_audio(asset_descriptor),
    }
}

fn suggest_as_text_query(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_QUERY_TEXT.to_string(),
        inplace_markdown: query_insert_file_content_as_text(asset_descriptor),
    }
}

fn suggest_as_code_query(asset_descriptor: &AssetDescriptor) -> Suggestion {
    Suggestion {
        explanation: INSERT_QUERY_CODE.to_string(),
        inplace_markdown: query_insert_file_content_as_code(asset_descriptor),
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::asset_descriptor;
    use crate::looksyk::media::suggestion::{
        get_suggestion_for_file, INSERT_HTML_AUDIO, INSERT_HTML_VIDEO, INSERT_LINK_TEXT,
        INSERT_MARKDOWN_PREVIEW, INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED, INSERT_QUERY_AUDIO,
        INSERT_QUERY_CODE, INSERT_QUERY_TEXT, INSERT_QUERY_VIDEO,
    };

    #[test]
    fn test_get_suggestion_for_image() {
        let suggestions = get_suggestion_for_file(&asset_descriptor("test.jpg"));

        assert_eq!(suggestions.suggestions.len(), 2);
        assert_eq!(
            suggestions.suggestions[0].explanation,
            INSERT_MARKDOWN_PREVIEW
        );
        assert_eq!(
            suggestions.suggestions[0].inplace_markdown,
            "![test.jpg](/assets/test.jpg)"
        );
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_LINK_TEXT);
        assert_eq!(
            suggestions.suggestions[1].inplace_markdown,
            "[test.jpg](/assets/test.jpg)"
        );
    }

    #[test]
    fn test_get_suggestion_for_video() {
        let suggestions = get_suggestion_for_file(&asset_descriptor("test.mp4"));

        assert_eq!(suggestions.suggestions.len(), 4);

        assert_eq!(suggestions.suggestions[0].explanation, INSERT_QUERY_VIDEO);
        assert_eq!(
            suggestions.suggestions[0].inplace_markdown,
            "{query: insert-file-content target-file:\"test.mp4\" display:\"video\" }"
        );
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_HTML_VIDEO);
        assert_eq!(suggestions.suggestions[1].inplace_markdown, "<video width=\"720\" controls>\n<source src=\"/assets/test.mp4\" type=\"video/mp4\">\n</video>");
        assert_eq!(suggestions.suggestions[2].explanation, INSERT_LINK_TEXT);
        assert_eq!(
            suggestions.suggestions[2].inplace_markdown,
            "[test.mp4](/assets/test.mp4)"
        );
        assert_eq!(
            suggestions.suggestions[3].explanation,
            INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED
        );
        assert_eq!(
            suggestions.suggestions[3].inplace_markdown,
            "![test.mp4](/assets/test.mp4)"
        );
    }

    #[test]
    fn test_get_suggestion_for_audio() {
        let suggestions = get_suggestion_for_file(&asset_descriptor("test.mp3"));

        assert_eq!(suggestions.suggestions.len(), 4);
        assert_eq!(suggestions.suggestions[0].explanation, INSERT_QUERY_AUDIO);
        assert_eq!(
            suggestions.suggestions[0].inplace_markdown,
            "{query: insert-file-content target-file:\"test.mp3\" display:\"audio\" }"
        );
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_HTML_AUDIO);
        assert_eq!(
            suggestions.suggestions[1].inplace_markdown,
            "<audio controls>\n<source src=\"/assets/test.mp3\" type=\"audio/mp3\">\n</audio>"
        );
        assert_eq!(suggestions.suggestions[2].explanation, INSERT_LINK_TEXT);
        assert_eq!(
            suggestions.suggestions[2].inplace_markdown,
            "[test.mp3](/assets/test.mp3)"
        );
        assert_eq!(
            suggestions.suggestions[3].explanation,
            INSERT_MARKDOWN_PREVIEW_NOT_SUPPORTED
        );
        assert_eq!(
            suggestions.suggestions[3].inplace_markdown,
            "![test.mp3](/assets/test.mp3)"
        );
    }

    #[test]
    fn test_get_suggestion_for_text() {
        let suggestions = get_suggestion_for_file(&asset_descriptor("test.txt"));

        assert_eq!(suggestions.suggestions.len(), 2);
        assert_eq!(suggestions.suggestions[0].explanation, INSERT_QUERY_TEXT);
        assert_eq!(
            suggestions.suggestions[0].inplace_markdown,
            "{query: insert-file-content target-file:\"test.txt\" display:\"inline-text\" }"
        );
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_LINK_TEXT);
        assert_eq!(
            suggestions.suggestions[1].inplace_markdown,
            "[test.txt](/assets/test.txt)"
        );
    }

    #[test]
    fn test_get_suggestion_for_file_code() {
        let suggestions = get_suggestion_for_file(&asset_descriptor("test.rs"));

        assert_eq!(suggestions.suggestions.len(), 3);
        assert_eq!(suggestions.suggestions[0].explanation, INSERT_QUERY_CODE);
        assert_eq!(
            suggestions.suggestions[0].inplace_markdown,
            "{query: insert-file-content target-file:\"test.rs\" display:\"code-block\" }"
        );
        assert_eq!(suggestions.suggestions[1].explanation, INSERT_QUERY_TEXT);
        assert_eq!(
            suggestions.suggestions[1].inplace_markdown,
            "{query: insert-file-content target-file:\"test.rs\" display:\"inline-text\" }"
        );
        assert_eq!(suggestions.suggestions[2].explanation, INSERT_LINK_TEXT);
        assert_eq!(
            suggestions.suggestions[2].inplace_markdown,
            "[test.rs](/assets/test.rs)"
        );
    }
}
