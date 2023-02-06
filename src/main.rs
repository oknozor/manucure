use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use async_session::MemoryStore;
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::Extension;
use axum::{routing::{get, post}, Router};
use http::StatusCode;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use views::{article, home};

use crate::auth::oauth_client;
use crate::settings::SETTINGS;
use crate::state::AppState;

pub(crate) mod auth;
mod db;
mod errors;
mod settings;
mod state;
mod views;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "manucure=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let connection_url = SETTINGS.database_url();

    tracing::debug!("Connecting to {connection_url}");
    let db = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(3))
        .connect(&connection_url)
        .await
        .expect("can connect to database");

    sqlx::migrate!().run(&db).await?;

    let store = MemoryStore::new();
    let oauth_client = oauth_client();

    let state = AppState {
        store,
        oauth_client,
    };

    let router = Router::new()
        .route("/articles/save", get(article::save))
        .route("/articles/:id", get(article::get_article))
        .route("/articles/:id/delete", get(article::delete_article))
        .route("/articles/:id/star", post(article::star_article))
        .route("/articles/:id/unstar", post(article::unstar_article))
        .route("/articles/:id/archive", post(article::archive_article))
        .route("/auth/manucure/", get(auth::openid_auth))
        .route("/auth/manucure", get(auth::openid_auth))
        .route("/auth/authorized/", get(auth::login_authorized))
        .route("/auth/authorized", get(auth::login_authorized))
        .route("/logout/", get(auth::logout))
        .route("/", get(home::index))
        .nest_service(
            "/assets",
            get_service(ServeDir::new("assets")).handle_error(handle_error),
        )
        .layer(Extension(db))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], SETTINGS.port));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
