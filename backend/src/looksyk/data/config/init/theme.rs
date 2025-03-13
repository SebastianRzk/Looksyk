use crate::looksyk::data::config::theme::custom_user_theme_path;
use crate::state::application_state::GraphRootLocation;

pub fn init_empty_user_theme_if_non_existent(graph_root_location: &GraphRootLocation) {
    let user_theme_path = custom_user_theme_path(graph_root_location);
    if !user_theme_path.exists() {
        std::fs::write(user_theme_path, "").expect("Failed to create user theme file");
    }
}
