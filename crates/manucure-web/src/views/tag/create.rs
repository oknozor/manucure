use crate::auth::{get_connected_user, Oauth2User};
use crate::errors::AppResult;
use crate::state::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use manucure_db::tag::Tag;
use manucure_db::PgPool;
use serde::Deserialize;

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
    let user_id = connected_user.id;

    let tags = manucure_db::tag::insert_all(user_id, tags.tags, &db).await?;

    state
        .meili_client
        .index_many(
            tags.iter()
                .cloned()
                .map(manucure_model::tag::Tag::from)
                .collect(),
        )
        .await;

    Ok(Json(tags))
}
