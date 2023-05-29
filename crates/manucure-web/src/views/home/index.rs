use axum::extract::Query;

use crate::auth::get_connected_user;
use crate::auth::openid::Oauth2User;
use crate::errors::{ErrorTemplate, ViewResult};
use crate::views::home::HomePageTemplate;
use crate::views::{HtmlTemplate, PageQuery};
use axum::response::IntoResponse;
use axum::Extension;
use manucure_db::article::get_all_active;
use manucure_db::PgPool;
use manucure_settings::SETTINGS;

pub async fn articles(
    Query(page): Query<PageQuery>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> ViewResult<HtmlTemplate<HomePageTemplate>> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(IntoResponse::into_response)?;

    let articles = get_all_active(connected_user.id, page.into(), &db)
        .await
        .map_err(|err| ErrorTemplate::response_from(err.into(), user.clone()))?;

    let articles = articles
        .try_into()
        .map_err(|err| ErrorTemplate::response_from(err, user))?;

    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Saves",
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}
