use sqlx::Error;
use thiserror::Error;

pub type DbResult<T> = Result<T, DbError>;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Internal error: {0}")]
    Internal(sqlx::Error),
    #[error("Migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("Not found: {0}")]
    NotFound(sqlx::Error),
    #[error("Unauthorized")]
    Unauthorized,
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            Error::RowNotFound => DbError::NotFound(err),
            other => DbError::Internal(other),
        }
    }
}
