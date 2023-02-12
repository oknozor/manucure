use crate::db::tag::Tag;
use crate::db::Page;
use crate::errors::AppResult;
use chrono::{DateTime, Utc};
use meilisearch_sdk::client::Client as MeiliClient;
use readability::extractor::scrape;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, error};

#[derive(Debug, Serialize)]
pub struct ArticlesWithPagination {
    pub items: Vec<ArticleDigest>,
    pub pages: Vec<i64>,
    pub current_page: i64,
}

#[derive(sqlx::FromRow, sqlx::Type, Serialize, Deserialize, Debug)]
pub struct Article {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
    pub title: String,
    pub text: String,
    pub starred: bool,
    pub archived: bool,
    pub created: DateTime<Utc>,
    pub content: String,
}

#[derive(sqlx::FromRow, sqlx::Type, Serialize, Deserialize, Debug)]
pub struct ArticleDigest {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub text: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ArticleWithTag {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
    pub title: String,
    pub text: String,
    pub starred: bool,
    pub archived: bool,
    pub created: DateTime<Utc>,
    pub content: String,
    pub tags: Vec<Tag>,
}

pub(crate) async fn all(db: &PgPool) -> AppResult<Vec<Article>> {
    // language=PostgreSQL
    let articles = sqlx::query_as!(Article, "SELECT * FROM article")
        .fetch_all(db)
        .await?;

    Ok(articles)
}

pub(crate) async fn get(user_id: i64, id: i64, db: &PgPool) -> AppResult<ArticleWithTag> {
    let mut transaction = db.begin().await?;

    let article = sqlx::query_as!(
        Article,
        // language=PostgreSQL
        r#"SELECT id, user_id, url, title, text, starred, archived, created, content
        FROM article WHERE id = $1 AND user_id = $2"#,
        id,
        user_id
    )
    .fetch_one(&mut transaction)
    .await?;

    let tags = sqlx::query_as!(
        Tag,
        // language=PostgreSQL
        r#"SELECT id, user_id, name FROM tag JOIN article_tag on article_tag.tag_id = tag.id WHERE article_id = $1"#,
        id,
    )
        .fetch_all(&mut transaction)
        .await?;

    transaction.commit().await?;

    let article = ArticleWithTag {
        id: article.id,
        user_id: article.user_id,
        url: article.url,
        title: article.title,
        text: article.text,
        starred: article.starred,
        archived: article.archived,
        created: article.created,
        content: article.content,
        tags,
    };

    Ok(article)
}

pub(crate) async fn delete(
    user_id: i64,
    id: i64,
    meili_client: MeiliClient,
    db: &PgPool,
) -> AppResult<()> {
    let mut transaction = db.begin().await?;

    sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "DELETE FROM article WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(&mut transaction)
    .await?;

    let user_index = format!("articles-{user_id}");
    tokio::spawn(async move {
        match meili_client.index(user_index).delete_document(id).await {
            Ok(task) => debug!("Article indexed: {task:?}"),
            Err(err) => error!("Indexation failed for article {id}: {err}"),
        };
    });

    transaction.commit().await?;

    Ok(())
}

pub(crate) async fn archive(user_id: i64, id: i64, db: &PgPool) -> AppResult<()> {
    sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "UPDATE article SET archived = true WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn unarchive(user_id: i64, id: i64, db: &PgPool) -> AppResult<()> {
    sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "UPDATE article SET archived = false WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn star(user_id: i64, id: i64, db: &PgPool) -> AppResult<()> {
    sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "UPDATE article SET starred = true WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn unstar(user_id: i64, id: i64, db: &PgPool) -> AppResult<()> {
    sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "UPDATE article SET starred = false WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn fetch_and_store(
    user_id: i64,
    url: &str,
    meili_client: MeiliClient,
    db: &PgPool,
) -> AppResult<()> {
    let url = url.to_owned();
    let article = scrape(&url).await?;
    let mut transaction = db.begin().await?;

    let article = sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "INSERT INTO article (url, user_id, text, content, title) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        url,
        user_id,
        article.text,
        article.content,
        article.title
    )
        .fetch_one(&mut transaction)
        .await?;

    tokio::spawn(async move {
        let id = article.id;
        let user_index = format!("articles-{user_id}");
        match meili_client
            .index(&user_index)
            .add_documents(&[article], Some("id"))
            .await
        {
            Ok(task) => debug!("Article indexed: {task:?}"),
            Err(err) => error!("Indexation failed for article {id}: {err}"),
        };
    });

    transaction.commit().await?;

    Ok(())
}

pub async fn get_all_active(
    user_id: i64,
    page: Page,
    db: &PgPool,
) -> AppResult<ArticlesWithPagination> {
    let (limit, offset) = page.to_limit_offset();

    let articles = sqlx::query_as!(
        ArticleDigest,
        // language=PostgreSQL
        r#"SELECT id, url, title, text, created FROM article
         WHERE user_id = $1 AND NOT archived ORDER BY created DESC
         LIMIT $2 OFFSET $3
         "#,
        user_id,
        limit,
        offset
    )
    .fetch_all(db)
    .await?;

    let article_count = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(*) from article WHERE user_id = $1 AND NOT archived"#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(ArticlesWithPagination {
        items: articles,
        pages: page.get_pagination(article_count),
        current_page: page.nth(),
    })
}

pub async fn get_all_archived(
    user_id: i64,
    page: Page,
    db: &PgPool,
) -> AppResult<ArticlesWithPagination> {
    let (limit, offset) = page.to_limit_offset();

    let articles = sqlx::query_as!(
        ArticleDigest,
        // language=PostgreSQL
        "SELECT id, url, title, text, created FROM article WHERE user_id = $1 AND archived ORDER BY created DESC LIMIT $2 OFFSET $3",
        user_id,
        limit,
        offset
    )
    .fetch_all(db)
    .await?;

    let archive_count = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(*) from article WHERE user_id = $1 AND archived"#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(ArticlesWithPagination {
        items: articles,
        pages: page.get_pagination(archive_count),
        current_page: page.nth(),
    })
}

pub async fn get_all_starred(
    user_id: i64,
    page: Page,
    db: &PgPool,
) -> AppResult<ArticlesWithPagination> {
    let (limit, offset) = page.to_limit_offset();

    let articles = sqlx::query_as!(
        ArticleDigest,
        // language=PostgreSQL
        "SELECT id, url, title, text, created FROM article WHERE user_id = $1 AND starred ORDER BY created DESC LIMIT $2 OFFSET $3",
        user_id,
        limit,
        offset
    )
    .fetch_all(db)
    .await?;

    let stared_count = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT count(*) from article WHERE user_id = $1 AND starred"#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(ArticlesWithPagination {
        items: articles,
        pages: page.get_pagination(stared_count),
        current_page: page.nth(),
    })
}

pub async fn get_all_having_tag(
    user_id: i64,
    tag_ids: Vec<i64>,
    page: Page,
    db: &PgPool,
) -> AppResult<ArticlesWithPagination> {
    let (limit, offset) = page.to_limit_offset();

    let articles = sqlx::query_as!(
        ArticleDigest,
        // language=PostgreSQL
        r#"SELECT DISTINCT id, url, title, text, created
            FROM article JOIN article_tag ON article.id = article_tag.article_id
            WHERE user_id = $1 AND tag_id IN (SELECT * FROM UNNEST($2::int8[])) ORDER BY created DESC
            LIMIT $3 OFFSET $4"#,
        user_id,
        tag_ids.as_slice(),
        limit,
        offset,
    )
    .fetch_all(db)
    .await?;

    let count = sqlx::query_scalar!(
        // language=PostgreSQL
        r#"SELECT DISTINCT count(*) FROM article JOIN article_tag ON article.id = article_tag.article_id
            WHERE user_id = $1 AND tag_id IN (SELECT * FROM UNNEST($2::int8[]))"#,
        user_id,
        tag_ids.as_slice()
    )
        .fetch_one(db)
        .await?;

    Ok(ArticlesWithPagination {
        items: articles,
        pages: page.get_pagination(count),
        current_page: page.nth(),
    })
}
