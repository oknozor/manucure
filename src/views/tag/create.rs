use crate::auth::Oauth2User;

use crate::db::user::get_connected_user;
use crate::errors::AppResult;

use crate::state::AppState;

use crate::db;
use crate::db::tag::Tag;
use axum::extract::State;
use axum::{Extension, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct CreateTagsCommand {
    pub tags: Vec<String>,
}

pub async fn save_many(
    State(state): State<AppState>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Json(tags): Json<CreateTagsCommand>,
) -> AppResult<Json<Vec<Tag>>> {
    let connected_user = get_connected_user(user.as_ref(), &db).await?;

    let tags = db::tag::insert_all(connected_user.id, tags.tags, state.meili_client, &db).await?;

    Ok(Json(tags))
}
