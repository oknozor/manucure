use crate::errors::ViewResult;
use crate::views::HtmlTemplate;
use askama::Template;
use axum::Extension;
use manucure_db::PgPool;

#[derive(Template, Debug)]
#[template(path = "signin.html")]
pub struct SignInTemplate {}

pub async fn signin(Extension(_db): Extension<PgPool>) -> ViewResult<HtmlTemplate<SignInTemplate>> {
    Ok(HtmlTemplate(SignInTemplate {}))
}
