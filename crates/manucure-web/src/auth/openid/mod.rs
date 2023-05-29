use async_session::{MemoryStore, Session, SessionStore};

use crate::auth::AuthRedirect;
use crate::auth::COOKIE_NAME;
use crate::state::AppState;

use axum::{
    async_trait,
    extract::{
        rejection::TypedHeaderRejectionReason, FromRef, FromRequestParts, Query, State, TypedHeader,
    },
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect},
    RequestPartsExt,
};
use http::{header, request::Parts};

use manucure_db::user::AsUser;

use manucure_settings::SETTINGS;
use oauth2::reqwest::async_http_client;
use oauth2::{
    basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Oauth2User {
    pub sub: String,
    pub email_verified: bool,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

pub async fn oauth_client() -> BasicClient {
    let provider = SETTINGS.openid.as_ref().expect("openid provider");
    let client_id = provider.client_id.to_string();
    let client_secret = provider.client_secret.to_string();
    let redirect_url = format!(
        "{}://{}/auth/authorized",
        SETTINGS.protocol(),
        SETTINGS.domain
    );
    let auth_url = SETTINGS.auth_url().await;
    let token_url = SETTINGS.token_url().await;

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url.to_string()).unwrap(),
        Some(TokenUrl::new(token_url.to_string()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

pub async fn openid_auth(State(state): State<AppState>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = state
        .oauth_client
        .expect("openid configuration")
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let token = state
        .oauth_client
        .expect("openid configuration")
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let client = reqwest::Client::new();
    let user_data: Oauth2User = client
        .get(SETTINGS.user_info_url().await)
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<Oauth2User>()
        .await
        .unwrap();

    let mut session = Session::new();
    session.insert("user", &user_data).unwrap();
    let cookie = state.store.store_session(session).await.unwrap().unwrap();
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; Path=/");
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (headers, Redirect::to("/"))
}

#[async_trait]
impl<S> FromRequestParts<S> for Oauth2User
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthRedirect;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);
        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {e}"),
                },
                _ => panic!("unexpected error getting cookies: {e}"),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;
        let session = store
            .load_session(session_cookie.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<Oauth2User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}
