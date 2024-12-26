use crate::looksyk::model::{PageId, PageType, ParsedBlock, SimplePageName};

#[derive(Clone)]
pub struct TodoIndex {
    pub entries: Vec<TodoIndexEntry>,
}

#[derive(Clone)]
pub struct TodoIndexEntry {
    pub state: TodoState,
    pub source: TodoSourceReference,
    pub block: ParsedBlock,
    pub tags: Vec<SimplePageName>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TodoSourceReference {
    pub page_id: PageId,
    pub page_name: SimplePageName,
    pub blocknumber: usize,
    pub page_type: PageType,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TodoState {
    Todo,
    Done,
}

#[cfg(test)]
pub mod builder {
    use crate::state::todo::TodoIndex;

    pub fn empty_todo_index() -> TodoIndex {
        TodoIndex { entries: vec![] }
    }
}
