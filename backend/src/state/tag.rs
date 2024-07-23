use std::collections::{HashMap, HashSet};
use crate::looksyk::model::PageId;

#[derive(Clone)]
pub struct TagIndex {
    pub entries: HashMap<PageId, HashSet<PageId>>,
}
