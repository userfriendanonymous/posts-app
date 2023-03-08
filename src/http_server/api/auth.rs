use actix_web::{Scope, web, Responder, post, HttpResponse, cookie::Cookie};
use serde::Deserialize;
use serde_json::json;

use crate::http_server::AppStateData;

pub fn service() -> Scope {
    web::scope("/auth")
    .service(login)
    .service(register)
}

#[derive(Deserialize)]
pub struct LoginBody {
    name: String,
    password: String,
}

#[post("/login")]
pub async fn login(app_state: AppStateData, body: web::Json<LoginBody>) -> impl Responder {
    let Some(ref mut session) = *app_state.session.write().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "user session doesn't exist"
        }));
    };

    match session.login(body.name.clone(), body.password.clone()).await {
        Ok(tokens) => {
            HttpResponse::Ok()
            .cookie(
                Cookie::build("access-token", tokens.access)
                .http_only(true)
                .secure(false)
                .finish()
            )
            .cookie(
                Cookie::build("key-token", tokens.key)
                .secure(false)
                .finish()
            )
            .json(json!({
                "message": "success!"
            }))
        },

        Err(error) => {
            HttpResponse::InternalServerError().json(json!({
                "message": error
            }))
        }
    }
}

#[derive(Deserialize)]
pub struct RegisterBody {
    name: String,
    email: String,
    password: String
}


#[post("/register")]
pub async fn register(app_state: AppStateData, body: web::Json<RegisterBody>) -> impl Responder {
    let Some(ref mut session) = *app_state.session.write().unwrap() else {
        return HttpResponse::InternalServerError().json(json!({
            "message": "user session doesn't exist"
        }));
    };

    match session.register(body.name.clone(), body.password.clone(), body.email.clone()).await {
        Ok(tokens) => {
            HttpResponse::Ok()
            .cookie(
                Cookie::build("access-token", tokens.access)
                .http_only(true)
                .secure(false)
                .finish()
            )
            .cookie(
                Cookie::build("key-token", tokens.key)
                .secure(false)
                .finish()
            )
            .json(json!({
                "message": "success!"
            }))
        },

        Err(error) => {
            HttpResponse::InternalServerError().json(json!({
                "message": error
            }))
        }
    }
}