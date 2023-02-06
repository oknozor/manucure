use askama::Template;
use axum::{
    response::{Html, IntoResponse, Redirect, Response},
    Extension, Form,
};
use axum_macros::debug_handler;
use http::StatusCode;
use sqlx::PgPool;
use serde::Deserialize;
use crate::{
    errors::AppResult,
    readable_html::{get_all_articles, Article, self},
};

pub struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    pub articles: Vec<Article>,
}

pub async fn index(Extension(db): Extension<PgPool>) -> AppResult<HtmlTemplate<HomePageTemplate>> {
    let articles = get_all_articles(&db).await?;
    Ok(HtmlTemplate(HomePageTemplate { articles }))
}

#[derive(Debug, Deserialize)]
pub struct ArticleSaveForm {
    pub url: String,
}

#[debug_handler]
pub async fn save(
    Extension(db): Extension<PgPool>,
    Form(input): Form<ArticleSaveForm>,
) -> AppResult<Redirect> {
    readable_html::fetch_and_store_article(&input.url, &db).await?;
    Ok(Redirect::to("/"))
}
