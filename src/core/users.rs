use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::db::DbPool;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct User {
    pub name: String,
    pub password_hash: String,
    pub email: String
}

pub async fn create(db_pool: DbPool, name: String, password_hash: String, email: String) -> Result<User, String> {
    match sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(name)
    .bind(password_hash)
    .bind(email)
    .fetch_one(&db_pool).await {
        Ok(user) => Ok(user),
        Err(error) => Err(error.to_string())
    }
}

pub async fn get(db_pool: DbPool, name: String) -> Result<User, String> {
    match sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE name = $1"
    )
    .bind(name)
    .fetch_one(&db_pool).await {
        Ok(user) => Ok(user),
        Err(error) => Err(error.to_string())
    }
}