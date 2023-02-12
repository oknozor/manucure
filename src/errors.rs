use crate::auth::Oauth2User;
use crate::views::HtmlTemplate;
use askama::Template;
use axum::response::{IntoResponse, Redirect, Response};
use http::StatusCode;
use tracing::error;

// A Result that either returns T or an Error.
pub type AppResult<T> = Result<T, AppError>;

// A Result that either returns T or a Response.
// used to serve Html template as error and redirect
// on auth error
pub type ViewResult<T> = Result<T, Response>;

#[derive(Debug)]
pub enum AppError {
    Internal(anyhow::Error),
    Unauthorized,
    NotFound,
}

#[derive(Template, Debug)]
#[template(path = "error.html")]
pub struct ErrorTemplate {
    pub user: Option<Oauth2User>,
    pub message: String,
    pub code: i16,
}

impl ErrorTemplate {
    pub(crate) fn response_from(error: AppError, user: Option<Oauth2User>) -> Response {
        let template = match error {
            AppError::Internal(_) => ErrorTemplate {
                user,
                message: "Internal server Error".to_string(),
                code: 500,
            },
            AppError::Unauthorized => ErrorTemplate {
                user,
                message: "Please login".to_string(),
                code: 401,
            },
            AppError::NotFound => ErrorTemplate {
                user,
                message: "Resource not found".to_string(),
                code: 404,
            },
        };

        let template = HtmlTemplate(template);
        template.into_response()
    }
}

impl<T> From<T> for AppError
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        let err = t.into();
        error!("{err}");
        AppError::Internal(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Internal(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")).into_response()
            }
            AppError::Unauthorized => Redirect::to("/auth/manucure").into_response(),
            AppError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND").into_response(),
        }
    }
}
