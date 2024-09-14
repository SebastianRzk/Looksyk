use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crate::io::fs::asset_cache_loader::load_cachable_asset;
use crate::io::fs::media::MediaOnDisk;
use crate::io::http::media::config::create_media_location;
use crate::looksyk::datatypes::AssetDescriptor;
use crate::looksyk::markdown::{render_as_audio, render_as_code_block, render_as_link, render_as_video};
use crate::looksyk::model::QueryRenderResult;
use crate::looksyk::queries::args::{ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX, PARAM_TARGET_FILE, parse_display_type_for_inplace, parse_property};
use crate::looksyk::queries::unknown::render_display_unknown;
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use crate::state::asset_cache::{AssetCache, AssetState};
use crate::state::state::DataRootLocation;

pub const QUERY_NAME_INSERT_FILE_CONTENT: &str = "insert-file-content";

pub fn query_insert_file_content_as_text(asset_descriptor: &AssetDescriptor) -> String {
    format!("{{query: insert-file-content target-file:\"{}\" display:\"inline-text\" }}", asset_descriptor.get_display_name())
}

pub fn query_insert_file_content_as_code(asset_descriptor: &AssetDescriptor) -> String {
    format!("{{query: insert-file-content target-file:\"{}\" display:\"code-block\" }}", asset_descriptor.get_display_name())
}

pub fn query_insert_file_content_as_video(asset_descriptor: &AssetDescriptor) -> String {
    format!("{{query: insert-file-content target-file:\"{}\" display:\"video\" }}", asset_descriptor.get_display_name())
}

pub fn query_insert_file_content_as_audio(asset_descriptor: &AssetDescriptor) -> String {
    format!("{{query: insert-file-content target-file:\"{}\" display:\"audio\" }}", asset_descriptor.get_display_name())
}


pub fn parse_query_insert_file_content(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str.strip_prefix(QUERY_NAME_INSERT_FILE_CONTENT).ok_or(Error::new(ErrorKind::Other, ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX))?.trim();
    let query_target_opt = parse_property(query_content, PARAM_TARGET_FILE)?;

    let display_type = parse_display_type_for_inplace(query_target_opt.remaining_text.clone())?;

    let mut args1 = HashMap::new();
    args1.insert(PARAM_TARGET_FILE.to_string(), query_target_opt.value);
    let (display_type, args) = (display_type, args1);
    Ok(Query {
        query_type: QueryType::InsertFileContent,
        display: display_type,
        args,
    })
}


pub fn render_query_insert_file_content(query: Query, data: &mut AssetCache, data_root_location: &DataRootLocation) -> QueryRenderResult {
    let media_on_disk = MediaOnDisk {
        name: query.args.get(PARAM_TARGET_FILE).unwrap().clone()
    };
    match query.display {
        QueryDisplayType::InlineText => render_inline(&media_on_disk, data, data_root_location),
        QueryDisplayType::Link => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: render_as_link(&media_on_disk.name, &create_media_location(&media_on_disk.name)),
            referenced_markdown: vec![],
        },
        QueryDisplayType::CodeBlock => render_code_block(&media_on_disk, data, data_root_location),
        QueryDisplayType::Video => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: render_as_video(&media_on_disk.as_asset_descriptor()),
            referenced_markdown: vec![],
        },
        QueryDisplayType::Audio => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: render_as_audio(&media_on_disk.as_asset_descriptor()),
            referenced_markdown: vec![],
        },
        _ => render_display_unknown(query.display)
    }
}


fn render_code_block(file_name: &MediaOnDisk, cache: &mut AssetCache, data_root_location: &DataRootLocation) -> QueryRenderResult {
    let mut cache_item = cache.get(&file_name);
    if AssetState::Miss == cache_item {
        cache_item = load_cachable_asset(file_name, data_root_location);
        cache.insert(file_name, cache_item.clone());
    }
    match cache_item {
        AssetState::Found(content) => {
            QueryRenderResult {
                has_dynamic_content: false,
                inplace_markdown: render_as_code_block(infer_language(&file_name), &content.content),
                referenced_markdown: vec![],
            }
        }
        AssetState::NotFound => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: "File not found".to_string(),
            referenced_markdown: vec![],
        },
        AssetState::NotText => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: format!("File is not a text file. Can not inline a binary file. Try display type \"link\" to render a link: {}", render_as_link(&file_name.name, &create_media_location(&file_name.name))),
            referenced_markdown: vec![],
        },
        AssetState::TooLarge(violation) => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: format!("File is too large. Max size is {}. File size is {}. Try display type \"link\" to render a link: {}", violation.max_size, violation.file_size, render_as_link(&file_name.name, &create_media_location(&file_name.name))),
            referenced_markdown: vec![],
        },
        _ => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: "Unknown error".to_string(),
            referenced_markdown: vec![],
        }
    }
}

fn infer_language(file_name: &MediaOnDisk) -> String {
    let file_name_str = file_name.name.as_str();
    if !file_name_str.contains('.') {
        return "text".to_string();
    }
    if file_name_str.ends_with(".rs") {
        return "rust".to_string();
    }
    if file_name_str.ends_with(".py") {
        return "python".to_string();
    }
    if file_name_str.ends_with(".js") {
        return "javascript".to_string();
    }
    if file_name_str.ends_with(".ts") {
        return "typescript".to_string();
    }
    if file_name_str.ends_with(".h") {
        return "c".to_string();
    }
    if file_name_str.ends_with(".hpp") {
        return "cpp".to_string();
    }
    return file_name_str.split('.').last().unwrap().to_string();
}

fn render_inline(file_name: &MediaOnDisk, cache: &mut AssetCache, data_root_location: &DataRootLocation) -> QueryRenderResult {
    let mut cache_item = cache.get(&file_name);
    if AssetState::Miss == cache_item {
        cache_item = load_cachable_asset(file_name, data_root_location);
        cache.insert(file_name, cache_item.clone());
    }
    match cache_item {
        AssetState::Found(content) => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: content.content,
            referenced_markdown: vec![],
        },
        AssetState::NotFound => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: "File not found".to_string(),
            referenced_markdown: vec![],
        },
        AssetState::NotText => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: format!("File is not a text file. Can not inline a binary file. Try display type \"link\" to render a link: {}", render_as_link(&file_name.name, &create_media_location(&file_name.name))),
            referenced_markdown: vec![],
        },
        AssetState::TooLarge(violation) => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: format!("File is too large. Max size is {}. File size is {}. Try display type \"link\" to render a link: {}", violation.max_size, violation.file_size, render_as_link(&file_name.name, &create_media_location(&file_name.name))),
            referenced_markdown: vec![],
        },
        _ => QueryRenderResult {
            has_dynamic_content: false,
            inplace_markdown: "Unknown error".to_string(),
            referenced_markdown: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::asset_descriptor;
    use super::*;

    #[test]
    pub fn test_query_insert_file_content_as_text_with_file_path_should_return_query_string() {
        let result = query_insert_file_content_as_text(&asset_descriptor("file_path"));

        assert_eq!(result, "{query: insert-file-content target-file:\"file_path\" display:\"inline-text\" }");
    }
    #[test]
    pub fn test_query_insert_file_content_as_code_with_file_path_should_return_query_string() {
        let result = query_insert_file_content_as_code(&asset_descriptor("file_path"));

        assert_eq!(result, "{query: insert-file-content target-file:\"file_path\" display:\"code-block\" }");
    }

    #[test]
    pub fn test_query_insert_file_content_as_video_with_file_path_should_return_query_string() {
        let result = query_insert_file_content_as_video(&asset_descriptor("file_path"));

        assert_eq!(result, "{query: insert-file-content target-file:\"file_path\" display:\"video\" }");
    }

    #[test]
    pub fn test_query_insert_file_content_as_audio_with_file_path_should_return_query_string() {
        let result = query_insert_file_content_as_audio(&asset_descriptor("file_path"));

        assert_eq!(result, "{query: insert-file-content target-file:\"file_path\" display:\"audio\" }");
    }
}