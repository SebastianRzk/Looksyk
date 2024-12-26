use crate::looksyk::model::PageId;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct TagIndex {
    pub entries: HashMap<PageId, HashSet<PageId>>,
}

#[cfg(test)]
pub mod builder {
    use crate::state::tag::TagIndex;
    use std::collections::HashMap;

    pub fn empty_tag_index() -> TagIndex {
        TagIndex {
            entries: HashMap::new(),
        }
    }
}
