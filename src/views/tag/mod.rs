use crate::state::AppState;
use axum::routing::{delete, post};
use axum::Router;

mod create;
mod delete;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create::save_many))
        .route("/:id", delete(delete::delete))
}
