use crate::io::http::dtos::{MarkdownReferenceDto, PreparedBlockContentDto, PreparedBlockDto, PreparedMarkdownFileDto, PreparedReferencedMarkdownDto, UpdateBlockContentDto, UpdateMarkdownFileDto};
use crate::looksyk::model::{MarkdownReference, PreparedBlock, PreparedMarkdownFile, PreparedReferencedMarkdown, RawBlock, UpdateBlock, UpdateMarkdownFile};

pub fn map_to_block_dto(prepared_block: &PreparedBlock) -> PreparedBlockDto {
    PreparedBlockDto {
        indentation: prepared_block.indentation,
        content: PreparedBlockContentDto {
            original_text: prepared_block.content.original_text.clone(),
            prepared_markdown: prepared_block.content.prepared_markdown.clone(),
        },
        referenced_content: prepared_block.referenced_markdown.iter().map(|x| map_to_reference_to(x)).collect(),
        has_dynamic_content: prepared_block.has_dynamic_content
    }
}

fn map_to_reference_to(prepared_referenced_markdown: &PreparedReferencedMarkdown) -> PreparedReferencedMarkdownDto {
    PreparedReferencedMarkdownDto {
        content: PreparedBlockContentDto {
            original_text: prepared_referenced_markdown.content.original_text.clone(),
            prepared_markdown: prepared_referenced_markdown.content.prepared_markdown.clone(),
        },
        reference: MarkdownReferenceDto {
            file_id: prepared_referenced_markdown.reference.page_id.id.clone(),
            file_name: prepared_referenced_markdown.reference.page_name.name.clone(),
            block_number: prepared_referenced_markdown.reference.block_number,
        },
    }
}


pub fn map_markdown_file_to_dto(prepared_markdown_file: PreparedMarkdownFile, is_fav: bool) -> PreparedMarkdownFileDto {
    PreparedMarkdownFileDto {
        is_favourite: is_fav,
        blocks: prepared_markdown_file.blocks.iter().map(|x| map_to_block_dto(x)).collect()
    }
}


pub fn map_from_update_markdown_dto(update_markdown_file_dto: UpdateMarkdownFileDto) -> UpdateMarkdownFile {
    UpdateMarkdownFile {
        blocks: update_markdown_file_dto.blocks.iter().map(|x| RawBlock {
            indentation: x.indentation,
            text_content: vec![x.markdown.trim().to_string()],
        }).collect()
    }
}

pub fn map_markdown_block_dto(update_block_dto: &UpdateBlockContentDto, reference: MarkdownReference) -> UpdateBlock {
    UpdateBlock {
        markdown: update_block_dto.markdown.clone(),
        reference,
    }
}