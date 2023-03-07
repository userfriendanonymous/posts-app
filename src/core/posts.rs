use super::db::DbPool;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Post {
    pub title: String,
    pub content: String,
    pub date: String,
}

pub async fn create(db_pool: DbPool, title: String, content: String) -> Result<i32, String> {
    match sqlx::query_as::<_, (i32,)>(
        "INSERT INTO posts (title, content, date) VALUES ($1, $2, $3) RETURNING id"
    )
    .bind(title.clone())
    .bind(content.clone())
    .bind(chrono::offset::Local::now().to_string())
    .fetch_one(&db_pool).await {
        Ok(id) => Ok(id.0),
        Err(error) => Err(error.to_string())
    }
}

pub async fn get_one(db_pool: DbPool, id: i32) -> Result<Post, String> {
    match sqlx::query_as::<_, Post>(
        "SELECT (title, content, date) FROM posts WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&db_pool).await {
        Ok(post) => Ok(post),
        Err(error) => Err(error.to_string())
    }
}

pub async fn get_many(db_pool: DbPool, limit: u32, offset: u32) -> Result<Vec<Post>, String> {
    match sqlx::query_as::<_, Post>(
        "SELECT * FROM posts LIMIT $1 OFFSET $2"
    )
    .bind(limit as i32)
    .bind(offset as i32)
    .fetch_all(&db_pool).await {
        Ok(posts) => Ok(posts),
        Err(error) => Err(error.to_string())
    }
}

pub async fn update(db_pool: DbPool, id: i32, title: Option<String>, content: Option<String>) -> Result<Post, String> {
    match sqlx::query_as::<_, Post>(
        "UPDATE posts SET title = $1, content = $2 WHERE id = $3 RETURNING *"
    )
    .bind(title.clone())
    .bind(content.clone())
    .bind(id)
    .fetch_one(&db_pool).await {
        Ok(post) => Ok(post),
        Err(error) => Err(error.to_string())
    }
}

pub async fn delete(db_pool: DbPool, id: i32) -> Result<Post, String> {
    match sqlx::query_as::<_, Post>(
        "DELETE FROM posts WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_one(&db_pool).await {
        Ok(post) => Ok(post),
        Err(error) => Err(error.to_string())
    }
}