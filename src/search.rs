use crate::db;
use crate::errors::AppResult;
use meilisearch_sdk::client::Client;
use sqlx::PgPool;
use tracing::info;

pub async fn reindex_all(client: Client, db: PgPool) -> AppResult<()> {
    delete_indexes(&client, &db).await?;
    reindex_articles(&client, &db).await?;
    reindex_tags(&client, &db).await?;
    Ok(())
}

async fn delete_indexes(client: &Client, db: &PgPool) -> AppResult<()> {
    let users = db::user::all(db).await?;
    for user in users {
        info!("cleaning indexes for user {}", user.id);
        client.delete_index(format!("articles-{}", user.id)).await?;
        client.delete_index(format!("tags-{}", user.id)).await?;
    }

    Ok(())
}

async fn reindex_articles(client: &Client, db: &PgPool) -> AppResult<()> {
    info!("enqueuing indexation tasks for articles");
    let articles = db::article::all(db).await?;
    for article in articles {
        client
            .index(format!("articles-{}", article.user_id))
            .add_documents(&[article], Some("id"))
            .await?;
    }

    Ok(())
}

async fn reindex_tags(client: &Client, db: &PgPool) -> AppResult<()> {
    info!("enqueuing indexation tasks for tags");
    let tags = db::tag::all(db).await?;
    for tag in tags {
        client
            .index(format!("tags-{}", tag.user_id))
            .add_documents(&[tag], Some("id"))
            .await?;
    }

    Ok(())
}
