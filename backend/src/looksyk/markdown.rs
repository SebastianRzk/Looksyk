pub fn render_as_link(filename: &String, path: &String) -> String {
    format!("[{}]({})", filename, path)
}

pub fn render_as_image(filename: &String, path: &String) -> String {
    format!("![{}]({})", filename, path)
}

pub fn render_as_video(filename_path: &String) -> String {
    format!("<video width=\"720\" controls>
<source src=\"/assets/{}\" type=\"video/{}\">
</video>", filename_path, get_extension(&filename_path, "mp4"))
}

pub fn render_as_audio(filename_path: &String) -> String {
    format!("<audio controls>
<source src=\"/assets/{}\" type=\"audio/{}\">
</audio>", filename_path, get_extension(&filename_path, "mp3"))
}

fn get_extension(filename: &String, default: &str) -> String {
    let result = filename.split('.').last().unwrap();
    if result.eq(filename) {
        return default.to_string();
    }
    return result.to_string();
}

pub fn render_as_code_block(language: String, content: &String) -> String {
    format!("\
```{}
{}
```", language, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_as_link_with_filename_and_path_should_render_link() {
        let filename = String::from("filename");
        let path = String::from("path");

        let result = render_as_link(&filename, &path);

        assert_eq!(result, "[filename](path)");
    }

    #[test]
    fn test_render_as_video_with_filename_and_path_should_render_video() {
        let filename = String::from("filename.mp4");

        let result = render_as_video(&filename);

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
    fn test_get_extension_with_filename_and_extension_should_return_extension() {
        let filename = String::from("filename.mp4");
        let default = String::from("default");

        let result = get_extension(&filename, &default);

        assert_eq!(result, "mp4");
    }

    #[test]
    fn test_get_extension_with_filename_and_no_extension_should_return_default() {
        let filename = String::from("filename");
        let default = String::from("default");

        let result = get_extension(&filename, &default);

        assert_eq!(result, "default");
    }

    #[test]
    fn test_render_as_audio_with_filename_and_path_should_render_audio() {
        let filename = String::from("filename.ogg");

        let result = render_as_audio(&filename);

        assert_eq!(result, "<audio controls>\n<source src=\"/assets/filename.ogg\" type=\"audio/ogg\">\n</audio>");
    }
}