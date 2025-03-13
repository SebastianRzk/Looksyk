use crate::state::application_state::{AppState, PureAppState};
use actix_web::web::Data;
use std::sync::Mutex;

pub fn convert_to_app_state(state: PureAppState, static_path: &str) -> Data<AppState> {
    Data::new(AppState {
        data_path: state.data_path,
        static_path: static_path.to_owned(),
        a_user_pages: Mutex::new(state.a_user_pages),
        b_journal_pages: Mutex::new(state.b_journal_pages),
        c_todo_index: Mutex::new(state.c_todo_index),
        d_tag_index: Mutex::new(state.d_tag_index),
        e_asset_cache: Mutex::new(state.e_asset_cache),
        f_media_index: Mutex::new(state.f_media_index),
        g_config: Mutex::new(state.g_config),
    })
}
