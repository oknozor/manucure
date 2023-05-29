use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum::{Extension, Form};
use manucure_db::article::fetch_and_store;

use crate::auth::get_connected_user;
use crate::auth::openid::Oauth2User;
use crate::errors::{AppError, ErrorTemplate, ViewResult};
use crate::state::AppState;
use manucure_db::PgPool;
use manucure_model::article::Article;
use manucure_readability::HtmlArticle;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ArticleSaveForm {
    pub url: String,
}

pub async fn save(
    State(state): State<AppState>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Form(input): Form<ArticleSaveForm>,
) -> ViewResult<Redirect> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(AppError::from)
        .map_err(IntoResponse::into_response)?;

    let article = HtmlArticle::scrape(&input.url)
        .await
        .map_err(|err| ErrorTemplate::response_from(err.into(), user.clone()))?;

    let article = fetch_and_store(connected_user.id, &input.url, article, &db)
        .await
        .map_err(|err| ErrorTemplate::response_from(err.into(), user))?;

    state
        .meili_client
        .index_one::<Article>(article.into())
        .await;
    Ok(Redirect::to("/"))
}
