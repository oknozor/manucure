use crate::auth::openid::oauth_client;
use crate::errors::AppResult;
use crate::state::AppState;
use crate::views::{article, home, tag};
use async_session::MemoryStore;
use axum::extract::State;
use axum::handler::HandlerWithoutStateExt;
use axum::routing::get;
use axum::{Extension, Router};
use http::StatusCode;
use manucure_db::PgPool;
use manucure_index::MeiliClient;
use manucure_settings::SETTINGS;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod errors;
pub mod state;
pub mod views;

pub async fn serve(meili_client: MeiliClient, db: PgPool) -> AppResult<()> {
    let store = MemoryStore::new();
    let oauth_client = if SETTINGS.openid.is_some() {
        Some(oauth_client().await)
    } else {
        None
    };

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

    let assets_service = ServeDir::new(asset_path).not_found_service(handle_error.into_service());

    let router = Router::new()
        .route("/health", get(health))
        .nest("/articles", article::router())
        .nest("/tags", tag::router())
        .nest("/auth", auth::router())
        .merge(home::router())
        .nest_service("/assets", assets_service)
        .layer(Extension(db))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], SETTINGS.port));
    tracing::debug!("Staring Manucure on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn handle_error() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}

pub async fn health(
    State(_state): State<AppState>,
    Extension(_db): Extension<PgPool>,
) -> AppResult<()> {
    Ok(())
}
