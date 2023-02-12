extern crate core;

use std::io;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use async_session::MemoryStore;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::Extension;
use axum::{routing::get, Router};
use clap::{Parser, Subcommand};
use http::StatusCode;
use meilisearch_sdk::client::Client as MeiliClient;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::auth::oauth_client;
use crate::errors::AppResult;
use crate::settings::SETTINGS;
use crate::state::AppState;

use crate::views::{article, home, tag};

pub(crate) mod auth;
mod db;
mod errors;
mod search;
mod settings;
mod state;
mod views;

/// A Self-Hosted alternative to pocket
#[derive(Parser)]
#[command(
    version,
    name = "Manucure",
    author = "Paul D. <paul.delafosse@protonmail.com>"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    // Start the manucure server
    Serve,
    // Refresh all meili search indexes
    Reindex,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let cli = Cli::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "manucure=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let connection_url = SETTINGS.database_url();

    let db = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(3))
        .connect(&connection_url)
        .await
        .expect("can connect to database");

    sqlx::migrate!().run(&db).await?;

    let meili_client = MeiliClient::new(
        &SETTINGS.search_engine.url,
        &SETTINGS.search_engine.admin_key,
    );

    match cli.command {
        Command::Serve => serve(meili_client, db).await?,
        Command::Reindex => search::reindex_all(meili_client, db).await?,
    }

    Ok(())
}

async fn serve(meili_client: MeiliClient, db: PgPool) -> AppResult<()> {
    let store = MemoryStore::new();
    let oauth_client = oauth_client();

    let state = AppState {
        store,
        oauth_client,
        meili_client,
    };

    let opt_assets = "/opt/manucure/assets";
    let asset_path = if PathBuf::from(opt_assets).exists() {
        opt_assets
    } else {
        "assets"
    };

    let favicon_service =
        get_service(ServeFile::new("assets/favicon-32x32.png")).handle_error(handle_error);
    let assets_service = get_service(ServeDir::new(asset_path)).handle_error(handle_error);

    let router = Router::new()
        .route("/health", get(health))
        .nest("/articles", article::router())
        .nest("/tags", tag::router())
        .nest("/auth", auth::router())
        .merge(home::router())
        .nest_service("/favicon.ico", favicon_service)
        .nest_service("/assets", assets_service)
        .layer(Extension(db))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], SETTINGS.port));

    tracing::debug!("Staring Manucure");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

pub async fn health(
    State(_state): State<AppState>,
    Extension(_db): Extension<PgPool>,
) -> AppResult<()> {
    Ok(())
}
