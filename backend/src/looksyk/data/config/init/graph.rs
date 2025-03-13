use crate::io::fs::basic_file::{create_folder, exists_folder, folder_empty};
use crate::io::fs::config::save_config_to_file;
use crate::io::fs::media::write_media_config;
use crate::looksyk::data::config::init::theme::init_empty_user_theme_if_non_existent;
use crate::looksyk::data::config::runtime_graph_configuration::{Config, Design};
use crate::looksyk::index::media::MediaIndex;
use crate::state::application_state::GraphRootLocation;

pub fn init_graph_if_needed(data_root_location: &GraphRootLocation) {
    if !exists_folder(data_root_location.path.to_path_buf())
        || folder_empty(data_root_location.path.to_path_buf())
    {
        init_empty_graph(data_root_location);
    }
    init_empty_user_theme_if_non_existent(data_root_location);
}

fn init_empty_graph(data_root_location: &GraphRootLocation) {
    create_folder(data_root_location.path.join("assets"));
    create_folder(data_root_location.path.join("config"));
    write_media_config(data_root_location, &MediaIndex { media: vec![] });
    save_config_to_file(
        data_root_location,
        &Config {
            favourites: vec![],
            design: Design {
                primary_color: "#0c884c".to_string(),
                background_color: "#15212D".to_string(),
                foreground_color: "white".to_string(),
                primary_shading: "rgba(255, 255, 255, 0.1)".to_string(),
            },
            title: Some("No Graph Title".to_string()),
        },
    );

    create_folder(data_root_location.path.join("journals"));
    create_folder(data_root_location.path.join("pages"));
}
