use crate::looksyk::model::{PreparedReferencedMarkdown, ReferencedMarkdown};

pub struct PreparedKanbanData {
    pub title: KanbanTitle,
    pub lists: Vec<PreparedKanbanList>,
}

pub struct PreparedKanbanList {
    pub title: KanbanListTitle,
    pub items: Vec<PreparedKanbanItem>,
}

pub struct PreparedKanbanItem {
    pub block: PreparedReferencedMarkdown,
    pub priority: KanbanItemPriority,
}

pub struct KanbanData {
    pub title: KanbanTitle,
    pub lists: Vec<KanbanList>,
}

pub struct KanbanList {
    pub title: KanbanListTitle,
    pub items: Vec<KanbanItem>,
}

pub struct KanbanItem {
    pub block: ReferencedMarkdown,
    pub priority: KanbanItemPriority,
}

#[derive(PartialEq, Eq, Debug)]
pub struct KanbanTitle {
    pub title: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct KanbanListTitle {
    pub title: String,
}

pub struct KanbanItemPriority {
    pub priority: String,
}

#[cfg(test)]
pub mod builder {
    use super::{KanbanListTitle, KanbanTitle};

    pub fn kanban_title(title: &str) -> KanbanTitle {
        KanbanTitle {
            title: title.to_string(),
        }
    }

    pub fn kanban_list_title(title: &str) -> KanbanListTitle {
        KanbanListTitle {
            title: title.to_string(),
        }
    }
}
