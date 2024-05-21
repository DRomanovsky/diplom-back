use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};
use actix_cors::Cors;

mod models;
mod repository;
mod api;

//Auth
use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}


#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = repository::database::Database::new();
    let app_data = web::Data::new(db);

    
    HttpServer::new(move|| {
        App::new()
        .app_data(app_data.clone())
        .configure(api::api::config)
        .service(healthcheck)
        .default_service(web::route().to(not_found))
        .wrap(actix_web::middleware::Logger::default())
        .wrap(Cors::permissive().supports_credentials())
        
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}