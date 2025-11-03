use crate::looksyk::kanban::models::{KanbanData, KanbanItem, KanbanList, PreparedKanbanData};
use crate::looksyk::renderer::model::StaticRenderContext;
use crate::looksyk::renderer::renderer_deep::render_block;
use crate::state::application_state::GraphRootLocation;
use crate::state::asset_cache::AssetCache;

pub fn render_kanban(
    kanban: KanbanData,
    static_render_context: &StaticRenderContext,
    asset_cache: &mut AssetCache,
    graph_root_location: &GraphRootLocation,
) -> PreparedKanbanData {
    PreparedKanbanData {
        title: kanban.title,
        lists: kanban
            .lists
            .into_iter()
            .map(|x| render_kanban_list(x, static_render_context, asset_cache, graph_root_location))
            .collect(),
    }
}

fn render_kanban_list(
    list: KanbanList,
    static_render_context: &StaticRenderContext,
    asset_cache: &mut AssetCache,
    graph_root_location: &GraphRootLocation,
) -> crate::looksyk::kanban::models::PreparedKanbanList {
    crate::looksyk::kanban::models::PreparedKanbanList {
        title: list.title,
        items: list
            .items
            .into_iter()
            .map(|x| render_kanban_item(x, static_render_context, asset_cache, graph_root_location))
            .collect(),
    }
}

fn render_kanban_item(
    item: KanbanItem,
    static_render_context: &StaticRenderContext,
    asset_cache: &mut AssetCache,
    graph_root_location: &GraphRootLocation,
) -> crate::looksyk::kanban::models::PreparedKanbanItem {
    crate::looksyk::kanban::models::PreparedKanbanItem {
        block: render_block(
            &item.block.content,
            static_render_context,
            asset_cache,
            graph_root_location,
        )
        .reference(item.block.reference),
        priority: item.priority,
    }
}
