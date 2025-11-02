use looksyk::looksyk::renderer::renderer_flat::render_block_flat;
use crate::looksyk::kanban::models::{KanbanData, PreparedKanbanData};

pub fn render_kanban(kanban:KanbanData) -> PreparedKanbanData{
    PreparedKanbanData{
        title: kanban.title,
        lists: kanban.lists.into_iter().map(render_kanban_list).collect(),
    }
}

fn render_kanban_list(list:crate::looksyk::kanban::models::KanbanList) -> crate::looksyk::kanban::models::PreparedKanbanList{
    crate::looksyk::kanban::models::PreparedKanbanList{
        title: list.title,
        items: list.items.into_iter().map(|item| item.prepare()).collect(),
    }
}

fn render_kanban_item(item:crate::looksyk::kanban::models::KanbanItem) -> crate::looksyk::kanban::models::PreparedKanbanItem{
    crate::looksyk::kanban::models::PreparedKanbanItem{
        block: render_block,
        priority: item.priority,
    }
}