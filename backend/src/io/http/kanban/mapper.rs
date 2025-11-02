use crate::io::http::kanban::dtos::{KanbanDataDto, KanbanItemDto, KanbanListDto};
use crate::looksyk::kanban::models::{
    PreparedKanbanData, PreparedKanbanItem, PreparedKanbanList,
};

impl Into<KanbanDataDto> for PreparedKanbanData {
    fn into(self) -> KanbanDataDto {
        KanbanDataDto {
            title: self.title.title,
            lists: self
                .lists
                .into_iter()
                .map(|list_dto| list_dto.into())
                .collect(),
        }
    }
}

impl Into<KanbanListDto> for PreparedKanbanList {
    fn into(self) -> KanbanListDto {
        KanbanListDto {
            title: self.title.title,
            items: self.items.into_iter().map(|item| item.into()).collect(),
        }
    }
}

impl Into<KanbanItemDto> for PreparedKanbanItem {
    fn into(self) -> KanbanItemDto {
        KanbanItemDto {
            block: (&self.block).into(),
            priority: self.priority.priority,
        }
    }
}
