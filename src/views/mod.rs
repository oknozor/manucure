use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use http::StatusCode;
use serde::Deserialize;

pub mod article;
pub mod filters;
pub mod home;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct PageQuery {
    pub page: i64,
}

impl Default for PageQuery {
    fn default() -> Self {
        PageQuery { page: 1 }
    }
}
