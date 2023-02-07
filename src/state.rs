use async_session::MemoryStore;
use axum::extract::FromRef;
use oauth2::basic::BasicClient;

#[derive(Clone)]
pub struct AppState {
    pub store: MemoryStore,
    pub oauth_client: BasicClient,
    pub meili_client: meilisearch_sdk::client::Client,
}

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

impl FromRef<AppState> for meilisearch_sdk::client::Client {
    fn from_ref(state: &AppState) -> Self {
        state.meili_client.clone()
    }
}
