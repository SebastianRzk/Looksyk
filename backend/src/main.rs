use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use self::looksyk::data::config::init::graph::init_graph_if_needed;
use self::looksyk::data::config::startup_configuration;
use crate::io::cli::endpoints::get_cli_args;
use crate::io::fs::basic_folder::home_directory;
use crate::io::fs::env;
use crate::io::fs::env::keys::LOOKSYK_CONFIG_PATH;
use crate::io::fs::root_path::{get_current_active_data_root_location, InitialConfigLocation};
use crate::io::http;
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
use crate::io::state::convert_to_app_state;
use crate::looksyk::data::graph::load_graph_data;
use actix_web::middleware::Logger;
use actix_web::{error, web, App, HttpResponse, HttpServer};

mod io;
mod looksyk;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let default_config = startup_configuration::get_default_configuration();
    let cli_args = get_cli_args();
    println!("Provided CLI args {:?}", cli_args);
    let config = default_config.overwrite(cli_args);
    println!("Computed configuration {:?}", config);

    let data_root_location = config.overwrite_graph_location.unwrap_or_else(|| {
        let initial_config_path = env::get_or_default(
            LOOKSYK_CONFIG_PATH,
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

    init_graph_if_needed(&data_root_location);

    let app_state = convert_to_app_state(load_graph_data(data_root_location), &config.static_path);

    println!(
        "Starting Looksyk on  http://{}:{}",
        config.application_host, config.application_port
    );

    HttpServer::new(move || {
        let json_cfg = web::FormConfig::default()
            .limit(40000 * 1000 * 1000)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
            });
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .app_data(json_cfg)
            .service(markdown::endpoints::parse)
            .service(page::endpoints::update_block)
            .service(userpage::endpoints::get_overview_page)
            .service(journalpage::endpoints::get_journal)
            .service(journalpage::endpoints::update_journal)
            .service(journalpage::endpoints::journal_overview)
            .service(userpage::endpoints::get_page)
            .service(userpage::endpoints::update_page)
            .service(userpage::endpoints::get_backlinks)
            .service(userpage::endpoints::delete_page)
            .service(userpage::endpoints::rename_page)
            .service(userpage::endpoints::append_page)
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
            .service(r#static::endpoints::user_css)
            .service(media::endpoints::assets)
            .service(media::endpoints::generate_assets_overview)
            .service(media::endpoints::get_asset_preview)
            .service(media::endpoints::get_metadata)
            .service(search::endpoints::search_in_files)
            .service(http::state::endpoints::update_block)
            .default_service(web::get().to(r#static::endpoints::index))
    })
    .bind(SocketAddr::new(
        IpAddr::from_str(config.application_host).unwrap(),
        config.application_port,
    ))?
    .run()
    .await
}
