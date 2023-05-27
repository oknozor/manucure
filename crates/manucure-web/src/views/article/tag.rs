use axum::extract::{Path, State};
use axum::{Extension, Json};
use manucure_db::tag;

use crate::auth::{get_connected_user, Oauth2User};
use crate::errors::{AppError, AppResult};
use crate::state::AppState;
use crate::views::home::ArticlesWithPaginationDto;
use crate::views::PageQuery;
use manucure_db::PgPool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilterTagsCommand {
    pub tags: Vec<i64>,
    pub page: i64,
}

pub async fn articles_with_tags(
    State(_): State<AppState>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Json(tags): Json<FilterTagsCommand>,
) -> AppResult<Json<ArticlesWithPaginationDto>> {
    let user = get_connected_user(user.as_ref(), &db).await?;
    let page = PageQuery::from(tags.page).into();
    let articles = manucure_db::article::get_all_having_tag(user.id, tags.tags, page, &db).await?;

    articles.try_into().map(Json)
}

pub async fn add_tag(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path((article_id, tag_id)): Path<(i64, i64)>,
) -> AppResult<()> {
    let _ = get_connected_user(user.as_ref(), &db).await?;
    tag::insert_for_article(article_id, tag_id, &db)
        .await
        .map_err(AppError::from)
}

pub async fn delete_tag(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path((article_id, tag_id)): Path<(i64, i64)>,
) -> AppResult<()> {
    get_connected_user(user.as_ref(), &db).await?;

    tag::delete_article_tag(article_id, tag_id, &db).await?;

    Ok(())
}
