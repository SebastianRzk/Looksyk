use crate::io::http::kanban::dtos::{KanbanDataDto, KanbanItemDto, KanbanListDto};
use crate::looksyk::kanban::models::{PreparedKanbanData, PreparedKanbanItem, PreparedKanbanList};

impl From<PreparedKanbanData> for KanbanDataDto {
    fn from(val: PreparedKanbanData) -> Self {
        KanbanDataDto {
            title: val.title.title,
            lists: val
                .lists
                .into_iter()
                .map(|list_dto| list_dto.into())
                .collect(),
        }
    }
}

impl From<PreparedKanbanList> for KanbanListDto {
    fn from(val: PreparedKanbanList) -> Self {
        KanbanListDto {
            title: val.title.title,
            items: val.items.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl From<PreparedKanbanItem> for KanbanItemDto {
    fn from(val: PreparedKanbanItem) -> Self {
        KanbanItemDto {
            block: (&val.block).into(),
            priority: val.priority.priority,
        }
    }
}
