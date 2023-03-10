use crate::auth::Oauth2User;
use crate::db;
use crate::db::article::ArticleWithTag;
use crate::db::user::get_connected_user;
use crate::errors::{ErrorTemplate, ViewResult};
use crate::settings::SETTINGS;

use crate::views::HtmlTemplate;
use askama::Template;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Extension;

use sqlx::PgPool;

#[derive(Template, Debug)]
#[template(path = "article.html")]
pub struct ArticleTemplate {
    article: ArticleWithTag,
    user_index: String,
    tag_index: Option<String>,
    meili_url: &'static str,
    meili_secret: &'static str,
}

pub async fn article(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
    Path(id): Path<i64>,
) -> ViewResult<HtmlTemplate<ArticleTemplate>> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(IntoResponse::into_response)?;

    let article = db::article::get(connected_user.id, id, &db)
        .await
        .map_err(|err| ErrorTemplate::response_from(err, user))?;

    Ok(HtmlTemplate(ArticleTemplate {
        article,
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
        tag_index: Some(format!("tags-{}", connected_user.id)),
    }))
}
