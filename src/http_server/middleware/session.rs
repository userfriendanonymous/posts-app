use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use crate::{http_server::AppStateData, core::{Session, Tokens}};

pub struct Factory {
    app_state: AppStateData
}

impl Factory {
    pub fn new(app_state: AppStateData) -> Self {
        Self {
            app_state
        }
    }
}

impl<NextService, ResponseBody> Transform<NextService, ServiceRequest> for Factory
where
    NextService: Service<
        ServiceRequest,
        Response = ServiceResponse<ResponseBody>,
        Error = Error
    >,
    NextService::Future: 'static,
    ResponseBody: 'static,
{
    type Response = ServiceResponse<ResponseBody>;
    type Error = Error;
    type InitError = ();
    type Transform = Middleware<NextService>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: NextService) -> Self::Future {
        ready(Ok(Middleware {
            service,
            app_state: self.app_state.clone()
        }))
    }
}

pub struct Middleware<Service> {
    service: Service,
    app_state: AppStateData
}

fn get_cookie(request: &ServiceRequest, name: &str) -> Option<String> {
    request.cookie(name).map(|cookie| {
        let cookie = cookie.to_string();
        let mut iter = cookie.split("=");
        iter.next();
        iter.next().map(|value| String::from(value)).unwrap_or_default()
    })
}

impl<S, ResponseBody> Service<ServiceRequest> for Middleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<ResponseBody>, Error = Error>,
    S::Future: 'static,
    ResponseBody: 'static,
{
    type Response = ServiceResponse<ResponseBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", request.path());

        let tokens = Tokens {
            access: get_cookie(&request, "access-token").unwrap_or_default(),
            key: get_cookie(&request, "key-token").unwrap_or_default()
        };

        println!("tokens: {tokens:?}");

        let mut session = self.app_state.session.write().unwrap();
        *session = Some(
            Session::new(tokens, self.app_state.db_pool.clone())
        );

        let next_service_future = self.service.call(request);

        Box::pin(async move {
            let response = next_service_future.await?;

            println!("Hi from response");
            Ok(response)
        })
    }
}