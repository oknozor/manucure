use crate::errors::AppError;
use crate::state::AppState;
use askama::Template;
use axum::routing::get;
use axum::Router;
use estimated_read_time::Options;
use manucure_db::article::{ArticleDigest, ArticlesWithPagination};
use serde::Serialize;
use url::Url;

mod archive;
mod index;
mod star;
mod tag;

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    articles: ArticlesWithPaginationDto,
    title: &'static str,
    user_index: String,
    meili_url: &'static str,
    meili_secret: &'static str,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index::articles))
        .route("/tag", get(tag::tags))
        .route("/archive", get(archive::archived))
        .route("/favorites", get(star::starred))
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticleDigestDto {
    pub id: i64,
    pub url: String,
    pub domain: String,
    pub title: String,
    pub read_time: String,
    pub date: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesWithPaginationDto {
    pub items: Vec<ArticleDigestDto>,
    pub pages: Vec<i64>,
    pub current_page: i64,
}

impl TryFrom<ArticlesWithPagination> for ArticlesWithPaginationDto {
    type Error = AppError;

    fn try_from(value: ArticlesWithPagination) -> Result<Self, Self::Error> {
        let mut items = vec![];
        for article in value.items {
            let dto = ArticleDigestDto::try_from(article)?;
            items.push(dto);
        }

        Ok(Self {
            items,
            pages: value.pages,
            current_page: value.current_page,
        })
    }
}

impl TryFrom<ArticleDigest> for ArticleDigestDto {
    type Error = AppError;

    fn try_from(article: ArticleDigest) -> Result<Self, Self::Error> {
        let read_time = estimated_read_time::text(&article.text, &Options::default());
        let minutes = read_time.seconds() / 60;

        let read_time = if minutes == 0 {
            "Less than a minute".to_string()
        } else {
            format!("{minutes} min")
        };

        let date = article.created.format("%v");
        let date = date.to_string();

        Ok(ArticleDigestDto {
            id: article.id,
            url: article.url.clone(),
            domain: Url::parse(&article.url)?
                .domain()
                .expect("Article url")
                .to_string(),
            title: article.title,
            read_time,
            date,
        })
    }
}
