use crate::errors::AppResult;
use meilisearch_sdk::Client as MeiliClient;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error};

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
}

pub(crate) async fn all(db: &PgPool) -> AppResult<Vec<Tag>> {
    // language=PostgreSQL
    let tags = sqlx::query_as!(Tag, "SELECT * FROM tag")
        .fetch_all(db)
        .await?;

    Ok(tags)
}

pub async fn all_for_user(user_id: i64, db: &PgPool) -> AppResult<Vec<Tag>> {
    let tags = sqlx::query_as!(
        Tag,
        // language=PostgreSQL
        "SELECT * FROM tag WHERE user_id = $1",
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(tags)
}

pub async fn insert_all(
    user_id: i64,
    tags: Vec<String>,
    meili_client: MeiliClient,
    db: &PgPool,
) -> AppResult<Vec<Tag>> {
    // FIXME: This function uses `SELECT * FROM UNNEST($2::text[])`
    //  because sqlx does not support multiple insert yet
    //  see https://github.com/launchbadge/sqlx/issues/294

    let mut transaction = db.begin().await?;

    // Insert all tags and return their ids
    let user_ids: Vec<i64> = tags.iter().map(|_| user_id).collect();

    let new_tags = sqlx::query_as!(
        Tag,
        // language=PostgreSQL
        "INSERT INTO tag (user_id, name)
                SELECT * FROM UNNEST($1::int8[], $2::text[])
                ON CONFLICT DO NOTHING RETURNING *",
        user_ids.as_slice(),
        tags.as_slice(),
    )
    .fetch_all(&mut transaction)
    .await?;

    let tags_copy = new_tags.clone();
    tokio::spawn(async move {
        let user_index = format!("tags-{user_id}");
        match meili_client
            .index(&user_index)
            .add_documents(tags_copy.as_slice(), Some("id"))
            .await
        {
            Ok(task) => debug!("Article indexed: {task:?}"),
            Err(err) => error!("Indexation failed for tags: {err}"),
        };
    });

    transaction.commit().await?;

    Ok(new_tags)
}

pub async fn delete(
    user_id: i64,
    tag_id: i64,
    meili_client: MeiliClient,
    db: &PgPool,
) -> AppResult<()> {
    let mut transaction = db.begin().await?;

    sqlx::query!(
        // language=PostgreSQL
        "DELETE FROM tag WHERE id = $1",
        tag_id
    )
    .execute(&mut transaction)
    .await?;

    let user_index = format!("tags-{user_id}");
    tokio::spawn(async move {
        match meili_client
            .index(&user_index)
            .delete_document(tag_id)
            .await
        {
            Ok(task) => debug!("Tag indexed: {task:?}"),
            Err(err) => {
                error!("Deleting tag from index {user_index} failed for tag {tag_id}: {err}")
            }
        };
    });

    transaction.commit().await?;

    Ok(())
}

pub async fn insert_for_article(article_id: i64, tag_id: i64, db: &PgPool) -> AppResult<()> {
    sqlx::query!(
        // language=PostgreSQL
        "INSERT INTO article_tag (tag_id, article_id) VALUES ($1, $2)",
        tag_id,
        article_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete_article_tag(article_id: i64, tag_id: i64, db: &PgPool) -> AppResult<()> {
    sqlx::query!(
        // language=PostgreSQL
        "DELETE FROM article_tag WHERE article_id = $1 AND tag_id = $2",
        article_id,
        tag_id
    )
    .execute(db)
    .await?;

    Ok(())
}
