pub mod error;
pub mod openid;
pub mod simple;

use async_session::SessionStore;

use crate::auth::openid::{login_authorized, openid_auth, Oauth2User};
use crate::auth::simple::login::login;
use crate::auth::simple::signin::signin;
use axum::routing::get;
use axum::{
    extract::{State, TypedHeader},
    response::{IntoResponse, Redirect, Response},
    Router,
};

use manucure_db::error::DbError;
use manucure_db::user::{AsUser, User};
use manucure_db::PgPool;
use manucure_settings::SETTINGS;

use crate::errors::{AppError, AppResult};
use crate::state::AppState;

pub static COOKIE_NAME: &str = "MANUCURE_SESSION";

pub async fn get_connected_user(user: Option<&Oauth2User>, db: &PgPool) -> AppResult<User> {
    let Some(user) = user else {
        return Err(AppError::Unauthorized);
    };

    manucure_db::user::get_connected_user(user, db)
        .await
        .map_err(|err| match err {
            DbError::Unauthorized => AppError::Unauthorized,
            err => AppError::Internal(err.into()),
        })
}

pub fn router() -> Router<AppState> {
    if SETTINGS.openid.is_some() {
        Router::new()
            .route("/manucure/", get(openid_auth))
            .route("/manucure", get(openid_auth))
            .route("/authorized/", get(login_authorized))
            .route("/authorized", get(login_authorized))
            .route("/logout", get(logout))
            .route("/logout/", get(logout))
    } else {
        Router::new()
            .route("/manucure/", get(login))
            .route("/manucure", get(login))
            .route("/signin", get(signin))
            .route("/signin/", get(signin))
            .route("/logout/", get(logout))
            .route("/register/", get(logout))
            .route("/register", get(logout))
    }
}

pub async fn logout(
    State(state): State<AppState>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> impl IntoResponse {
    let cookie = cookies.get(COOKIE_NAME).unwrap();
    let session = match state.store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        // No session active, just redirect
        None => return Redirect::to("/"),
    };

    state.store.destroy_session(session).await.unwrap();

    Redirect::to("/")
}

impl AsUser for &Oauth2User {
    fn username(&self) -> &str {
        &self.preferred_username
    }
    fn email(&self) -> &str {
        &self.email
    }
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/manucure").into_response()
    }
}
