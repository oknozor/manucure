use crate::auth::Oauth2User;

use crate::db::article::fetch_and_store;
use crate::db::user::get_connected_user;
use crate::errors::{ErrorTemplate, ViewResult};

use crate::state::AppState;

use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum::{Extension, Form};
use serde::Deserialize;
use sqlx::PgPool;

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
        .map_err(IntoResponse::into_response)?;

    fetch_and_store(connected_user.id, &input.url, state.meili_client, &db)
        .await
        .map_err(|err| ErrorTemplate::response_from(err, user))?;

    Ok(Redirect::to("/"))
}
