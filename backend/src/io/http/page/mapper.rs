use crate::io::http::link_encoding::encode_link_component;
use crate::io::http::page::dtos::{
    MarkdownReferenceDto, PreparedBlockContentDto, PreparedBlockDto, PreparedMarkdownFileDto,
    PreparedReferencedMarkdownDto, UpdateBlockContentDto, UpdateMarkdownFileDto,
};
use crate::io::http::page_type::page_id_to_external_string;
use crate::looksyk::model::{
    PageType, PreparedBlock, PreparedMarkdownFile, PreparedReferencedMarkdown, RawBlock,
    UpdateBlock, UpdateMarkdownFile,
};
use crate::state::block::BlockReference;

pub fn map_to_block_dto(prepared_block: &PreparedBlock) -> PreparedBlockDto {
    PreparedBlockDto {
        indentation: prepared_block.indentation,
        content: PreparedBlockContentDto {
            original_text: prepared_block.content.original_text.clone(),
            prepared_markdown: prepared_block.content.prepared_markdown.clone(),
        },
        referenced_content: prepared_block
            .referenced_markdown
            .iter()
            .map(map_to_prepared_reference_to)
            .collect(),
        has_dynamic_content: prepared_block.has_dynamic_content,
    }
}

impl From<&PreparedBlock> for PreparedBlockDto {
    fn from(prepared_block: &PreparedBlock) -> Self {
        map_to_block_dto(prepared_block)
    }
}

impl From<&PreparedReferencedMarkdown> for PreparedReferencedMarkdownDto {
    fn from(prepared_referenced_markdown: &PreparedReferencedMarkdown) -> Self {
        map_to_prepared_reference_to(prepared_referenced_markdown)
    }
}

pub fn map_to_prepared_reference_to(
    prepared_referenced_markdown: &PreparedReferencedMarkdown,
) -> PreparedReferencedMarkdownDto {
    PreparedReferencedMarkdownDto {
        content: PreparedBlockContentDto {
            original_text: prepared_referenced_markdown.content.original_text.clone(),
            prepared_markdown: prepared_referenced_markdown
                .content
                .prepared_markdown
                .clone(),
        },
        reference: map_markdown_reference_to_dto(&prepared_referenced_markdown.reference),
    }
}

pub fn map_markdown_reference_to_dto(reference: &BlockReference) -> MarkdownReferenceDto {
    MarkdownReferenceDto {
        file_id: page_id_to_external_string(&reference.page_id),
        file_name: reference.page_id.name.name.clone(),
        block_number: reference.block_number,
        link: from_markdown_reference_to_link(reference),
    }
}

pub fn from_markdown_reference_to_link(markdown_reference: &BlockReference) -> String {
    match markdown_reference.page_id.page_type {
        PageType::UserPage => {
            format!(
                "/page/{}",
                encode_link_component(&markdown_reference.page_id.name.name)
            )
        }
        PageType::JournalPage => {
            format!("/journal/{}", markdown_reference.page_id.name.name)
        }
    }
}

pub fn map_markdown_file_to_dto(
    prepared_markdown_file: PreparedMarkdownFile,
    is_fav: bool,
) -> PreparedMarkdownFileDto {
    PreparedMarkdownFileDto {
        is_favourite: is_fav,
        blocks: prepared_markdown_file
            .blocks
            .iter()
            .map(map_to_block_dto)
            .collect(),
    }
}

pub fn map_from_update_markdown_dto(
    update_markdown_file_dto: UpdateMarkdownFileDto,
) -> UpdateMarkdownFile {
    UpdateMarkdownFile {
        blocks: update_markdown_file_dto
            .blocks
            .iter()
            .map(|x| RawBlock {
                indentation: x.indentation,
                text_content: vec![x.markdown.trim().to_string()],
            })
            .collect(),
    }
}

pub fn map_markdown_block_dto(
    update_block_dto: &UpdateBlockContentDto,
    reference: BlockReference,
) -> UpdateBlock {
    UpdateBlock {
        markdown: update_block_dto.markdown.clone(),
        reference,
    }
}

#[cfg(test)]
mod tests {
    use crate::io::http::page::mapper::from_markdown_reference_to_link;
    use crate::looksyk::builder::test_builder::{journal_page_id, user_page_id};
    use crate::state::block::BlockReference;

    #[test]
    fn test_map_markdown_file_to_dto_journal() {
        let markdown_reference = BlockReference {
            page_id: journal_page_id("my-journal"),
            block_number: 0,
        };

        let link = from_markdown_reference_to_link(&markdown_reference);

        assert_eq!(link, "/journal/my-journal");
    }
    #[test]
    fn test_map_markdown_file_to_dto_user_page() {
        let markdown_reference = BlockReference {
            page_id: user_page_id("my-page"),
            block_number: 0,
        };

        let link = from_markdown_reference_to_link(&markdown_reference);

        assert_eq!(link, "/page/my-page");
    }
}
