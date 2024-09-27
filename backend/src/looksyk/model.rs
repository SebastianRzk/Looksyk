use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct RawBlock {
    pub indentation: usize,
    pub text_content: Vec<String>,
}


#[derive(Clone)]
pub struct ParsedBlock {
    pub indentation: usize,
    pub content: Vec<BlockContent>,
}


#[cfg(test)]
pub mod builder {
    use crate::looksyk::builder::text_token;
    use crate::looksyk::model::{BlockContent, BlockToken, ParsedBlock};

    pub fn block_with_text_content(content: &str) -> ParsedBlock {
        ParsedBlock {
            indentation: 0,
            content: vec![BlockContent {
                as_text: content.to_string(),
                as_tokens: vec![text_token(content)],
            }],
        }
    }


    pub fn query_block_token(query_payload: &str) -> BlockToken {
        BlockToken {
            block_token_type: super::BlockTokenType::QUERY,
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

#[derive(Clone)]
pub struct PreparedBlockContent {
    pub original_text: String,
    pub prepared_markdown: String,
}

#[derive(Clone)]
pub struct BlockContent {
    pub as_text: String,
    pub as_tokens: Vec<BlockToken>,
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum BlockTokenType {
    TEXT,
    LINK,
    JOURNALLINK,
    QUERY,
    TODO,
}

#[derive(Clone, Debug, PartialEq)]
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


impl Display for RawMarkdownFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.blocks)
    }
}

pub struct UpdateBlock {
    pub markdown: String,
    pub reference: MarkdownReference,
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
    pub reference: MarkdownReference,
}

#[derive(Clone)]
pub struct MarkdownReference {
    pub page_id: PageId,
    pub page_name: SimplePageName,
    pub block_number: usize,
}

#[derive(Clone)]
pub struct PreparedReferencedMarkdown {
    pub content: PreparedBlockContent,
    pub reference: MarkdownReference,
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
        self.id.hash(state)
    }
}


impl PartialEq for PageId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PageId {}


#[derive(Clone, Debug)]
pub struct PageId {
    pub id: String,
}