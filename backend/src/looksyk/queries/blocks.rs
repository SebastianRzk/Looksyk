use crate::looksyk::model::{
    PageId, PageType, ParsedBlock, ParsedMarkdownFile, QueryRenderResult, ReferencedMarkdown,
    SimplePageName,
};
use crate::looksyk::queries::args::{
    parse_display_type, parse_property, ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX, PARAM_TAG,
    PARAM_TARGET,
};
use crate::looksyk::queries::basic::count::render_as_count;
use crate::looksyk::queries::basic::unknown::render_display_unknown;
use crate::looksyk::query::{Query, QueryDisplayType, QueryType};
use crate::looksyk::renderer::{render_block_flat, render_block_link, render_user_link};
use crate::state::block::BlockReference;
use crate::state::journal::JournalPageIndex;
use crate::state::tag::TagIndex;
use crate::state::userpage::UserPageIndex;
use std::collections::{HashMap, HashSet};
use std::io::Error;

pub const QUERY_NAME_BLOCKS: &str = "blocks";

pub fn parse_query_blocks(query_str: &str) -> Result<Query, Error> {
    let query_content = query_str
        .strip_prefix(QUERY_NAME_BLOCKS)
        .ok_or(Error::other(ERROR_CAN_NOT_STRIP_QUERY_NAME_PREFIX))?
        .trim();
    let query_tag = parse_property(query_content, PARAM_TAG)?;

    let display_type = parse_display_type(query_tag.remaining_text.clone())?;

    let mut args1 = HashMap::new();
    args1.insert(PARAM_TARGET.to_string(), query_tag.value);
    let (display_type, args) = (display_type, args1);
    Ok(Query {
        query_type: QueryType::Blocks,
        display: display_type,
        args,
    })
}

pub fn render_blocks_query(
    query: Query,
    tag_index: &TagIndex,
    user_page_index: &UserPageIndex,
    journal_page_index: &JournalPageIndex,
) -> QueryRenderResult {
    let target = SimplePageName {
        name: query.args.get(PARAM_TARGET).unwrap().clone(),
    };

    let empty_set: HashSet<PageId> = HashSet::new();
    let mut references: Vec<&PageId> = tag_index
        .entries
        .get(&target.as_user_page())
        .unwrap_or(&empty_set)
        .iter()
        .collect();
    references.sort();

    let resolved_blocks = resolve_blocks(&target, &references, user_page_index, journal_page_index);

    match query.display {
        QueryDisplayType::InplaceList => render_as_list(&target, &resolved_blocks),
        QueryDisplayType::Count => render_as_count(&resolved_blocks),
        QueryDisplayType::ReferencedList => render_as_referenced_list(&resolved_blocks),
        QueryDisplayType::Paragraphs => render_as_paragraph(&target, &resolved_blocks),
        _ => render_display_unknown(
            query.display,
            vec![
                QueryDisplayType::InplaceList,
                QueryDisplayType::ReferencedList,
                QueryDisplayType::Count,
                QueryDisplayType::Paragraphs,
            ],
        ),
    }
}

fn resolve_blocks(
    target: &SimplePageName,
    page_ids: &[&PageId],
    user_page_index: &UserPageIndex,
    journal_page_index: &JournalPageIndex,
) -> Vec<BlockQueryResult> {
    let mut result = vec![];

    for page_id in page_ids.iter() {
        let blocks = match page_id.page_type {
            PageType::UserPage => resolve_blocks_in_page(
                target,
                page_id,
                user_page_index.entries.get(&page_id.name).unwrap(),
            ),
            PageType::JournalPage => resolve_blocks_in_page(
                target,
                page_id,
                journal_page_index.entries.get(&page_id.name).unwrap(),
            ),
        };
        result.extend(blocks);
    }

    result
}

fn resolve_blocks_in_page(
    target: &SimplePageName,
    page_id: &PageId,
    page: &ParsedMarkdownFile,
) -> Vec<BlockQueryResult> {
    let mut result = vec![];
    for (index, block) in page.blocks.iter().enumerate() {
        if block.contains_reference(target) {
            result.push(BlockQueryResult {
                block_reference: BlockReference {
                    page_id: page_id.clone(),
                    block_number: index,
                },
                parsed_block: block.clone(),
            });
        }
    }
    result
}

struct BlockQueryResult {
    parsed_block: ParsedBlock,
    block_reference: BlockReference,
}

fn render_as_list(tag: &SimplePageName, refs: &[BlockQueryResult]) -> QueryRenderResult {
    let mut result = format!("Blocks that reference {}:\n\n", render_user_link(tag));
    for r in refs.iter() {
        result.push_str(
            format!(
                "* {}:{}\n",
                render_block_link(&r.block_reference),
                render_block_flat(&r.parsed_block).content.prepared_markdown
            )
            .as_str(),
        );
    }
    if refs.is_empty() {
        result.push_str("* No blocks found!\n");
    }

    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: result,
        has_dynamic_content: false,
    }
}

fn render_as_paragraph(tag: &SimplePageName, refs: &[BlockQueryResult]) -> QueryRenderResult {
    let mut result = format!("Blocks that reference {}:\n\n", render_user_link(tag));
    for r in refs.iter() {
        result.push_str(
            format!(
                "### {}\n\n{}\n\n---\n\n",
                render_block_link(&r.block_reference),
                render_block_flat(&r.parsed_block).content.prepared_markdown
            )
            .as_str(),
        );
    }
    if refs.is_empty() {
        result.push_str("* No blocks found!\n");
    }

    QueryRenderResult {
        referenced_markdown: vec![],
        inplace_markdown: result,
        has_dynamic_content: false,
    }
}

fn render_as_referenced_list(refs: &[BlockQueryResult]) -> QueryRenderResult {
    let mut result = vec![];
    for r in refs.iter() {
        result.push(ReferencedMarkdown {
            content: r.parsed_block.clone(),
            reference: r.block_reference.clone(),
        });
    }

    QueryRenderResult {
        referenced_markdown: result,
        inplace_markdown: "".to_string(),
        has_dynamic_content: false,
    }
}

#[cfg(test)]
mod tests {
    use super::render_blocks_query;
    use crate::looksyk::builder::test_builder::empty_journal_index;
    use crate::looksyk::builder::{link_token, page_name_str, text_token_str};
    use crate::looksyk::model::{BlockContent, ParsedBlock};
    use crate::looksyk::queries::args::PARAM_TARGET;
    use crate::looksyk::query::Query;
    use crate::looksyk::renderer::render_block_flat;
    use crate::state::tag::builder::empty_tag_index;
    use crate::state::tag::TagIndex;
    use crate::state::userpage::builder::empty_user_page_index;
    use crate::state::userpage::UserPageIndex;
    use std::collections::HashSet;

    #[test]
    fn test_parse_query() {
        let query = "blocks tag:\"foo\" display:\"inplace-list\"";
        let result = super::parse_query_blocks(query).unwrap();
        assert_eq!(result.query_type, super::QueryType::Blocks);
        assert_eq!(result.display, super::QueryDisplayType::InplaceList);
        assert_eq!(result.args.get(PARAM_TARGET).unwrap(), "foo");
    }

    #[test]
    fn test_render_blocks_query_as_inplace_list_with_empty_db_should_show_empty() {
        let result = render_blocks_query(
            inplace_list_query(),
            &empty_tag_index(),
            &empty_user_page_index(),
            &empty_journal_index(),
        );
        assert_eq!(
            result.inplace_markdown,
            "Blocks that reference [foo](page/foo):\n\n* No blocks found!\n"
        );
    }

    fn inplace_list_query() -> Query {
        Query {
            query_type: super::QueryType::Blocks,
            display: super::QueryDisplayType::InplaceList,
            args: vec![(PARAM_TARGET.to_string(), "foo".to_string())]
                .into_iter()
                .collect(),
        }
    }

    fn paragraph_query() -> Query {
        Query {
            query_type: super::QueryType::Blocks,
            display: super::QueryDisplayType::Paragraphs,
            args: vec![(PARAM_TARGET.to_string(), "foo".to_string())]
                .into_iter()
                .collect(),
        }
    }

    fn matching_text_block() -> ParsedBlock {
        ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_text: "".to_string(),
                as_tokens: vec![
                    text_token_str("before"),
                    link_token("foo"),
                    text_token_str("after"),
                ],
            }],
        }
    }

    #[test]
    fn test_render_blocks_query_as_inplace_list_with_one_block_should_show_block() {
        let result = render_blocks_query(
            inplace_list_query(),
            &tag_index_with_existing_tag(),
            &user_page_index_with_existing_page(),
            &empty_journal_index(),
        );
        assert_eq!(result.inplace_markdown, "Blocks that reference [foo](page/foo):\n\n* [referencing:0](page/referencing):before [foo](page/foo) after\n");
    }

    #[test]
    fn test_render_blocks_query_as_paragraphs_with_one_block_should_show_block() {
        let result = render_blocks_query(
            paragraph_query(),
            &tag_index_with_existing_tag(),
            &user_page_index_with_existing_page(),
            &empty_journal_index(),
        );
        assert_eq!(result.inplace_markdown, "Blocks that reference [foo](page/foo):\n\n### [referencing:0](page/referencing)\n\nbefore [foo](page/foo) after\n\n---\n\n");
    }

    fn tag_index_with_existing_tag() -> TagIndex {
        let mut entries = std::collections::HashMap::new();
        let mut refs = HashSet::new();
        refs.insert(page_name_str("referencing").as_user_page());
        entries.insert(page_name_str("foo").as_user_page(), refs);

        TagIndex { entries }
    }

    fn user_page_index_with_existing_page() -> UserPageIndex {
        let mut entries = std::collections::HashMap::new();
        entries.insert(
            page_name_str("referencing"),
            crate::looksyk::model::ParsedMarkdownFile {
                blocks: vec![matching_text_block()],
            },
        );

        UserPageIndex { entries }
    }

    #[test]
    fn test_render_blocks_query_as_referenced_list_with_one_block_should_show_block() {
        let result = render_blocks_query(
            referenced_list_query(),
            &tag_index_with_existing_tag(),
            &user_page_index_with_existing_page(),
            &empty_journal_index(),
        );

        assert_eq!(result.referenced_markdown.len(), 1);
        assert_eq!(
            render_block_flat(&result.referenced_markdown[0].content)
                .content
                .prepared_markdown,
            "before [foo](page/foo) after"
        );
        assert_eq!(
            result.referenced_markdown[0].reference,
            crate::state::block::BlockReference {
                page_id: page_name_str("referencing").as_user_page(),
                block_number: 0,
            }
        );
    }

    fn referenced_list_query() -> Query {
        Query {
            query_type: super::QueryType::Blocks,
            display: super::QueryDisplayType::ReferencedList,
            args: vec![(PARAM_TARGET.to_string(), "foo".to_string())]
                .into_iter()
                .collect(),
        }
    }
}
