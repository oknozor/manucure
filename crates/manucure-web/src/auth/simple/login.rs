use crate::errors::ViewResult;
use crate::views::HtmlTemplate;
use askama::Template;
use axum::Extension;
use manucure_db::PgPool;

#[derive(Template, Debug)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

pub async fn login(Extension(_db): Extension<PgPool>) -> ViewResult<HtmlTemplate<LoginTemplate>> {
    Ok(HtmlTemplate(LoginTemplate {}))
}
