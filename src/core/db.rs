use sqlx::{
    Pool,
    postgres::{PgPoolOptions, Postgres}
};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(url: &str) -> DbPool {
    let pool = PgPoolOptions::new()
    .max_connections(4)
    .connect(url)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL NOT NULL PRIMARY KEY,
            name VARCHAR(20),
            password_hash VARCHAR(100),
            email VARCHAR(255)
        )
        "
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS posts (
            id SERIAL NOT NULL PRIMARY KEY,
            title VARCHAR(20),
            content TEXT,
            date VARCHAR(50)
        )
        "
    )
    .execute(&pool)
    .await
    .unwrap();



    pool
}