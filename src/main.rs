use std::net::SocketAddr;
use std::time::Duration;

use axum::Extension;
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod errors;
mod readable_html;
mod views;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "manucure=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO: setup with config crate or env vars
    let connection_url = "postgres://postgres:postgres@localhost/manucure";

    tracing::debug!("Connecting to {connection_url}");
    let db = PgPoolOptions::new()
        .max_connections(10)
        .idle_timeout(Duration::from_secs(3))
        .connect(connection_url)
        .await
        .expect("can connect to database");

    sqlx::migrate!().run(&db).await?;

    let router = Router::new()
        .route("/article/save", get(views::save))
        .route("/", get(views::index))
        .layer(Extension(db));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
