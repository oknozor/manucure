use sqlx::PgPool;

use crate::{
    auth::Oauth2User,
    errors::{AppError, AppResult},
};

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

pub(crate) async fn get_connected_user(user: Option<&Oauth2User>, db: &PgPool) -> AppResult<User> {
    let Some(user) = user else {
        return Err(AppError::Unauthorized);
    };

    let db_user = sqlx::query_as!(User, "SELECT * FROM users where email = $1", user.email)
        .fetch_one(db)
        .await;

    let user = match db_user {
        Ok(user) => Ok(user),
        Err(err) if matches!(err, sqlx::Error::RowNotFound) => {
            let user = sqlx::query_as!(
                User,
                r#"INSERT INTO users(username, email) 
                VALUES($1, $2) 
                RETURNING *"#,
                user.preferred_username,
                user.email
            )
            .fetch_one(db)
            .await?;
            Ok(user)
        }
        Err(err) => Err(err),
    }?;

    Ok(user)
}
