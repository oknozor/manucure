use crate::auth::Oauth2User;
use crate::db::article::{fetch_and_store, Article};
use crate::db::user::get_connected_user;
use crate::errors::AppResult;
use crate::views::HtmlTemplate;
use crate::{db};
use askama::Template;
use axum::extract::Path;
use axum::response::Redirect;
use axum::{Extension, Form};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct ArticleSaveForm {
    pub url: String,
}

pub async fn save(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Form(input): Form<ArticleSaveForm>,
) -> AppResult<Redirect> {
    let user = get_connected_user(user, &db).await?;
    fetch_and_store(user.id, &input.url, &db).await?;
    Ok(Redirect::to("/"))
}

#[derive(Template, Debug)]
#[template(path = "article.html")]
pub struct ArticleTemplate {
    article: Article,
}

pub async fn get_article(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> AppResult<HtmlTemplate<ArticleTemplate>> {
    let user = get_connected_user(user, &db).await?;
    let article = db::article::get(user.id, id, &db).await?;
    let template = ArticleTemplate { article };

    Ok(HtmlTemplate(template))
}

pub async fn delete_article(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> AppResult<Redirect> {
    let user = get_connected_user(user, &db).await?;
    db::article::delete(user.id, id, &db).await?;

    Ok(Redirect::to("/"))
}

pub async fn star_article(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> AppResult<()> {
    let user = get_connected_user(user, &db).await?;
    db::article::star(user.id, id, &db).await?;

    Ok(())
}

pub async fn unstar_article(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> AppResult<()> {
    let user = get_connected_user(user, &db).await?;
    db::article::unstar(user.id, id, &db).await?;

    Ok(())
}

pub async fn archive_article(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> AppResult<()> {
    let user = get_connected_user(user, &db).await?;
    db::article::archive(user.id, id, &db).await?;

    Ok(())
}
