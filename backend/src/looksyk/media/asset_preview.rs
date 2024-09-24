use crate::io::fs::media::MediaOnDisk;
use crate::looksyk::datatypes::AssetDescriptor;
use crate::looksyk::syntax::markdown::{render_as_audio, render_as_image, render_as_video};
use crate::looksyk::media::media_type::{get_media_type_from_extension, MediaType};
use crate::looksyk::queries::insert_file_content::render_code_block;
use crate::state::asset_cache::AssetCache;
use crate::state::state::DataRootLocation;
use humansize::{format_size, DECIMAL};

pub struct AssetPreview {
    pub markdown_preview: Option<String>,
    pub html_preview_link: Option<String>,
    pub properties: AssetProperties,
}

pub struct AssetProperties {
    pub size: String,
    pub full_qualified_path: String,
}



pub fn generate_asset_preview(asset_descriptor: AssetDescriptor, file_size: u64, cache: &mut AssetCache, data_root_location: &DataRootLocation) -> AssetPreview {
    let media_type = get_media_type_from_extension(&asset_descriptor);
    match media_type {
        MediaType::Image => {
            AssetPreview {
                markdown_preview: Some(render_as_image(&asset_descriptor)),
                html_preview_link: None,
                properties: AssetProperties {
                    size: format_size(file_size, DECIMAL),
                    full_qualified_path: asset_descriptor.get_qualified_path(),
                }
            }
        }
        MediaType::Video => {
            AssetPreview {
                markdown_preview: Some(render_as_video(&asset_descriptor)),
                html_preview_link: None,
                properties: AssetProperties {
                    size: format_size(file_size, DECIMAL),
                    full_qualified_path: asset_descriptor.get_qualified_path(),
                }
            }
        }
        MediaType::Audio => {
            AssetPreview {
                markdown_preview: Some(render_as_audio(&asset_descriptor)),
                html_preview_link: None,
                properties: AssetProperties {
                    size: format_size(file_size, DECIMAL),
                    full_qualified_path: asset_descriptor.get_qualified_path(),
                }
            }

        }
        MediaType::Other => {
            AssetPreview {
                markdown_preview: None,
                html_preview_link: None,
                properties: AssetProperties {
                    size: format_size(file_size, DECIMAL),
                    full_qualified_path: asset_descriptor.get_qualified_path(),
                }
            }
        }
        MediaType::Code => {
            AssetPreview {
                markdown_preview: Some(render_code_block(&MediaOnDisk::new(&asset_descriptor), cache, data_root_location).inplace_markdown),
                html_preview_link: None,
                properties: AssetProperties {
                    size: format_size(file_size, DECIMAL),
                    full_qualified_path: asset_descriptor.get_qualified_path(),
                }
            }

        }
        MediaType::Text => {
            AssetPreview {
                markdown_preview: Some(render_code_block(&MediaOnDisk::new(&asset_descriptor), cache, data_root_location).inplace_markdown),
                html_preview_link: None,
                properties: AssetProperties {
                    size: format_size(file_size, DECIMAL),
                    full_qualified_path: asset_descriptor.get_qualified_path(),
                }
            }
        }
        MediaType::Html => {
            AssetPreview {
                markdown_preview: None,
                html_preview_link: Some(asset_descriptor.get_qualified_path()),
                properties: AssetProperties {
                    size: format_size(file_size, DECIMAL),
                    full_qualified_path: asset_descriptor.get_qualified_path(),
                }
            }

        }
        MediaType::Pdf => {
            AssetPreview {
                markdown_preview: None,
                html_preview_link: Some(asset_descriptor.get_qualified_path()),
                properties: AssetProperties {
                    size: format_size(file_size, DECIMAL),
                    full_qualified_path: asset_descriptor.get_qualified_path(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::data_root_location;
    use crate::looksyk::datatypes::AssetDescriptor;
    use crate::looksyk::media::asset_preview::{generate_asset_preview};
    use crate::state::asset_cache::AssetCache;

    #[test]
    fn test_generate_asset_preview_should_return_image_preview() {
        let asset_descriptor = AssetDescriptor::new("filename.png".to_string());
        let file_size = 100;
        let mut cache = AssetCache::new();
        let data_root_location = data_root_location("data");
        let result = generate_asset_preview(asset_descriptor, file_size, &mut cache, &data_root_location);

        assert_eq!(result.markdown_preview.unwrap(), "![filename.png](/assets/filename.png)");
        assert_eq!(result.html_preview_link, None);
        assert_eq!(result.properties.size, "100 B");
        assert_eq!(result.properties.full_qualified_path, "/assets/filename.png");
    }

    #[test]
    fn test_generate_asset_preview_should_return_video_preview() {
        let asset_descriptor = AssetDescriptor::new("filename.mp4".to_string());
        let file_size = 100;
        let mut cache = AssetCache::new();
        let data_root_location = data_root_location("data");
        let result = generate_asset_preview(asset_descriptor, file_size, &mut cache, &data_root_location);

        assert_eq!(result.markdown_preview.unwrap(), "<video width=\"720\" controls>\n<source src=\"/assets/filename.mp4\" type=\"video/mp4\">\n</video>");
        assert_eq!(result.html_preview_link, None);
        assert_eq!(result.properties.size, "100 B");
        assert_eq!(result.properties.full_qualified_path, "/assets/filename.mp4");
    }

}