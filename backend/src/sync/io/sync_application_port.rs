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
    ConfigChanged,
}

impl GraphChangeType {
    pub fn description(&self) -> String {
        match self {
            GraphChangeType::PageChanged => "Page changed".to_string(),
            GraphChangeType::PageRenamed => "Page renamed".to_string(),
            GraphChangeType::PageDeleted => "Page deleted".to_string(),
            GraphChangeType::GraphUpdated => "Graph updated".to_string(),
            GraphChangeType::MediaAdded => "Media added".to_string(),
            GraphChangeType::ConfigChanged => "Configuration changed".to_string(),
        }
    }
}

impl Default for GraphChanges {
    fn default() -> Self {
        GraphChanges {
            changes: Mutex::new(HashSet::new()),
        }
    }
}
