use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};
use axum::Extension;

use crate::auth::get_connected_user;
use crate::auth::openid::Oauth2User;
use crate::errors::{ErrorTemplate, ViewResult};
use crate::state::AppState;
use manucure_db::PgPool;
use manucure_model::article::Article;

pub async fn delete_article(
    State(state): State<AppState>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> ViewResult<Redirect> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(IntoResponse::into_response)?;

    let article = manucure_db::article::delete(connected_user.id, id, &db)
        .await
        .map_err(|err| ErrorTemplate::response_from(err.into(), user))?;

    state.meili_client.delete::<Article>(article.into()).await;

    Ok(Redirect::to("/"))
}
