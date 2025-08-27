use crate::io::fs::config::read_config_from_file;
use crate::io::fs::media::{init_media, read_media_config, write_media_config};
use crate::io::fs::pages::{read_all_journal_files, read_all_user_files};
use crate::looksyk::index::asset::create_empty_asset_cache;
use crate::looksyk::index::tag::create_tag_index;
use crate::looksyk::index::todo::create_todo_index;
use crate::looksyk::index::userpage::{create_journal_page_index, create_user_page_index};
use crate::state::application_state::{GraphRootLocation, PureAppState};

pub fn load_graph_data(data_root_location: &GraphRootLocation) -> PureAppState {
    let mut media_index = read_media_config(data_root_location);
    media_index = init_media(data_root_location, &media_index);
    write_media_config(data_root_location, &media_index);

    let config = read_config_from_file(data_root_location);
    let all_pages = read_all_user_files(data_root_location);
    let all_journals = read_all_journal_files(data_root_location);
    let user_page_index = create_user_page_index(all_pages);
    let journal_index = create_journal_page_index(all_journals);
    let todo_index = create_todo_index(&user_page_index, &journal_index);
    let tag_index = create_tag_index(&user_page_index, &journal_index);
    let asset_cache = create_empty_asset_cache();

    println!("all data refreshed");

    PureAppState {
        data_path: data_root_location.clone(),
        a_user_pages: user_page_index,
        b_journal_pages: journal_index,
        c_todo_index: todo_index,
        d_tag_index: tag_index,
        e_asset_cache: asset_cache,
        f_media_index: media_index,
        g_config: config,
    }
}
