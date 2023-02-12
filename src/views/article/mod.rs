use crate::state::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

mod archive;
mod create;
mod delete;
mod get;
mod star;
mod tag;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(tag::articles_with_tags))
        .route("/:id", get(get::article))
        .route("/save", get(create::save))
        .route("/:id/delete", get(delete::delete_article))
        .route("/:id/star", post(star::star_article))
        .route("/:id/unstar", post(star::unstar_article))
        .route("/:id/tags/:tag_id", post(tag::add_tag))
        .route("/:id/tags/:tag_id", delete(tag::delete_tag))
        .route("/:id/archive", post(archive::archive_article))
        .route("/:id/unarchive", post(archive::unarchive_article))
}
