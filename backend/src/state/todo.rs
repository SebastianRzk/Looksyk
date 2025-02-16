use crate::looksyk::model::{ParsedBlock, SimplePageName};
use crate::state::block::BlockReference;

#[derive(Clone)]
pub struct TodoIndex {
    pub entries: Vec<TodoIndexEntry>,
}

#[derive(Clone)]
pub struct TodoIndexEntry {
    pub state: TodoState,
    pub source: BlockReference,
    pub block: ParsedBlock,
    pub tags: Vec<SimplePageName>,
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
