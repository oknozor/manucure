use readability::extractor::{extract, Product};
use reqwest::Url;
use sqlx::PgPool;

#[derive(sqlx::FromRow, sqlx::Type, Debug)]
pub struct Article {
    pub id: i64,
    pub url: String,
    pub title: String,
    pub text: String,
    pub content: String,
}

async fn get_article(id: i64, db: &PgPool) -> anyhow::Result<Article> {
    let article = sqlx::query_as!(
        Article,
        "SELECT id, url, title, text, content FROM article WHERE id = $1",
        id
    )
    .fetch_one(db)
    .await?;

    Ok(article)
}

pub async fn fetch_and_store_article(url: &str, db: &PgPool) -> anyhow::Result<()> {
    let url = url.to_owned();
    let article = scrape(&url).await?;
    sqlx::query!(
        "INSERT INTO article (url, text, content, title) VALUES ($1, $2, $3, $4)",
        url,
        article.text,
        article.content,
        article.title
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn scrape(url: &str) -> anyhow::Result<Product> {
    let response = reqwest::get(url).await?;
    let url = Url::parse(url)?;
    let read = response.text().await?;
    extract(&mut read.as_bytes(), &url).map_err(Into::into)
}

pub async fn get_all_articles(db: &PgPool) -> anyhow::Result<Vec<Article>> {
    let articles = sqlx::query_as!(Article, "SELECT * FROM article")
        .fetch_all(db)
        .await?;

    Ok(articles)
}
