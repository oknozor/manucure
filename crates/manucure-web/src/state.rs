use async_session::MemoryStore;
use axum::extract::FromRef;
use manucure_index::MeiliClient;
use oauth2::basic::BasicClient;

#[derive(Clone)]
pub struct AppState {
    pub store: MemoryStore,
    pub oauth_client: BasicClient,
    pub meili_client: MeiliClient,
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

impl FromRef<AppState> for MeiliClient {
    fn from_ref(state: &AppState) -> Self {
        state.meili_client.clone()
    }
}
