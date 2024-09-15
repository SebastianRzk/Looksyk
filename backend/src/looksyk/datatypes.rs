use crate::io::http::media::config::REL_MEDIA_LOCATION;

pub struct AssetDescriptor {
    display_name: String
}


impl AssetDescriptor {
    pub fn new(display_name: String) -> AssetDescriptor {
        AssetDescriptor {
            display_name
        }
    }

    pub fn get_display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn get_qualified_path(&self) -> String {
        format!("/{}/{}", REL_MEDIA_LOCATION, encode_uri_component(&self.display_name))
    }

    pub fn get_extension(&self, default: &str) -> String {
        let result = self.display_name.split('.').last().unwrap();
        if result.eq(&self.display_name) {
            return default.to_string();
        }
        result.to_string()
    }

    pub fn find_extension(&self) -> Option<String> {
        let result = self.display_name.split('.').last().unwrap();
        if result.eq(&self.display_name) {
            return None;
        }
        Some(result.to_string())
    }
}


fn encode_uri_component(file_name: &String) -> String {
    file_name.replace(" ", "%20").replace("#", "%23")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_descriptor_should_return_display_name() {
        let asset_descriptor = AssetDescriptor::new("filename".to_string());

        let result = asset_descriptor.get_display_name();

        assert_eq!(result, "filename");
    }

    #[test]
    fn test_asset_descriptor_should_return_qualified_path() {
        let asset_descriptor = AssetDescriptor::new("filename".to_string());

        let result = asset_descriptor.get_qualified_path();

        assert_eq!(result, "/assets/filename");
    }

    #[test]
    fn test_asset_descriptor_should_return_extension() {
        let asset_descriptor = AssetDescriptor::new("filename.mp4".to_string());

        let result = asset_descriptor.get_extension("mp3");

        assert_eq!(result, "mp4");
    }

    #[test]
    fn test_asset_descriptor_should_return_default_extension() {
        let asset_descriptor = AssetDescriptor::new("filename".to_string());

        let result = asset_descriptor.get_extension("mp3");

        assert_eq!(result, "mp3");
    }

    #[test]
    fn test_asset_descriptor_should_escape_qualified_path() {
        assert_eq!(AssetDescriptor::new("a b".to_string()).get_qualified_path(), "/assets/a%20b");
        assert_eq!(AssetDescriptor::new("a#b".to_string()).get_qualified_path(), "/assets/a%23b");
    }
}