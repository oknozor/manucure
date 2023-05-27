use crate::error::DbResult;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(sqlx::FromRow, sqlx::Type, Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
}

pub async fn all(db: &PgPool) -> DbResult<Vec<Tag>> {
    // language=PostgreSQL
    let tags = sqlx::query_as!(Tag, "SELECT * FROM tag")
        .fetch_all(db)
        .await?;

    Ok(tags)
}

pub async fn all_for_user(user_id: i64, db: &PgPool) -> DbResult<Vec<Tag>> {
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

pub async fn insert_all(user_id: i64, tags: Vec<String>, db: &PgPool) -> DbResult<Vec<Tag>> {
    // FIXME: This function uses `SELECT * FROM UNNEST($2::text[])`
    //  because sqlx does not support multiple insert yet
    //  see https://github.com/launchbadge/sqlx/issues/294

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
    .fetch_all(db)
    .await?;

    Ok(new_tags)
}

pub async fn delete(tag_id: i64, db: &PgPool) -> DbResult<Tag> {
    let tag = sqlx::query_as!(
        Tag,
        // language=PostgreSQL
        "DELETE FROM tag WHERE id = $1 RETURNING *",
        tag_id
    )
    .fetch_one(db)
    .await?;

    Ok(tag)
}

pub async fn insert_for_article(article_id: i64, tag_id: i64, db: &PgPool) -> DbResult<()> {
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

pub async fn delete_article_tag(article_id: i64, tag_id: i64, db: &PgPool) -> DbResult<()> {
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
