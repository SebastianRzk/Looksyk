use crate::state::application_state::GraphRootLocation;
use std::path::PathBuf;

pub fn custom_user_theme_path(user_data_path: &GraphRootLocation) -> PathBuf {
    user_data_path.path.join("config").join("user-theme.css")
}
