use std::time::Duration;

use clap::{Parser, Subcommand};
use manucure_db::PgPoolOptions;
use manucure_index::MeiliClient;
use manucure_settings::SETTINGS;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod search;

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
async fn main() -> anyhow::Result<()> {
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

    manucure_db::migrate(&db).await?;

    let meili_client = MeiliClient::new(
        SETTINGS.search_engine.url.to_owned(),
        &SETTINGS.search_engine.admin_key,
    );

    match cli.command {
        // Fixme
        Command::Serve => manucure_web::serve(meili_client, db)
            .await
            .expect("Fix me this error everywhere"),
        Command::Reindex => search::reindex_all(&meili_client, db)
            .await
            .expect("Fix me this error everywhere"),
    }

    Ok(())
}
