use crate::auth::{get_connected_user, Oauth2User};
use crate::errors::AppResult;
use axum::extract::Path;
use axum::Extension;
use manucure_db::PgPool;

pub async fn star_article(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> AppResult<()> {
    let user = get_connected_user(user.as_ref(), &db).await?;
    manucure_db::article::star(user.id, id, &db).await?;

    Ok(())
}

pub async fn unstar_article(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> AppResult<()> {
    let user = get_connected_user(user.as_ref(), &db).await?;
    manucure_db::article::unstar(user.id, id, &db).await?;

    Ok(())
}