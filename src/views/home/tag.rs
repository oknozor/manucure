use crate::auth::Oauth2User;

use crate::db::tag;
use crate::db::tag::Tag;
use crate::db::user::get_connected_user;
use crate::errors::{ErrorTemplate, ViewResult};
use crate::settings::SETTINGS;
use crate::views::HtmlTemplate;
use askama::Template;

use axum::response::IntoResponse;
use axum::Extension;

use sqlx::PgPool;

#[derive(Template, Debug)]
#[template(path = "tags.html")]
pub struct TagPageTemplate {
    tags: Vec<Tag>,
    user_index: String,
    meili_url: &'static str,
    meili_secret: &'static str,
}

pub async fn tags(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> ViewResult<HtmlTemplate<TagPageTemplate>> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(IntoResponse::into_response)?;

    let tags = tag::all_for_user(connected_user.id, &db)
        .await
        .map_err(|err| ErrorTemplate::response_from(err, user))?;

    Ok(HtmlTemplate(TagPageTemplate {
        tags,
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}
