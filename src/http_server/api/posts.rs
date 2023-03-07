use actix_web::{Scope, web::{self, Query}, get, Responder, HttpResponse, post, web::Path};
use serde::Deserialize;
use serde_json::json;

use super::super::AppStateData;

#[derive(Deserialize)]
pub struct CreatePost {
    title: String,
    content: String,
}

pub fn service() -> Scope {
    web::scope("/posts")
    .service(create)
    .service(get_one)
    .service(get_many)
}

#[post("/create")]
pub async fn create(app_state: AppStateData, body: web::Json<CreatePost>) -> impl Responder {
    let Some(ref session) = *app_state.session.read().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "user session doesn't exist"
        }));
    };

    match session.create_post(body.title.clone(), body.content.clone()).await {
        Ok(result) => HttpResponse::Created().json(json!({
            "id": result
        })),

        Err(error) => HttpResponse::InternalServerError().json(json!({
            "message": error
        }))
    }
}

#[get("/{id}")]
pub async fn get_one(app_state: AppStateData, path: Path<i32>) -> impl Responder {
    let Some(ref session) = *app_state.session.read().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "user session doesn't exist"
        }));
    };

    match session.get_post(*path).await {
        Ok(post) => {
            HttpResponse::Ok().json(post)
        },

        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "message": error
            }));
        }
    }
}

#[derive(Deserialize)]
pub struct GetManyQuery {
    limit: u32,
    offset: u32,
}

#[get("/")]
pub async fn get_many(app_state: AppStateData, query: Query<GetManyQuery>) -> impl Responder {
    let Some(ref session) = *app_state.session.read().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "user session doesn't exist"
        }));
    };

    match session.get_posts(query.limit, query.offset).await {
        Ok(posts) => {
            HttpResponse::Ok().json(posts)
        },

        Err(error) => {
            return HttpResponse::InternalServerError().json(json!({
                "message": error
            }));
        }
    }
}