use crate::error::DbResult;
use sqlx::PgPool;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

pub trait AsUser {
    fn username(&self) -> &str;
    fn email(&self) -> &str;
}

pub async fn all(db: &PgPool) -> DbResult<Vec<User>> {
    // language=PostgreSQL
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(db)
        .await?;

    Ok(users)
}

pub async fn get_connected_user(user: impl AsUser, db: &PgPool) -> DbResult<User> {
    let db_user = sqlx::query_as!(User, "SELECT * FROM users where email = $1", user.email())
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
                user.username(),
                user.email()
            )
            .fetch_one(db)
            .await?;
            Ok(user)
        }
        Err(err) => Err(err),
    }?;

    Ok(user)
}
