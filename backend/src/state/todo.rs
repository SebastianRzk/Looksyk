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
    use crate::looksyk::builder::test_builder::any_page_id;
    use crate::looksyk::model::{ParsedBlock, SimplePageName};
    use crate::state::block::BlockReference;
    use crate::state::todo::TodoIndex;

    pub fn empty_todo_index() -> TodoIndex {
        TodoIndex { entries: vec![] }
    }

    pub fn todo_index_entry(state: super::TodoState, tag: SimplePageName) -> super::TodoIndexEntry {
        super::TodoIndexEntry {
            state,
            source: BlockReference {
                page_id: any_page_id(),
                block_number: 1,
            },
            block: ParsedBlock::empty(),
            tags: vec![tag],
        }
    }
}
