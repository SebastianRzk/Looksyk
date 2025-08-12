use std::collections::HashSet;
use std::sync::Mutex;

pub struct GraphChanges {
    pub changes: Mutex<HashSet<GraphChange>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphChange {
    pub change_type: GraphChangeType,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphChangeType {
    PageChanged,
    PageRenamed,
    PageDeleted,
    GraphUpdated,
    MediaAdded,
    ConfigChanged
}

impl Default for GraphChanges {
    fn default() -> Self {
        GraphChanges {
            changes: Mutex::new(HashSet::new()),
        }
    }
}
