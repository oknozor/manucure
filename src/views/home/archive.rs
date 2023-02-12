use crate::auth::Oauth2User;
use crate::db::article::get_all_archived;
use crate::db::user::get_connected_user;
use crate::errors::{ErrorTemplate, ViewResult};
use crate::settings::SETTINGS;
use crate::views::home::HomePageTemplate;
use crate::views::{HtmlTemplate, PageQuery};

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Extension;
use sqlx::PgPool;

pub async fn archived(
    Query(page): Query<PageQuery>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> ViewResult<HtmlTemplate<HomePageTemplate>> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(IntoResponse::into_response)?;

    let articles = get_all_archived(connected_user.id, page.into(), &db)
        .await
        .map_err(|err| ErrorTemplate::response_from(err, user.clone()))?;

    let articles = articles
        .try_into()
        .map_err(|err| ErrorTemplate::response_from(err, user))?;

    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Archive",
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}
