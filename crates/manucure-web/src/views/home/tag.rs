use axum::response::IntoResponse;
use axum::Extension;

use crate::auth::{get_connected_user, Oauth2User};
use crate::errors::{ErrorTemplate, ViewResult};
use crate::views::HtmlTemplate;
use askama::Template;
use manucure_db::tag::Tag;
use manucure_db::{tag, PgPool};
use manucure_settings::SETTINGS;

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
        .map_err(|err| ErrorTemplate::response_from(err.into(), user))?;

    Ok(HtmlTemplate(TagPageTemplate {
        tags,
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}
