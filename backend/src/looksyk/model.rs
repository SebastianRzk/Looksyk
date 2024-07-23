use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

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
    JOURNAL_LINK,
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
}

#[derive(Clone)]
pub struct ReferencedMarkdown {
    pub content: ParsedBlock,
    pub refernce: MarkdownReference,
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
        return self.name.hash(state);
    }
}


impl PartialEq for SimplePageName {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name;
    }
}

impl Eq for SimplePageName {}


impl Hash for PageId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return self.id.hash(state);
    }
}


impl PartialEq for PageId {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for PageId {}


#[derive(Clone, Debug)]
pub struct PageId {
    pub id: String,
}