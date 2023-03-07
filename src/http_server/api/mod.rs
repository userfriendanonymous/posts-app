mod posts;
mod auth;
use actix_web::{Scope, web};

pub fn service() -> Scope {
    web::scope("/api")
    .service(posts::service())
    .service(auth::service())
}