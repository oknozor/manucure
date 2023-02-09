pub mod get {
    use crate::auth::Oauth2User;
    use crate::db;
    use crate::db::article::{fetch_and_store, Article};
    use crate::db::user::get_connected_user;
    use crate::errors::{ErrorTemplate, ViewResult};
    use crate::settings::SETTINGS;
    use crate::state::AppState;
    use crate::views::HtmlTemplate;
    use askama::Template;
    use axum::extract::{Path, State};
    use axum::response::{IntoResponse, Redirect};
    use axum::{Extension, Form};
    use serde::Deserialize;
    use sqlx::PgPool;

    #[derive(Debug, Deserialize)]
    pub struct ArticleSaveForm {
        pub url: String,
    }
    pub async fn save(
        State(state): State<AppState>,
        user: Option<Oauth2User>,
        Extension(db): Extension<PgPool>,
        Form(input): Form<ArticleSaveForm>,
    ) -> ViewResult<Redirect> {
        let connected_user = get_connected_user(user.as_ref(), &db)
            .await
            .map_err(IntoResponse::into_response)?;
        fetch_and_store(connected_user.id, &input.url, state.meili_client, &db)
            .await
            .map_err(|err| ErrorTemplate::to_response(err, user))?;
        Ok(Redirect::to("/"))
    }

    #[derive(Template, Debug)]
    #[template(path = "article.html")]
    pub struct ArticleTemplate {
        article: Article,
        user_index: String,
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
            .map_err(|err| ErrorTemplate::to_response(err, user))?;

        Ok(HtmlTemplate(ArticleTemplate {
            article,
            user_index: format!("articles-{}", connected_user.id),
            meili_url: &SETTINGS.search_engine.url,
            meili_secret: &SETTINGS.search_engine.api_key,
        }))
    }

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
            .map_err(|err| ErrorTemplate::to_response(err, user))?;

        Ok(Redirect::to("/"))
    }
}

pub mod post {
    use crate::auth::Oauth2User;
    use crate::db;
    use crate::db::user::get_connected_user;
    use crate::errors::AppResult;
    use axum::extract::Path;
    use axum::Extension;
    use sqlx::PgPool;

    pub async fn star_article(
        user: Option<Oauth2User>,
        Extension(db): Extension<PgPool>,
        Path(id): Path<i64>,
    ) -> AppResult<()> {
        let user = get_connected_user(user.as_ref(), &db).await?;
        db::article::star(user.id, id, &db).await?;

        Ok(())
    }

    pub async fn unstar_article(
        user: Option<Oauth2User>,
        Extension(db): Extension<PgPool>,
        Path(id): Path<i64>,
    ) -> AppResult<()> {
        let user = get_connected_user(user.as_ref(), &db).await?;
        db::article::unstar(user.id, id, &db).await?;

        Ok(())
    }

    pub async fn archive_article(
        user: Option<Oauth2User>,
        Extension(db): Extension<PgPool>,
        Path(id): Path<i64>,
    ) -> AppResult<()> {
        let user = get_connected_user(user.as_ref(), &db).await?;
        db::article::archive(user.id, id, &db).await?;

        Ok(())
    }

    pub async fn unarchive_article(
        user: Option<Oauth2User>,
        Extension(db): Extension<PgPool>,
        Path(id): Path<i64>,
    ) -> AppResult<()> {
        let user = get_connected_user(user.as_ref(), &db).await?;
        db::article::unarchive(user.id, id, &db).await?;

        Ok(())
    }
}
