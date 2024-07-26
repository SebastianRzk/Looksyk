use std::sync::Mutex;

use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::io::fs::config::read_config_from_file;
use crate::io::fs::media::{init_media, read_media_config, write_media_config};
use crate::io::fs::pages::{read_all_journal_files, read_all_user_files};
use crate::io::http::endpoints::{get_journal, get_overview_page, parse, update_block, update_journal};
use crate::io::http::favourites;
use crate::io::http::media;
use crate::io::http::design;
use crate::io::http::userpage;
use crate::io::http::metainfo;
use crate::looksyk::index::tag::create_tag_index;
use crate::looksyk::index::todo::create_todo_index;
use crate::looksyk::index::userpage::{create_journal_page_index, create_user_page_index};
use crate::state::state::{AppState, DataRootLocation};

mod looksyk;
mod state;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let data_root_location = DataRootLocation {
        path: "./data/".to_string()
    };

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
            .service(media::endpoints::assets)
            .service(design::endpoints::css_theme)
            .service(metainfo::endpoints::get_metainfo)
    })
        .bind(("127.0.0.1", 8989))?
        .run()
        .await
}


