use readability::extractor::scrape;
use sqlx::PgPool;

#[derive(sqlx::FromRow, sqlx::Type, Debug)]
pub struct Article {
    pub id: i64,
    pub user_id: i64,
    pub url: String,
    pub title: String,
    pub text: String,
    pub starred: bool,
    pub archived: bool,
    pub created: chrono::NaiveDateTime,
    pub content: String,
}

#[derive(sqlx::FromRow, sqlx::Type, Debug)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Debug)]
pub struct ArticleWithTag {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub text: String,
    pub content: String,
    pub tags: Vec<Tag>,
}

pub(crate) async fn get(user_id: i64, id: i64, db: &PgPool) -> anyhow::Result<Article> {
    let article = sqlx::query_as!(
        Article,
        // language=PostgreSQL
        r#"SELECT id, user_id, url, title, text, starred, archived, created, content
        FROM article WHERE id = $1 AND user_id = $2"#,
        id,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(article)
}

pub(crate) async fn delete(user_id: i64, id: i64, db: &PgPool) -> anyhow::Result<()> {
    sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "DELETE FROM article WHERE id = $1 AND user_id = $2",
        id,
        user_id
    )
    .execute(db)
    .await?;

    Ok(())
}

pub(crate) async fn archive(user_id: i64, id: i64, db: &PgPool) -> anyhow::Result<()> {
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

pub(crate) async fn star(user_id: i64, id: i64, db: &PgPool) -> anyhow::Result<()> {
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

pub(crate) async fn unstar(user_id: i64, id: i64, db: &PgPool) -> anyhow::Result<()> {
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

pub async fn fetch_and_store(user_id: i64, url: &str, db: &PgPool) -> anyhow::Result<()> {
    let url = url.to_owned();
    let article = scrape(&url).await?;
    sqlx::query!(
        // language=PostgreSQL
        "INSERT INTO article (url, user_id, text, content, title) VALUES ($1, $2, $3, $4, $5)",
        url,
        user_id,
        article.text,
        article.content,
        article.title
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn get_all_active(user_id: i64, db: &PgPool) -> anyhow::Result<Vec<Article>> {
    let articles = sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "SELECT * FROM article WHERE user_id = $1 AND NOT archived",
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(articles)
}

pub async fn get_all_archived(user_id: i64, db: &PgPool) -> anyhow::Result<Vec<Article>> {
    let articles = sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "SELECT * FROM article WHERE user_id = $1 AND archived",
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(articles)
}

pub async fn get_all_starred(user_id: i64, db: &PgPool) -> anyhow::Result<Vec<Article>> {
    let articles = sqlx::query_as!(
        Article,
        // language=PostgreSQL
        "SELECT * FROM article WHERE user_id = $1 AND starred",
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(articles)
}
