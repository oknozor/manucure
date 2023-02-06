use crate::auth::Oauth2User;
use crate::db::article::{get_all_active, Article};
use crate::db::user::get_connected_user;
use crate::errors::AppResult;
use crate::views::HtmlTemplate;
use askama::Template;
use axum::Extension;
use sqlx::PgPool;

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct HomePageTemplate {
    pub articles: Vec<Article>,
}

pub async fn index(
    user: Option<Oauth2User>,
    Extension(db): Extension<PgPool>,
) -> AppResult<HtmlTemplate<HomePageTemplate>> {
    let user = get_connected_user(user, &db).await?;
    let articles = get_all_active(user.id, &db).await?;
    Ok(HtmlTemplate(HomePageTemplate { articles }))
}
