use std::path::PathBuf;
use std::sync::Mutex;

use crate::looksyk::config::config::Config;
use crate::looksyk::index::media::MediaIndex;
use crate::state::asset_cache::AssetCache;
use crate::state::journal::JournalPageIndex;
use crate::state::tag::TagIndex;
use crate::state::todo::TodoIndex;
use crate::state::userpage::UserPageIndex;

pub struct AppState {
    pub media_index: Mutex<MediaIndex>,
    pub data_path: DataRootLocation,
    pub user_pages: Mutex<UserPageIndex>,
    pub journal_pages: Mutex<JournalPageIndex>,
    pub todo_index: Mutex<TodoIndex>,
    pub tag_index: Mutex<TagIndex>,
    pub config: Mutex<Config>,
    pub asset_cache: Mutex<AssetCache>
}

pub struct DataRootLocation{
    pub path: PathBuf,
}




