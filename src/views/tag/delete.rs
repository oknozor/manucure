use crate::auth::Oauth2User;
use crate::db;
use crate::db::user::get_connected_user;
use crate::errors::AppResult;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::Extension;
use sqlx::PgPool;

pub async fn delete(
    State(state): State<AppState>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(tag_id): Path<i64>,
) -> AppResult<()> {
    let connected_user = get_connected_user(user.as_ref(), &db).await?;

    db::tag::delete(connected_user.id, tag_id, state.meili_client, &db).await?;

    Ok(())
}
