use crate::auth::Oauth2User;
use crate::db;
use crate::db::user::get_connected_user;
use crate::errors::{ErrorTemplate, ViewResult};
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};
use axum::Extension;

use sqlx::PgPool;

pub async fn delete_article(
    State(state): State<AppState>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> ViewResult<Redirect> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(IntoResponse::into_response)?;

    db::article::delete(connected_user.id, id, state.meili_client, &db)
        .await
        .map_err(|err| ErrorTemplate::response_from(err, user))?;

    Ok(Redirect::to("/"))
}
