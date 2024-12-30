use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Mutex;

use crate::configuration::APPLICATION_HOST;
use crate::io::cli::endpoints::get_cli_args;
use crate::io::fs::basic_file::{create_folder, exists_folder, folder_empty};
use crate::io::fs::basic_folder::home_directory;
use crate::io::fs::config::{read_config_from_file, save_config_to_file};
use crate::io::fs::env;
use crate::io::fs::media::{init_media, read_media_config, write_media_config};
use crate::io::fs::pages::{read_all_journal_files, read_all_user_files};
use crate::io::fs::root_path::{get_current_active_data_root_location, InitialConfigLocation};
use crate::io::http::design;
use crate::io::http::favourites;
use crate::io::http::markdown;
use crate::io::http::media;
use crate::io::http::metainfo;
use crate::io::http::page;
use crate::io::http::page::journalpage;
use crate::io::http::page::search;
use crate::io::http::page::userpage;
use crate::io::http::r#static;
use crate::looksyk::config::config::{Config, Design};
use crate::looksyk::index::asset::create_empty_asset_cache;
use crate::looksyk::index::media::MediaIndex;
use crate::looksyk::index::tag::create_tag_index;
use crate::looksyk::index::todo::create_todo_index;
use crate::looksyk::index::userpage::{create_journal_page_index, create_user_page_index};
use crate::state::state::{AppState, DataRootLocation};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};

mod configuration;
mod io;
mod looksyk;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let default_config = configuration::get_default_configuration();
    let config = default_config.overwrite(get_cli_args());

    let data_root_location = config.overwrite_graph_location.unwrap_or_else(|| {
        let initial_config_path = env::get_or_default(
            "LOOKSYK_CONFIG_PATH",
            home_directory()
                .join(".local")
                .join("share")
                .join("looksyk")
                .to_str()
                .unwrap(),
        );
        get_current_active_data_root_location(&InitialConfigLocation {
            path: initial_config_path,
        })
    });

    if !exists_folder(data_root_location.path.to_path_buf())
        || folder_empty(data_root_location.path.to_path_buf())
    {
        init_empty_graph(&data_root_location);
    }

    let app_state = create_app_state(data_root_location, config.application_title);

    println!(
        "Starting Looksyk on  http://{}:{}",
        APPLICATION_HOST, config.application_port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(markdown::endpoints::parse)
            .service(page::endpoints::update_block)
            .service(userpage::endpoints::get_overview_page)
            .service(journalpage::endpoints::get_journal)
            .service(journalpage::endpoints::update_journal)
            .service(userpage::endpoints::get_page)
            .service(userpage::endpoints::update_page)
            .service(userpage::endpoints::get_backlinks)
            .service(userpage::endpoints::delete_page)
            .service(userpage::endpoints::rename_page)
            .service(favourites::endpoints::insert_favourite)
            .service(favourites::endpoints::delete_favourite)
            .service(favourites::endpoints::get_favourites)
            .service(favourites::endpoints::update_favourites)
            .service(media::endpoints::upload_file)
            .service(media::endpoints::compute_asset_suggestion)
            .service(design::endpoints::get_css_theme)
            .service(metainfo::endpoints::get_metainfo)
            .service(metainfo::endpoints::get_title)
            .service(r#static::endpoints::fav)
            .service(r#static::endpoints::index_html)
            .service(r#static::endpoints::css)
            .service(r#static::endpoints::js)
            .service(r#static::endpoints::font_css)
            .service(r#static::endpoints::font_garamond)
            .service(r#static::endpoints::font_material)
            .service(r#static::endpoints::emoji)
            .service(r#static::endpoints::asset_js)
            .service(media::endpoints::assets)
            .service(media::endpoints::generate_assets_overview)
            .service(media::endpoints::get_asset_preview)
            .service(search::endpoints::search_in_files)
            .service(r#static::endpoints::catch_all_journal)
            .service(r#static::endpoints::catch_all_journals)
            .service(r#static::endpoints::catch_all_pages)
    })
    .bind(SocketAddr::new(
        IpAddr::from_str(config.application_host.as_str()).unwrap(),
        config.application_port,
    ))?
    .run()
    .await
}

fn init_empty_graph(data_root_location: &DataRootLocation) {
    create_folder(data_root_location.path.join("assets"));
    create_folder(data_root_location.path.join("config"));
    write_media_config(&data_root_location, &MediaIndex { media: vec![] });
    save_config_to_file(
        &data_root_location,
        &Config {
            favourites: vec![],
            design: Design {
                primary_color: "#0c884c".to_string(),
                background_color: "#15212D".to_string(),
                foreground_color: "white".to_string(),
                primary_shading: "rgba(255, 255, 255, 0.1)".to_string(),
            },
        },
    );

    create_folder(data_root_location.path.join("journals"));
    create_folder(data_root_location.path.join("pages"));
}

fn create_app_state(data_root_location: DataRootLocation, title: String) -> Data<AppState> {
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
    let asset_cache = create_empty_asset_cache();

    println!("all data refreshed");

    let app_state = Data::new(AppState {
        title,
        data_path: data_root_location,
        a_user_pages: Mutex::new(user_page_index.clone()),
        b_journal_pages: Mutex::new(journal_index.clone()),
        c_todo_index: Mutex::new(todo_index.clone()),
        d_tag_index: Mutex::new(tag_index.clone()),
        e_asset_cache: Mutex::new(asset_cache),
        f_media_index: Mutex::new(media_index),
        g_config: Mutex::new(config),
    });
    app_state
}
