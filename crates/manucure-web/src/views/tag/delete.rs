use crate::auth::get_connected_user;
use crate::auth::openid::Oauth2User;
use crate::errors::AppResult;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::Extension;
use manucure_db::PgPool;
use manucure_model::tag::Tag;

pub async fn delete(
    State(state): State<AppState>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(tag_id): Path<i64>,
) -> AppResult<()> {
    let _connected_user = get_connected_user(user.as_ref(), &db).await?;

    let tag = manucure_db::tag::delete(tag_id, &db).await?;
    state.meili_client.delete::<Tag>(tag.into()).await;

    Ok(())
}
