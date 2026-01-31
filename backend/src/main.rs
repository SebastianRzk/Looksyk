use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use self::looksyk::data::config::init::graph::init_graph_if_needed;
use self::looksyk::data::config::startup_configuration;
use crate::io::cli::endpoints::get_cli_args;
use crate::io::fs::basic_folder::config_directory;
use crate::io::fs::env;
use crate::io::fs::env::keys::LOOKSYK_CONFIG_PATH;
use crate::io::fs::root_path::{get_current_active_data_root_location, InitialConfigLocation};
use crate::io::http;
use crate::io::http::block_properties;
use crate::io::http::config;
use crate::io::http::favourites;
use crate::io::http::help;
use crate::io::http::markdown;
use crate::io::http::media;
use crate::io::http::metainfo;
use crate::io::http::page;
use crate::io::http::page::search;
use crate::io::http::page::userpage;
use crate::io::http::page::{journalpage, templates};
use crate::io::http::r#static;
use crate::io::http::{design, kanban};
use crate::io::state::convert_to_app_state;
use crate::looksyk::data::graph::load_graph_data;
use actix_web::middleware::Logger;
mod io;

use crate::io::actix::{json_form_config, multipart_form_config};
use crate::io::cargo::get_current_application_version;
use crate::io::fs::version::load_graph_version;
use crate::migration::migration_1_14_3::migriere_1_14_3;
use crate::migration::migrator::{run_migrations, MigrationResult};
use crate::sync::git::application_port::git_sync_application_port::{
    load_git_config, try_to_commit_and_push, try_to_update_graph, CommitInitiator,
    GraphChangesToClear,
};
use crate::sync::io::sync_application_port::{GraphChange, GraphChanges, GraphChangesState};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use migration::migrator::would_migrate_something;

mod looksyk;
mod migration;
mod state;
mod sync;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let default_config = startup_configuration::get_default_configuration();
    let cli_args = get_cli_args();
    println!("Provided CLI args {cli_args:?}");
    let config = default_config.overwrite(cli_args);
    println!("Computed configuration {config:?}");

    //TODO remove again after some time
    migriere_1_14_3();

    let graph_root_location = config.overwrite_graph_location.unwrap_or_else(|| {
        let initial_config_path =
            env::get_or_default(LOOKSYK_CONFIG_PATH, config_directory().to_str().unwrap());
        get_current_active_data_root_location(&InitialConfigLocation {
            path: initial_config_path,
        })
    });

    init_graph_if_needed(&graph_root_location);

    let current_application_version = get_current_application_version();
    let graph_version = load_graph_version(&graph_root_location);
    let git_config = load_git_config(&graph_root_location);

    let pull_changes = try_to_update_graph(
        &graph_root_location,
        &git_config.config.lock().unwrap(),
        CommitInitiator::Startup,
        &GraphChanges::new(),
    );
    if pull_changes == GraphChangesToClear::Error
        && git_config
            .config
            .lock()
            .unwrap()
            .halt_on_migration_without_internet
        && would_migrate_something(&graph_version)
    {
        panic!(
            "Failed to pull changes from remote repository. Halting startup due to configuration."
        );
    }

    let migration_result = run_migrations(
        &current_application_version,
        &graph_version,
        &graph_root_location,
    );

    if migration_result == MigrationResult::MigratedSomething {
        try_to_commit_and_push(
            &graph_root_location,
            &git_config.config.lock().unwrap(),
            CommitInitiator::Migration,
            &GraphChanges::from_iter([GraphChange::graph_updated(
                current_application_version.to_string(),
            )]),
        );
    }

    let git_config = Data::new(load_git_config(&graph_root_location));

    let app_state =
        convert_to_app_state(load_graph_data(&graph_root_location), &config.static_path);

    let changes_state = Data::new(GraphChangesState::default());

    eprintln!(
        "Starting Looksyk on address http://{}:{}",
        config.application_host, config.application_port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .app_data(json_form_config())
            .app_data(multipart_form_config())
            .app_data(git_config.clone())
            .app_data(changes_state.clone())
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
            .service(design::endpoints::get_appearance)
            .service(design::endpoints::set_design_config)
            .service(metainfo::endpoints::get_metainfo)
            .service(metainfo::endpoints::get_title)
            .service(metainfo::endpoints::set_title)
            .service(metainfo::endpoints::get_graph_location)
            .service(metainfo::endpoints::get_application_version)
            .service(r#static::endpoints::fav)
            .service(r#static::endpoints::index_html)
            .service(r#static::endpoints::css)
            .service(r#static::endpoints::js)
            .service(r#static::endpoints::font_css)
            .service(r#static::endpoints::font_garamond)
            .service(r#static::endpoints::font_noto)
            .service(r#static::endpoints::font_material)
            .service(r#static::endpoints::emoji)
            .service(r#static::endpoints::asset_js)
            .service(r#static::endpoints::user_css)
            .service(media::endpoints::assets)
            .service(media::endpoints::generate_assets_overview)
            .service(media::endpoints::get_asset_preview)
            .service(media::endpoints::get_metadata)
            .service(templates::endpoints::list_all_templates)
            .service(templates::endpoints::insert_template_into_page)
            .service(search::endpoints::search_in_files)
            .service(http::state::endpoints::post_refresh_internal_state)
            .service(help::help)
            .service(kanban::endpoints::get_kanban)
            .service(kanban::endpoints::move_card)
            .service(block_properties::get_block_properties)
            .service(sync::git::io::git_controller::get_current_git_status)
            .service(sync::git::io::git_controller::update_current_data)
            .service(sync::git::io::git_controller::post_create_checkpoint)
            .service(sync::git::io::git_controller::post_create_shutdown_checkpoint)
            .service(sync::git::io::git_controller::post_retry_upload)
            .service(sync::git::io::git_controller::post_clone_existing_graph)
            .service(sync::git::io::git_controller::post_connect_to_git)
            .service(sync::git::io::git_controller::get_shutdown_status)
            .service(config::endpoints::get_journal_title_format)
            .service(config::endpoints::set_journal_title_format)
            .service(crate::io::http::plot::endpoints::example_plot_png)
            .default_service(web::get().to(r#static::endpoints::index))
    })
    .bind(SocketAddr::new(
        IpAddr::from_str(config.application_host).unwrap(),
        config.application_port,
    ))?
    .run()
    .await
}
