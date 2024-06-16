use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
use actix_cors::Cors;
mod models;
mod repository;
mod api;
use std::path::Path;
use tokio::fs;
use actix_files as fs2;

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

    if !Path::new("./upload").exists() {
        fs::create_dir("./upload").await?;
    }

    HttpServer::new(move|| {        
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .service(fs2::Files::new("/upload", "./upload").show_files_listing())
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
            
            .wrap(Cors::permissive()
                    .supports_credentials()
                    .allow_any_header()
                    .allow_any_origin()
                    .allow_any_method()
                    .max_age(3600))
        
    })
    .workers(2)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}