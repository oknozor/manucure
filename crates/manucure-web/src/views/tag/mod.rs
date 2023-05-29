use axum::routing::{delete, post};
use axum::Router;

use crate::state::AppState;

mod create;
mod delete;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create::save_many))
        .route("/:id", delete(delete::delete))
}
