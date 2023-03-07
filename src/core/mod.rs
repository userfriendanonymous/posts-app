mod posts;
mod auth;
mod db;
mod users;

pub use auth::{Auth, Tokens, Info as AuthInfo};
pub use posts::Post;

pub use db::DbPool;
pub use db::create_pool as create_db_pool;

const NAME_CHARS: &str = "qazwsxedcrfvtgbyhnujmikolpQAZWSXEDCRFVTGBYHNUJMIKOLP1234567890_";

pub struct Session {
    auth_info: Auth,
    db_pool: DbPool
}

impl Session {
    pub fn new(tokens: Tokens, db_pool: DbPool) -> Self {
        Self {
            auth_info: auth::verify_tokens(&tokens),
            db_pool
        }
    }

    pub async fn create_post(&self, title: String, content: String) -> Result<i32, String> {
        let Auth::Valid {info: _} = self.auth_info else {
            return Err("Unauthorized!".to_string());
        };
        posts::create(self.db_pool.clone(), title, content).await
    }

    pub async fn get_post(&self, id: i32) -> Result<Post, String> {
        posts::get_one(self.db_pool.clone(), id).await
    }

    pub async fn get_posts(&self, limit: u32, offset: u32) -> Result<Vec<Post>, String> {
        posts::get_many(self.db_pool.clone(), limit, offset).await
    }

    pub async fn register(&mut self, name: String, password: String, email: String) -> Result<Tokens, String> {
        if name.len() > 20 || name.len() < 3 {
            return Err("Username too short or big".to_string());
        }

        for character in name.chars() {
            if !NAME_CHARS.contains(character){
                return Err("Invalid username characters".to_string());
            }
        }

        if password.len() > 35 || password.len() < 7 {
            return Err("Password too short (or big)".to_string())
        }

        let tokens = auth::get_tokens(AuthInfo {
            name: name.clone()
        })?;

        self.auth_info = auth::verify_tokens(&tokens);

        let password_hash = auth::hash_password(password)?;
        users::create(self.db_pool.clone(), name, password_hash, email).await?;

        Ok(tokens)
    }

    pub async fn login(&mut self, name: String, password: String) -> Result<Tokens, String> {
        let user = users::get(self.db_pool.clone(), name.clone()).await?;

        if !auth::compare_password(password, user.password_hash){
            return Err("Invalid credentials".to_string());
        }

        let tokens = auth::get_tokens(AuthInfo {
            name
        })?;

        self.auth_info = auth::verify_tokens(&tokens);

        Ok(tokens)
    }
}