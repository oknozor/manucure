use crate::error::{DbError, DbResult};
use crate::tag::Tag;
use crate::Page;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

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

pub async fn all(db: &PgPool) -> DbResult<Vec<Article>> {
    // language=PostgreSQL
    sqlx::query_as!(Article, "SELECT * FROM article")
        .fetch_all(db)
        .await
        .map_err(DbError::from)
}

pub async fn get(user_id: i64, id: i64, db: &PgPool) -> DbResult<ArticleWithTag> {
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

pub async fn delete(user_id: i64, id: i64, db: &PgPool) -> DbResult<Article> {
    let article = sqlx::query_as!(
        Article,
        // language=PostgreSQL
        r#"DELETE FROM article WHERE id = $1 AND user_id = $2 RETURNING *"#,
        id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(article)
}

pub async fn archive(user_id: i64, id: i64, db: &PgPool) -> DbResult<()> {
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

pub async fn unarchive(user_id: i64, id: i64, db: &PgPool) -> DbResult<()> {
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

pub async fn star(user_id: i64, id: i64, db: &PgPool) -> DbResult<()> {
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

pub async fn unstar(user_id: i64, id: i64, db: &PgPool) -> DbResult<()> {
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

pub trait AsArticle {
    fn title(&self) -> &str;
    fn text(&self) -> &str;
    fn content(&self) -> &str;
}

pub async fn fetch_and_store(
    user_id: i64,
    url: &str,
    raw_html_article: impl AsArticle,
    db: &PgPool,
) -> DbResult<Article> {
    let url = url.to_owned();

    let article = sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "INSERT INTO article (url, user_id, text, content, title) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        url,
        user_id,
        raw_html_article.text(),
        raw_html_article.content(),
        raw_html_article.title()
    )
        .fetch_one(db)
        .await?;

    Ok(article)
}

pub async fn get_all_active(
    user_id: i64,
    page: Page,
    db: &PgPool,
) -> DbResult<ArticlesWithPagination> {
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
) -> DbResult<ArticlesWithPagination> {
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
) -> DbResult<ArticlesWithPagination> {
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
) -> DbResult<ArticlesWithPagination> {
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
