use std::sync::Mutex;

use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::io::fs::basic_file::{create_folder, exists_folder};
use crate::io::fs::basic_folder::home_directory;
use crate::io::fs::config::{read_config_from_file, save_config_to_file};
use crate::io::fs::env;
use crate::io::fs::media::{init_media, read_media_config, write_media_config};
use crate::io::fs::pages::{read_all_journal_files, read_all_user_files};
use crate::io::fs::root_path::{get_current_active_data_root_location, InitialConfigLocation};
use crate::io::http::design;
use crate::io::http::endpoints::{get_journal, get_overview_page, parse, update_block, update_journal};
use crate::io::http::favourites;
use crate::io::http::media;
use crate::io::http::metainfo;
use crate::io::http::r#static;
use crate::io::http::userpage;
use crate::looksyk::config::config::{Config, Design};
use crate::looksyk::index::media::MediaIndex;
use crate::looksyk::index::tag::create_tag_index;
use crate::looksyk::index::todo::create_todo_index;
use crate::looksyk::index::userpage::{create_journal_page_index, create_user_page_index};
use crate::state::state::{AppState, DataRootLocation};

mod looksyk;
mod state;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_config_path = env::get_or_default("LOOKSYK_CONFIG_PATH", home_directory().join(".local").join("share").join("looksyk").to_str().unwrap());
    let data_root_location = get_current_active_data_root_location(&InitialConfigLocation {
        path: initial_config_path
    });

    if !exists_folder(data_root_location.path.to_path_buf()) {
        init_empty_graph(&data_root_location);
    }

    let app_state = create_app_state(data_root_location);

    let port = 8989;

    println!("Starting Looksyk on Port {}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(parse)
            .service(update_block)
            .service(get_overview_page)
            .service(get_journal)
            .service(update_journal)
            .service(userpage::endpoints::get_page)
            .service(userpage::endpoints::update_page)
            .service(userpage::endpoints::get_backlinks)
            .service(favourites::endpoints::insert_favourite)
            .service(favourites::endpoints::delete_favourite)
            .service(favourites::endpoints::get_favourites)
            .service(favourites::endpoints::update_favourites)
            .service(media::endpoints::post_file)
            .service(design::endpoints::css_theme)
            .service(metainfo::endpoints::get_metainfo)
            .service(r#static::endpoints::index_html)
            .service(r#static::endpoints::css)
            .service(r#static::endpoints::js)
            .service(r#static::endpoints::font_css)
            .service(r#static::endpoints::font_garamond)
            .service(r#static::endpoints::font_material)
            .service(r#static::endpoints::emoji)
            .service(r#static::endpoints::asset_js)
            .service(media::endpoints::assets)
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await
}

fn init_empty_graph(data_root_location: &DataRootLocation) {
    create_folder(data_root_location.path.join("assets"));
    create_folder(data_root_location.path.join("config"));
    write_media_config(&data_root_location, &MediaIndex {
        media: vec![]
    });
    save_config_to_file(&data_root_location, &Config {
        favourites: vec![],
        design: Design {
            primary_color: "#0c884c".to_string(),
            background_color: "rgb(20, 20, 20)".to_string(),
            foreground_color: "white".to_string(),
            primary_shading: "rgba(255, 255, 255, 0.1)".to_string(),
        },
    });

    create_folder(data_root_location.path.join("journals"));
    create_folder(data_root_location.path.join("pages"));
}

fn create_app_state(data_root_location: DataRootLocation) -> Data<AppState> {
    let mut media_index = read_media_config(&data_root_location);
    media_index = init_media(&data_root_location, &media_index);
    write_media_config(&data_root_location, &media_index);


    let config = read_config_from_file(&data_root_location);
    let all_pages = read_all_user_files(&data_root_location);
    let all_journals = read_all_journal_files(&data_root_location);
    let user_page_index = create_user_page_index(all_pages);
    let journal_index = create_journal_page_index(all_journals);
    let todo_index = create_todo_index(&user_page_index, &journal_index);
    let tag_index = create_tag_index(&user_page_index, &journal_index);

    println!("all data refreshed");

    let app_state = Data::new(AppState {
        media_index: Mutex::new(media_index),
        data_path: data_root_location,
        config: Mutex::new(config),
        user_pages: Mutex::new(user_page_index.clone()),
        todo_index: Mutex::new(todo_index.clone()),
        tag_index: Mutex::new(tag_index.clone()),
        journal_pages: Mutex::new(journal_index.clone()),
    });
    app_state
}


