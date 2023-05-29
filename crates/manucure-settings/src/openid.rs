use crate::SETTINGS;
use serde::Deserialize;

use tokio::sync::OnceCell;

pub static OAUTH_ENDPOINTS: OnceCell<OauthEndpoints> = OnceCell::const_new();

#[derive(Debug, Clone, Deserialize)]
pub struct OauthEndpoints {
    pub userinfo_endpoint: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
}

async fn discover(url: &str) -> reqwest::Result<OauthEndpoints> {
    reqwest::get(url).await?.json().await
}

pub async fn fetch_open_id_endpoints() -> OauthEndpoints {
    let url = SETTINGS
        .openid
        .as_ref()
        .expect("openid discover url")
        .openid_discovery_url
        .as_ref();
    discover(url)
        .await
        .expect("Failed to discover openid endpoint via HTTP")
}
