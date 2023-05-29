use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Failed to hash password: {0}")]
    HashError(argon2::password_hash::Error),
}
