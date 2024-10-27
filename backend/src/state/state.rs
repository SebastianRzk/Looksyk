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
    pub title: String,
    pub data_path: DataRootLocation,
    pub a_user_pages: Mutex<UserPageIndex>,
    pub b_journal_pages: Mutex<JournalPageIndex>,
    pub c_todo_index: Mutex<TodoIndex>,
    pub d_tag_index: Mutex<TagIndex>,
    pub e_asset_cache: Mutex<AssetCache>,
    pub f_media_index: Mutex<MediaIndex>,
    pub g_config: Mutex<Config>,
}

pub struct DataRootLocation {
    pub path: PathBuf,
}

#[cfg(test)]
pub mod builder {
    use std::path::Path;
    use crate::state::state::DataRootLocation;

    pub fn empty_data_root_location() -> DataRootLocation {
        DataRootLocation {
            path: Path::new("").to_path_buf(),
        }
    }
}


pub struct CurrentPageAssociatedState<'a> {
    pub user_pages: &'a UserPageIndex,
    pub journal_pages: &'a JournalPageIndex,
    pub todo_index: &'a TodoIndex,
    pub tag_index: &'a TagIndex,
}


pub struct NewPageAssociatedState {
    pub user_pages: UserPageIndex,
    pub journal_pages: JournalPageIndex,
    pub todo_index: TodoIndex,
    pub tag_index: TagIndex,
}



pub struct CurrentPageOnDiskState<'a> {
    pub user_pages: &'a UserPageIndex,
    pub journal_pages: &'a JournalPageIndex,
}

#[derive(Clone)]
pub struct NewPageOnDiskState {
    pub user_pages: UserPageIndex,
    pub journal_pages: JournalPageIndex,
}







