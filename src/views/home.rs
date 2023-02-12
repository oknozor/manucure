use super::filters;
use crate::auth::Oauth2User;
use crate::db::article::{
    get_all_active, get_all_archived, get_all_starred, ArticlesWithPagination,
};
use crate::db::user::get_connected_user;
use crate::db::Page;
use crate::errors::{ErrorTemplate, ViewResult};
use crate::settings::SETTINGS;
use crate::views::{HtmlTemplate, PageQuery};
use askama::Template;
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Extension;
use sqlx::PgPool;

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    articles: ArticlesWithPagination,
    title: &'static str,
    user_index: String,
    meili_url: &'static str,
    meili_secret: &'static str,
}

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
        .map_err(|err| ErrorTemplate::to_response(err, user))?;

    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Saves",
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}

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
        .map_err(|err| ErrorTemplate::to_response(err, user))?;

    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Archive",
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}

pub async fn starred(
    Query(page): Query<PageQuery>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> ViewResult<HtmlTemplate<HomePageTemplate>> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(IntoResponse::into_response)?;

    let articles = get_all_starred(connected_user.id, page.into(), &db)
        .await
        .map_err(|err| ErrorTemplate::to_response(err, user))?;

    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Favorites",
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}

pub async fn tags(
    Query(page): Query<PageQuery>,
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> ViewResult<HtmlTemplate<HomePageTemplate>> {
    let connected_user = get_connected_user(user.as_ref(), &db)
        .await
        .map_err(IntoResponse::into_response)?;

    let articles = get_all_active(connected_user.id, Page::from(page), &db)
        .await
        .map_err(|err| ErrorTemplate::to_response(err, user))?;

    Ok(HtmlTemplate(HomePageTemplate {
        articles,
        title: "Todo",
        user_index: format!("articles-{}", connected_user.id),
        meili_url: &SETTINGS.search_engine.url,
        meili_secret: &SETTINGS.search_engine.api_key,
    }))
}
