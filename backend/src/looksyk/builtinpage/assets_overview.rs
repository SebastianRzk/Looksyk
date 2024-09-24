use crate::looksyk::builtinpage::generating_page_util::create_textblock;
use crate::looksyk::datatypes::AssetDescriptor;
use crate::looksyk::index::media::MediaIndex;
use crate::looksyk::syntax::markdown::render_as_link;
use crate::looksyk::media::media_type::{get_media_type_from_extension, MediaType};
use crate::looksyk::model::{ParsedBlock, ParsedMarkdownFile};
use std::cmp::Ordering;
use std::collections::HashMap;
use humansize::{format_size, DECIMAL};

pub fn generate_assets_overview_page(media_index: &MediaIndex, sizes: HashMap<String, u64>) -> ParsedMarkdownFile {
    let sorted_assets = sort_assets(&media_index, sizes);

    let result = vec![
        render_table("Images", sorted_assets.images),
        render_table("PDFs", sorted_assets.pdfs),
        render_table("Htmls", sorted_assets.htmls),
        render_table("Videos", sorted_assets.videos),
        render_table("Audios", sorted_assets.audios),
        render_table("Code", sorted_assets.codes),
        render_table("Texts", sorted_assets.texts),
        render_table("Other", sorted_assets.others),
    ];


    ParsedMarkdownFile {
        blocks: result
    }
}

fn render_table(asset_type_name: &str, assets: Vec<AssetDescription>) -> ParsedBlock {
    let mut result = String::from(format!("### {} \n \n | name | size |\n| :-- | :-- |\n", asset_type_name));
    for asset in assets {
        result.push_str(&format!("| {} | {} |\n",
                                 render_as_link(&asset.indexed_media.get_display_name(),
                                                &asset.indexed_media.get_qualified_path()),
                                 format_size(asset.file_size, DECIMAL)));
    }
    create_textblock(result.as_str(), 0)
}

fn sort_assets(media_index: &&MediaIndex, sizes: HashMap<String, u64>) -> SortedAssets {
    let mut images: Vec<AssetDescription> = vec![];
    let mut videos: Vec<AssetDescription> = vec![];
    let mut audios: Vec<AssetDescription> = vec![];
    let mut others: Vec<AssetDescription> = vec![];
    let mut codes: Vec<AssetDescription> = vec![];
    let mut texts: Vec<AssetDescription> = vec![];
    let mut htmls: Vec<AssetDescription> = vec![];
    let mut pdfs: Vec<AssetDescription> = vec![];


    for media in &media_index.media {
        let indexed_media = AssetDescriptor::new(media.file_name.clone());
        let media_type = get_media_type_from_extension(&indexed_media);
        let file_size = sizes.get(&media.file_name).unwrap_or(&0).clone();
        let asset_description = AssetDescription {
            indexed_media,
            file_size,
        };
        match media_type {
            MediaType::Image => {
                images.push(asset_description);
            }
            MediaType::Video => {
                videos.push(asset_description);
            }
            MediaType::Audio => {
                audios.push(asset_description);
            }
            MediaType::Other => {
                others.push(asset_description);
            }
            MediaType::Code => {
                codes.push(asset_description);
            }
            MediaType::Text => {
                texts.push(asset_description);
            }
            MediaType::Html => {
                htmls.push(asset_description);
            }
            MediaType::Pdf => {
                pdfs.push(asset_description);
            }
        }
    }
    images.sort_by(sort_by_filesize);
    videos.sort_by(sort_by_filesize);
    audios.sort_by(sort_by_filesize);
    others.sort_by(sort_by_filesize);
    codes.sort_by(sort_by_filesize);
    texts.sort_by(sort_by_filesize);
    htmls.sort_by(sort_by_filesize);
    pdfs.sort_by(sort_by_filesize);


    let sorted_assets = SortedAssets {
        images,
        videos,
        audios,
        others,
        codes,
        texts,
        htmls,
        pdfs,
    };
    sorted_assets
}

fn sort_by_filesize(a: &AssetDescription, b: &AssetDescription) -> Ordering {
    b.file_size.cmp(&a.file_size)
}

struct SortedAssets {
    images: Vec<AssetDescription>,
    videos: Vec<AssetDescription>,
    audios: Vec<AssetDescription>,
    others: Vec<AssetDescription>,
    codes: Vec<AssetDescription>,
    texts: Vec<AssetDescription>,
    htmls: Vec<AssetDescription>,
    pdfs: Vec<AssetDescription>,
}

struct AssetDescription {
    indexed_media: AssetDescriptor,
    file_size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::looksyk::index::media::test_builder::indexed_media;

    #[test]
    fn test_sort_assets_should_sort_assets_by_filesize() {
        let media_index = MediaIndex {
            media: vec![
                indexed_media("a.jpg", ""),
                indexed_media("b.jpg", ""),
                indexed_media("c.jpg", ""),
            ]
        };
        let sizes = vec![
            ("a.jpg".to_string(), 1),
            ("b.jpg".to_string(), 3),
            ("c.jpg".to_string(), 2),
        ].into_iter().collect();

        let result = sort_assets(&&media_index, sizes);

        assert_eq!(result.images[0].indexed_media.get_display_name(), "b.jpg");
        assert_eq!(result.images[1].indexed_media.get_display_name(), "c.jpg");
        assert_eq!(result.images[2].indexed_media.get_display_name(), "a.jpg");
    }

    #[test]
    fn test_sort_assets_should_sort_assets_by_type() {
        let media_index = MediaIndex {
            media: vec![
                indexed_media("a.jpg", ""),
                indexed_media("b.mp4", ""),
                indexed_media("c.mp3", ""),
            ]
        };
        let sizes = vec![
            ("a.jpg".to_string(), 1),
            ("b.mp4".to_string(), 3),
            ("c.mp3".to_string(), 2),
        ].into_iter().collect();

        let result = sort_assets(&&media_index, sizes);

        assert_eq!(result.images[0].indexed_media.get_display_name(), "a.jpg");
        assert_eq!(result.videos[0].indexed_media.get_display_name(), "b.mp4");
        assert_eq!(result.audios[0].indexed_media.get_display_name(), "c.mp3");
    }

    #[test]
    fn test_render_table_should_render_table() {
        let asset_description = AssetDescription {
            indexed_media: AssetDescriptor::new("a.jpg".to_string()),
            file_size: 1,
        };
        let result = render_table("Images", vec![asset_description]);

        assert_eq!(get_first_text_payload(&result), "### Images \n \n | name | size |\n| :-- | :-- |\n| [a.jpg](/assets/a.jpg) | 1 B |\n");
    }

    fn get_first_text_payload(result: &ParsedBlock) -> String {
        result.content.get(0).unwrap().as_tokens.get(0).unwrap().payload.clone()
    }
}