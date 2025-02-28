use crate::looksyk::datatypes::AssetDescriptor;

pub fn render_as_link(filename: &String, path: &str) -> String {
    format!("[{}]({})", filename, path)
}

pub fn render_asset_as_link(asset_descriptor: &AssetDescriptor) -> String {
    render_as_link(
        &asset_descriptor.get_display_name(),
        &asset_descriptor.get_qualified_path(),
    )
}

pub fn render_as_image(asset_descriptor: &AssetDescriptor) -> String {
    format!(
        "![{}]({})",
        asset_descriptor.get_display_name(),
        asset_descriptor.get_qualified_path()
    )
}

pub fn render_as_video(asset_descriptor: &AssetDescriptor) -> String {
    format!(
        "<video width=\"720\" controls>
<source src=\"{}\" type=\"video/{}\">
</video>",
        asset_descriptor.get_qualified_path(),
        asset_descriptor.get_extension("mp4")
    )
}

pub fn render_as_audio(asset_descriptor: &AssetDescriptor) -> String {
    format!(
        "<audio controls>
<source src=\"{}\" type=\"audio/{}\">
</audio>",
        asset_descriptor.get_qualified_path(),
        asset_descriptor.get_extension("mp3")
    )
}

pub fn render_as_code_block(language: String, content: &String) -> String {
    format!(
        "\
```{}
{}
```",
        language, content
    )
}

pub fn encode_uri_component(file_name: &str) -> String {
    file_name.replace(" ", "%20").replace("#", "%23")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::builder::test_builder::asset_descriptor;

    #[test]
    fn test_render_as_link_with_filename_and_path_should_render_link() {
        let filename = String::from("filename");
        let path = String::from("path");

        let result = render_as_link(&filename, &path);

        assert_eq!(result, "[filename](path)");
    }

    #[test]
    fn test_render_as_video_with_filename_and_path_should_render_video() {
        let result = render_as_video(&asset_descriptor("filename.mp4"));

        assert_eq!(result, "<video width=\"720\" controls>\n<source src=\"/assets/filename.mp4\" type=\"video/mp4\">\n</video>");
    }

    #[test]
    fn test_render_as_code_block_with_language_and_content_should_render_code_block() {
        let language = String::from("rust");
        let content = String::from("content");

        let result = render_as_code_block(language, &content);

        assert_eq!(result, "```rust\ncontent\n```");
    }

    #[test]
    fn test_render_as_audio_with_filename_and_path_should_render_audio() {
        let result = render_as_audio(&asset_descriptor("filename.ogg"));

        assert_eq!(
            result,
            "<audio controls>\n<source src=\"/assets/filename.ogg\" type=\"audio/ogg\">\n</audio>"
        );
    }
}
