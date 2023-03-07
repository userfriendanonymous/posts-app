mod api;
mod middleware;

use std::sync::{Mutex, RwLock};

use crate::core::Session;
use actix_web::{HttpServer, App, web::Data, dev::{Service, ServiceResponse, ServiceRequest}, HttpRequest, Error};

use super::core::DbPool;

pub struct AppState {
    pub db_pool: DbPool,
    pub session: RwLock<Option<Session>>
}

pub type AppStateData = Data<AppState>;

pub async fn launch(db_pool: DbPool){
    let app_state = Data::new(AppState {
        db_pool,
        session: RwLock::new(None)
    });

    HttpServer::new(move || {
        App::new()
        .wrap(middleware::session::Factory::new(
            app_state.clone()
        ))
        .app_data(app_state.clone())
        .service(api::service())
    })
    .bind(("127.0.0.1", 5000)).unwrap()
    .run()
    .await.unwrap();
}