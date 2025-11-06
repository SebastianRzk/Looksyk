use actix_web::web::Data;
use std::collections::HashSet;
use std::sync::Mutex;

pub struct GraphChangesState {
    pub changes: Mutex<GraphChanges>,
}

pub struct GraphChanges {
    changes: HashSet<GraphChange>,
}

impl GraphChanges {
    pub fn new() -> Self {
        GraphChanges {
            changes: HashSet::new(),
        }
    }

    pub fn from_iter<I: IntoIterator<Item = GraphChange>>(iter: I) -> Self {
        GraphChanges {
            changes: iter.into_iter().collect(),
        }
    }

    pub fn add_change(&mut self, change: GraphChange) {
        self.changes.insert(change);
    }

    pub fn get_changes(&self) -> &HashSet<GraphChange> {
        &self.changes
    }

    pub fn clear(&mut self) {
        self.changes.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphChange {
    pub change_type: GraphChangeType,
    pub target: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GraphChangeType {
    UserPageChanged,
    JournalPageChanged,
    UserPageRenamed,
    UserPageDeleted,
    GraphUpdated,
    MediaAdded,
    ConfigChanged,
}

impl GraphChangeType {
    pub fn description(&self) -> String {
        match self {
            GraphChangeType::JournalPageChanged => "Journal page changed".to_string(),
            GraphChangeType::UserPageChanged => "Wiki Page changed".to_string(),
            GraphChangeType::UserPageRenamed => "Wiki Page renamed".to_string(),
            GraphChangeType::UserPageDeleted => "Page deleted".to_string(),
            GraphChangeType::GraphUpdated => "Graph updated".to_string(),
            GraphChangeType::MediaAdded => "Media added".to_string(),
            GraphChangeType::ConfigChanged => "Configuration changed".to_string(),
        }
    }
}

impl Default for GraphChangesState {
    fn default() -> Self {
        GraphChangesState {
            changes: Mutex::new(GraphChanges::new()),
        }
    }
}

impl GraphChange {
    pub fn configuration_changed(target: String) -> Self {
        GraphChange {
            change_type: GraphChangeType::ConfigChanged,
            target,
        }
    }

    pub fn media_added(target: String) -> Self {
        GraphChange {
            change_type: GraphChangeType::MediaAdded,
            target,
        }
    }

    pub fn journal_page_changed(target: String) -> Self {
        GraphChange {
            change_type: GraphChangeType::JournalPageChanged,
            target,
        }
    }

    pub fn kanban_item_moved(target: String, block_index: usize, from: String, to: String) -> Self {
        GraphChange {
            change_type: GraphChangeType::JournalPageChanged,
            target: format!(
                "Kanban item ({}:{}) moved from {} to {}",
                target, block_index, from, to
            ),
        }
    }

    pub fn user_page_changed(target: String) -> Self {
        GraphChange {
            change_type: GraphChangeType::UserPageChanged,
            target,
        }
    }

    pub fn page_renamed(old_name: String, new_name: String) -> Self {
        GraphChange {
            change_type: GraphChangeType::UserPageRenamed,
            target: format!("{} -> {}", old_name, new_name),
        }
    }

    pub fn user_page_deleted(target: String) -> Self {
        GraphChange {
            change_type: GraphChangeType::UserPageDeleted,
            target,
        }
    }

    pub fn graph_updated(current_version: String) -> Self {
        GraphChange {
            change_type: GraphChangeType::GraphUpdated,
            target: format!("Graph updated to version {}", current_version),
        }
    }
}

pub fn document_change(changes: Data<GraphChangesState>, graph_change: GraphChange) {
    let mut changes = changes.changes.lock().unwrap();
    changes.add_change(graph_change);
    drop(changes);
}
