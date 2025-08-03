use crate::state::block::BlockReference;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct RawBlock {
    pub indentation: usize,
    pub text_content: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ParsedBlock {
    pub indentation: usize,
    pub content: Vec<BlockContent>,
}

impl ParsedBlock {
    pub fn contains_reference(&self, reference: &SimplePageName) -> bool {
        self.content.iter().any(|block_content| {
            block_content.as_tokens.iter().any(|block_token| {
                block_token.block_token_type == BlockTokenType::Link
                    && block_token.payload == reference.name
            })
        })
    }
}

#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::text_token_str;
    use crate::looksyk::model::{BlockContent, BlockToken, ParsedBlock};

    pub fn block_with_text_content(content: &str) -> ParsedBlock {
        ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_text: content.to_string(),
                as_tokens: vec![text_token_str(content)],
            }],
        }
    }

    pub fn query_block_token(query_payload: &str) -> BlockToken {
        BlockToken {
            block_token_type: super::BlockTokenType::Query,
            payload: query_payload.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct PreparedBlock {
    pub indentation: usize,
    pub content: PreparedBlockContent,
    pub referenced_markdown: Vec<PreparedReferencedMarkdown>,
    pub has_dynamic_content: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PreparedBlockContent {
    pub original_text: String,
    pub prepared_markdown: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockContent {
    pub as_text: String,
    pub as_tokens: Vec<BlockToken>,
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum BlockTokenType {
    Text,
    Link,
    JournalLink,
    Query,
    Todo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockToken {
    pub block_token_type: BlockTokenType,
    pub payload: String,
}

#[derive(Clone)]
pub struct PreparedMarkdownFile {
    pub blocks: Vec<PreparedBlock>,
}

#[derive(Clone)]
pub struct RawMarkdownFile {
    pub blocks: Vec<RawBlock>,
}

#[derive(Clone)]
pub struct ParsedMarkdownFile {
    pub blocks: Vec<ParsedBlock>,
}

impl ParsedMarkdownFile {
    pub fn empty() -> Self {
        ParsedMarkdownFile { blocks: vec![] }
    }
}

impl Display for RawMarkdownFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.blocks)
    }
}

pub struct UpdateBlock {
    pub markdown: String,
    pub reference: BlockReference,
}

pub struct UpdateMarkdownFile {
    pub blocks: Vec<RawBlock>,
}

pub struct QueryRenderResult {
    pub inplace_markdown: String,
    pub referenced_markdown: Vec<ReferencedMarkdown>,
    pub has_dynamic_content: bool,
}

#[derive(Clone)]
pub struct ReferencedMarkdown {
    pub content: ParsedBlock,
    pub reference: BlockReference,
}

#[derive(Clone)]
pub struct PreparedReferencedMarkdown {
    pub content: PreparedBlockContent,
    pub reference: BlockReference,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PageType {
    UserPage,
    JournalPage,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplePageName {
    pub name: String,
}

impl SimplePageName {
    pub fn as_journal_page(&self) -> PageId {
        PageId {
            page_type: PageType::JournalPage,
            name: self.clone(),
        }
    }

    pub fn as_user_page(&self) -> PageId {
        PageId {
            page_type: PageType::UserPage,
            name: self.clone(),
        }
    }

    pub fn as_page_id(&self, page_type: &PageType) -> PageId {
        PageId {
            page_type: page_type.clone(),
            name: self.clone(),
        }
    }
}

impl Hash for SimplePageName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl PartialEq for SimplePageName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for SimplePageName {}

impl Hash for PageId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PageId {
    pub page_type: PageType,
    pub name: SimplePageName,
}

pub fn no_text_content() -> String {
    "".to_string()
}

impl PageId {
    pub fn is_user_page(&self) -> bool {
        self.page_type == PageType::UserPage
    }
}

impl PartialOrd<Self> for PageId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PageId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.name.cmp(&other.name.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::looksyk::builder::test_builder::user_page_id;
    use crate::looksyk::builder::{link_token, page_name_str, text_token_str};
    use crate::looksyk::model::{BlockContent, PageId, PageType, ParsedBlock, ParsedMarkdownFile};

    #[test]
    fn test_journal_page_id_should_be_a_journal_page() {
        let page_id = PageId {
            page_type: PageType::JournalPage,
            name: page_name_str("my-page"),
        };
        assert!(!page_id.is_user_page());
    }
    #[test]
    fn test_user_page_id_should_be_a_user_page() {
        let page_id = PageId {
            page_type: PageType::UserPage,
            name: page_name_str("my-page"),
        };
        assert!(page_id.is_user_page());
    }

    #[test]
    fn test_as_journal_page_id_should_be_journal_page_id() {
        let page_id = page_name_str("my-page").as_journal_page();
        assert_eq!(page_id.page_type, PageType::JournalPage);
    }

    #[test]
    fn test_as_user_page_id_should_be_user_page_id() {
        let page_id = page_name_str("my-page").as_user_page();
        assert_eq!(page_id.page_type, PageType::UserPage);
    }

    #[test]
    fn test_as_page_id_should_be_page_id() {
        let page_id = page_name_str("my-page").as_page_id(&PageType::UserPage);
        assert_eq!(page_id.page_type, PageType::UserPage);
    }

    #[test]
    fn test_page_id_should_be_equal() {
        let page_id1 = user_page_id("my-page");
        let page_id2 = user_page_id("my-page");
        assert_eq!(page_id1, page_id2);
    }

    #[test]
    fn test_page_id_should_not_be_equal() {
        let page_id1 = user_page_id("my-page");
        let page_id2 = user_page_id("my-page2");
        assert_ne!(page_id1, page_id2);
    }

    #[test]
    fn test_page_id_sort_should_work() {
        let page_id1 = user_page_id("my-page");
        let page_id2 = user_page_id("my-page2");
        let page_id3 = user_page_id("my-page3");
        let mut vec = vec![page_id3.clone(), page_id1.clone(), page_id2.clone()];
        vec.sort();
        assert_eq!(vec, vec![page_id1, page_id2, page_id3]);
    }

    #[test]
    fn test_parsed_block_contains_reference_with_empty_block_return_false() {
        let page_id = user_page_id("my-page");

        let block = ParsedBlock {
            indentation: 0,
            content: vec![],
        };

        assert!(!block.contains_reference(&page_id.name));
    }

    #[test]
    fn test_parsed_block_contains_reference_with_block_without_link_return_false() {
        let page_id = user_page_id("my-page");

        let block = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_text: "my text".to_string(),
                as_tokens: vec![text_token_str("my-page")],
            }],
        };

        assert!(!block.contains_reference(&page_id.name));
    }

    #[test]
    fn test_parsed_block_contains_reference_with_block_with_link_return_true() {
        let page_id = user_page_id("my-page");

        let block = ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_text: "my text".to_string(),
                as_tokens: vec![link_token("my-page")],
            }],
        };

        assert!(block.contains_reference(&page_id.name));
    }

    #[test]
    fn test_empty_should_return_empty() {
        assert_eq!(ParsedMarkdownFile::empty().blocks.len(), 0);
    }
}
